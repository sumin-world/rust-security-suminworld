// research/crypto/src/lib.rs
pub mod crypto;

pub use crypto::{
    bytes_to_hex, calculate_entropy, hex_to_bytes, run_crypto_demo, simple_hash, CaesarCipher,
    CryptoResult, HashChain, SimpleFeistel, SimpleRSA, VigenereCipher, XorCipher,
};
