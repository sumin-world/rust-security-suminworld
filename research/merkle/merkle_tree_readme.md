# Merkle Tree Implementation in Rust

A fast, secure Rust implementation of Merkle Trees (Hash Trees) for data integrity verification and efficient membership proofs. Built with SHA-256 hashing and designed for blockchain, cryptographic, and distributed systems applications.

## Overview

This library provides a complete Merkle Tree implementation that allows you to:
- Generate cryptographic summaries of large datasets
- Create compact proofs of data inclusion
- Verify data integrity without storing the entire dataset
- Build tamper-evident data structures

## Features

- **Fast Tree Construction**: Build trees from data leaves in O(n) time
- **Compact Proofs**: Generate inclusion proofs of O(log n) size
- **Efficient Verification**: Verify proofs in O(log n) time
- **SHA-256 Security**: Uses industry-standard cryptographic hashing
- **Odd Leaf Handling**: Automatically handles unbalanced trees by duplicating the last leaf
- **Zero-Copy Design**: Efficient memory usage with minimal allocations

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
merkle = { path = "research/merkle" }
sha2 = "0.10"
hex = "0.4"
```

Or for workspace usage, the crate is already configured as a workspace member.

## Quick Start

```rust
use merkle::MerkleTree;
use hex::ToHex;

fn main() {
    // Create tree from data leaves
    let data = ["transaction1", "transaction2", "transaction3"]
        .map(|s| s.as_bytes());
    let tree = MerkleTree::from_leaves(data);

    // Get the root hash (32-byte SHA-256)
    let root = tree.root().expect("Non-empty tree");
    println!("Root hash: {}", root.encode_hex::<String>());

    // Generate proof for second item
    let index = 1;
    let proof = tree.proof(index);

    // Verify the proof
    let is_valid = MerkleTree::verify(
        root, 
        b"transaction2", 
        &proof, 
        index
    );
    
    assert!(is_valid);
    println!("Proof verification: {}", is_valid);
}
```

## API Reference

### `MerkleTree::from_leaves(leaves)`
Creates a new Merkle Tree from an iterator of byte slices.

**Parameters:**
- `leaves`: Iterator yielding `&[u8]` data to be hashed

**Returns:** `MerkleTree` instance

### `tree.root() -> Option<[u8; 32]>`
Returns the root hash of the tree.

**Returns:** 
- `Some(hash)` if tree is non-empty
- `None` if tree was created with no leaves

### `tree.proof(index) -> Vec<[u8; 32]>`
Generates an inclusion proof for the leaf at the given index.

**Parameters:**
- `index`: Zero-based index of the leaf (must be < leaf count)

**Returns:** Vector of 32-byte hashes representing the authentication path

### `MerkleTree::verify(root, leaf, proof, index) -> bool`
Verifies that a leaf is included in a tree with the given root.

**Parameters:**
- `root`: The tree's root hash
- `leaf`: Original leaf data (not pre-hashed)
- `proof`: Authentication path from `tree.proof()`
- `index`: Position of the leaf in the original data

**Returns:** `true` if proof is valid, `false` otherwise

## Examples

### Basic Usage with String Data

```rust
use merkle::MerkleTree;

let documents = vec!["doc1.pdf", "doc2.txt", "doc3.jpg"];
let tree = MerkleTree::from_leaves(
    documents.iter().map(|doc| doc.as_bytes())
);

// Prove doc2.txt is in the set
let proof = tree.proof(1);
let root = tree.root().unwrap();

assert!(MerkleTree::verify(root, b"doc2.txt", &proof, 1));
```

### File Integrity Verification

```rust
use merkle::MerkleTree;
use std::fs;

// Read file chunks
let files = vec!["file1.dat", "file2.dat", "file3.dat"];
let file_data: Vec<Vec<u8>> = files.iter()
    .map(|path| fs::read(path).unwrap())
    .collect();

let tree = MerkleTree::from_leaves(
    file_data.iter().map(|data| data.as_slice())
);

// Store root hash for later verification
let integrity_hash = tree.root().unwrap();

// Later: verify a specific file hasn't been tampered with
let file_index = 1;
let proof = tree.proof(file_index);
let current_file = fs::read("file2.dat").unwrap();

let is_intact = MerkleTree::verify(
    integrity_hash,
    &current_file,
    &proof,
    file_index
);
```

## Testing

Run tests for this crate:

```bash
cargo test -p merkle
```

The test suite covers:
- Root hash generation and consistency
- Proof generation for all indices
- Verification round-trips
- Edge cases (empty trees, single leaves, odd counts)
- Security properties (tamper detection)

## Technical Details

### Hash Function
- **Algorithm**: SHA-256
- **Internal nodes**: `H(left_child || right_child)`
- **Leaf nodes**: `H(original_data)`

### Tree Structure
- **Binary tree**: Each internal node has exactly 2 children
- **Balanced**: All leaves are at the same depth (with duplication for odd counts)
- **Bottom-up construction**: Leaves hashed first, then combined upward

### Odd Leaf Handling
When the number of leaves is odd, the last leaf hash is duplicated to maintain the binary tree structure:

```
Original: [A, B, C]
Tree:     [A, B, C, C]  // C duplicated
```

### Performance
- **Time Complexity**:
  - Tree construction: O(n)
  - Proof generation: O(log n)
  - Proof verification: O(log n)
- **Space Complexity**: O(n) for tree storage
- **Proof Size**: log₂(n) hashes (32 bytes each)

## Use Cases

### Blockchain & Cryptocurrency
- Transaction verification without downloading full blocks
- Light client implementations
- Efficient blockchain synchronization

### Distributed Systems
- Content-addressed storage verification
- P2P file sharing integrity checks
- Database replication validation

### Software Distribution
- Package integrity verification
- Software update validation
- Distributed build verification

### Data Archival
- Long-term data integrity monitoring
- Tamper-evident audit logs
- Backup verification systems

## Security Considerations

### Strengths
- **Collision Resistance**: SHA-256 provides strong collision resistance
- **Tamper Detection**: Any data modification changes the root hash
- **Selective Disclosure**: Prove inclusion without revealing other data

### Limitations
- **Known Root Required**: Verifier must have authentic root hash
- **No Proof of Exclusion**: Cannot prove data is NOT in the tree
- **Quantum Vulnerability**: SHA-256 may be vulnerable to future quantum computers

## Contributing

This implementation prioritizes correctness, performance, and security. Contributions should include:
- Comprehensive tests
- Benchmarks for performance-critical changes
- Documentation updates
- Security consideration analysis

## Future Enhancements

- **Alternative Hash Functions**: Blake3, Keccak support
- **Serialization**: Serde support for proof storage/transmission
- **CLI Tools**: Command-line utilities for file/directory hashing
- **Streaming Support**: Process large datasets without full memory loading
- **Parallel Construction**: Multi-threaded tree building for large datasets

## License

MIT License - See LICENSE file for details

---

**⚠️ Security Notice**: This library is provided for educational and research purposes. Production use should undergo thorough security review and testing appropriate for your specific requirements.