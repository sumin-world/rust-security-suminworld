use merkle::MerkleTree;

#[test]
fn small_root() {
    let mt = MerkleTree::from_leaves(["a", "b", "c", "d"].map(|s| s.as_bytes()));
    let root = mt.root().unwrap();
    println!("root = {:x?}", root);
    assert!(root.len() == 32);
}

#[test]
fn proof_roundtrip() {
    let leaves = ["a", "b", "c", "d"].map(|s| s.as_bytes());
    let mt = MerkleTree::from_leaves(leaves);
    let root = mt.root().unwrap();

    for (i, s) in ["a","b","c","d"].iter().enumerate() {
        let proof = mt.proof(i);
        assert!(MerkleTree::verify(root, s.as_bytes(), &proof, i));
    }
}
