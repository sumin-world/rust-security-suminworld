use merkle::MerkleTree;

#[test]
fn small_root() {
    let mt = MerkleTree::from_leaves(["a", "b", "c", "d"].map(|s| s.as_bytes()));
    let root = mt.root().unwrap();
    assert_eq!(root.len(), 32);
    assert_eq!(mt.leaf_count(), 4);
}

#[test]
fn proof_roundtrip() {
    let leaves = ["a", "b", "c", "d"].map(|s| s.as_bytes());
    let mt = MerkleTree::from_leaves(leaves);
    let root = mt.root().unwrap();

    for (i, s) in ["a", "b", "c", "d"].iter().enumerate() {
        let proof = mt.proof(i);
        assert!(
            MerkleTree::verify(root, s.as_bytes(), &proof, i),
            "proof failed for leaf {i}"
        );
    }
}

#[test]
fn wrong_index_fails() {
    let mt = MerkleTree::from_leaves(["a", "b", "c", "d"].map(|s| s.as_bytes()));
    let root = mt.root().unwrap();
    let proof = mt.proof(0);
    // Correct leaf but wrong index should fail
    assert!(!MerkleTree::verify(root, b"a", &proof, 1));
}
