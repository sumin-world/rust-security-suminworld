// research/crypto/src/lib.rs
pub mod crypto;

pub use crypto::{
    CaesarCipher, VigenereCipher,
    XorCipher, SimpleFeistel,
    simple_hash, HashChain,
    SimpleRSA,
    bytes_to_hex, hex_to_bytes, calculate_entropy, CryptoResult,
    run_crypto_demo,
};
