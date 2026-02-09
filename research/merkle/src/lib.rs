use sha2::{Digest, Sha256};

/// A 32-byte SHA-256 digest.
pub type Hash32 = [u8; 32];

/// Hash a leaf node (domain-separated from internal nodes by design).
#[inline]
fn h_leaf(data: &[u8]) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(b"\x00"); // leaf domain separator
    hasher.update(data);
    hasher.finalize().into()
}

/// Hash two child nodes into a parent.
#[inline]
fn h_internal(left: &Hash32, right: &Hash32) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(b"\x01"); // internal domain separator
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

/// A complete Merkle hash tree built from leaf data.
///
/// `levels[0]` contains leaf hashes; `levels[last]` contains the single root hash.
pub struct MerkleTree {
    levels: Vec<Vec<Hash32>>,
}

impl MerkleTree {
    /// Build a tree from an iterator of leaf byte slices.
    pub fn from_leaves<I, B>(leaves: I) -> Self
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        let level0: Vec<Hash32> = leaves.into_iter().map(|b| h_leaf(b.as_ref())).collect();

        if level0.is_empty() {
            return Self {
                levels: vec![vec![]],
            };
        }

        let mut levels = vec![level0];

        loop {
            let prev = levels.last().unwrap();
            if prev.len() == 1 {
                break;
            }
            let mut next = Vec::with_capacity(prev.len().div_ceil(2));
            for chunk in prev.chunks(2) {
                let left = &chunk[0];
                let right = chunk.get(1).unwrap_or(left);
                next.push(h_internal(left, right));
            }
            levels.push(next);
        }

        Self { levels }
    }

    /// Return the root hash, or `None` for an empty tree.
    pub fn root(&self) -> Option<Hash32> {
        self.levels.last().and_then(|lvl| lvl.first()).copied()
    }

    /// Return the number of leaves.
    pub fn leaf_count(&self) -> usize {
        self.levels.first().map_or(0, Vec::len)
    }

    /// Generate an authentication (Merkle) proof for the leaf at `index`.
    pub fn proof(&self, mut index: usize) -> Vec<Hash32> {
        let mut proof = Vec::new();
        for lvl in &self.levels {
            if lvl.len() <= 1 {
                break;
            }
            let sibling = if index.is_multiple_of(2) {
                lvl.get(index + 1).unwrap_or(&lvl[index])
            } else {
                &lvl[index - 1]
            };
            proof.push(*sibling);
            index /= 2;
        }
        proof
    }

    /// Verify that `leaf_data` at `index` is included under `root`.
    pub fn verify(root: Hash32, leaf_data: &[u8], proof: &[Hash32], mut index: usize) -> bool {
        let mut acc = h_leaf(leaf_data);
        for sib in proof {
            acc = if index.is_multiple_of(2) {
                h_internal(&acc, sib)
            } else {
                h_internal(sib, &acc)
            };
            index /= 2;
        }
        acc == root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree() {
        let mt = MerkleTree::from_leaves(Vec::<&[u8]>::new());
        assert!(mt.root().is_none());
        assert_eq!(mt.leaf_count(), 0);
    }

    #[test]
    fn single_leaf() {
        let mt = MerkleTree::from_leaves([b"only"]);
        assert!(mt.root().is_some());
        assert_eq!(mt.leaf_count(), 1);
        let proof = mt.proof(0);
        assert!(MerkleTree::verify(mt.root().unwrap(), b"only", &proof, 0));
    }

    #[test]
    fn odd_number_of_leaves() {
        let mt = MerkleTree::from_leaves(["a", "b", "c"].map(|s| s.as_bytes()));
        let root = mt.root().unwrap();
        for (i, s) in ["a", "b", "c"].iter().enumerate() {
            let proof = mt.proof(i);
            assert!(MerkleTree::verify(root, s.as_bytes(), &proof, i));
        }
    }

    #[test]
    fn tampered_leaf_fails() {
        let mt = MerkleTree::from_leaves(["a", "b", "c", "d"].map(|s| s.as_bytes()));
        let root = mt.root().unwrap();
        let proof = mt.proof(0);
        assert!(!MerkleTree::verify(root, b"TAMPERED", &proof, 0));
    }
}
