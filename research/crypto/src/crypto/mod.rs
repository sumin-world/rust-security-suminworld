pub mod classical;
pub mod symmetric;
pub mod asymmetric;
pub mod hash;
pub mod utils;

pub use classical::{CaesarCipher, VigenereCipher};
pub use symmetric::{XorCipher, SimpleFeistel};
pub use hash::{simple_hash, HashChain};
pub use asymmetric::SimpleRSA;
pub use utils::{bytes_to_hex, hex_to_bytes, calculate_entropy, CryptoResult};

pub fn run_crypto_demo() {
    println!("Cryptography demo initialized successfully.");
}
