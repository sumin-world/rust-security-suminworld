use sha2::{Digest, Sha256};

pub type Hash32 = [u8; 32];

#[inline]
fn h_leaf(data: &[u8]) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

#[inline]
fn h_internal(left: &Hash32, right: &Hash32) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

pub struct MerkleTree {
    levels: Vec<Vec<Hash32>>, // levels[0] = leaves, levels[last] = root level(길이 1)
}

impl MerkleTree {
    pub fn from_leaves<I, B>(leaves: I) -> Self
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        let level0: Vec<Hash32> = leaves
            .into_iter()
            .map(|b| h_leaf(b.as_ref()))
            .collect();

        if level0.is_empty() {
            return Self { levels: vec![vec![]] };
        }

        let mut levels = vec![level0];

        // 위로 쌓기
        loop {
            let prev = levels.last().unwrap();
            if prev.len() == 1 {
                break;
            }
            let mut next = Vec::with_capacity((prev.len() + 1) / 2);
            let mut i = 0;
            while i < prev.len() {
                let left = &prev[i];
                let right = if i + 1 < prev.len() { &prev[i + 1] } else { left }; // 홀수 처리
                next.push(h_internal(left, right));
                i += 2;
            }
            levels.push(next);
        }

        Self { levels }
    }

    pub fn root(&self) -> Option<Hash32> {
        self.levels.last().and_then(|lvl| lvl.first()).copied()
    }

    /// 인증 경로: 해당 leaf의 형제들 해시를 위로 모음
    pub fn proof(&self, mut index: usize) -> Vec<Hash32> {
        let mut proof = Vec::new();
        for lvl in &self.levels {
            if lvl.len() <= 1 { break; }
            let sib = if index % 2 == 0 {
                // 오른쪽 형제 없으면 자기 자신(홀수 보정 규칙과 일치)
                if index + 1 < lvl.len() { lvl[index + 1] } else { lvl[index] }
            } else {
                lvl[index - 1]
            };
            proof.push(sib);
            index /= 2;
        }
        proof
    }

    pub fn verify(root: Hash32, leaf_data: &[u8], proof: &[Hash32], mut index: usize) -> bool {
        let mut acc = h_leaf(leaf_data);
        for sib in proof {
            acc = if index % 2 == 0 {
                h_internal(&acc, sib)
            } else {
                h_internal(sib, &acc)
            };
            index /= 2;
        }
        acc == root
    }
}
