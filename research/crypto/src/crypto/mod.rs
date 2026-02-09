pub mod asymmetric;
pub mod classical;
pub mod hash;
pub mod symmetric;
pub mod utils;

pub use asymmetric::SimpleRSA;
pub use classical::{CaesarCipher, VigenereCipher};
pub use hash::{simple_hash, HashChain};
pub use symmetric::{SimpleFeistel, XorCipher};
pub use utils::{bytes_to_hex, calculate_entropy, hex_to_bytes, CryptoResult};

pub fn run_crypto_demo() {
    println!("══════════════════════════════════════");
    println!("  Cryptography Toolkit — Demo");
    println!("══════════════════════════════════════\n");

    // --- Classical Ciphers ---
    println!("▸ Caesar Cipher (shift=3)");
    let caesar = CaesarCipher::new(3);
    let ct = caesar.encrypt("Hello World");
    let pt = caesar.decrypt(&ct);
    println!("  plain  → {ct}");
    println!("  cipher → {pt}\n");

    println!("▸ Vigenère Cipher (key=\"KEY\")");
    let vig = VigenereCipher::new("KEY");
    let ct = vig.encrypt("Attack at dawn");
    let pt = vig.decrypt(&ct);
    println!("  plain  → {ct}");
    println!("  cipher → {pt}\n");

    // --- Symmetric ---
    println!("▸ XOR Cipher");
    let xor = XorCipher::new(b"secret");
    let ct = xor.encrypt(b"Hello!");
    let pt = xor.decrypt(&ct);
    println!("  encrypted → {:?}", ct);
    println!("  decrypted → {}\n", String::from_utf8_lossy(&pt));

    println!("▸ Feistel Network (4 rounds)");
    let feistel = SimpleFeistel::new(0xDEAD_BEEF_CAFE_BABE, 4);
    let ct = feistel.encrypt(0x0123_4567_89AB_CDEF);
    let pt = feistel.decrypt(ct);
    println!("  encrypted → {ct:#018X}");
    println!("  decrypted → {pt:#018X}\n");

    // --- Hashing ---
    println!("▸ FNV-1a Hash");
    let h = simple_hash(b"rust-security");
    println!("  hash(\"rust-security\") → {h:#010X}\n");

    // --- Utilities ---
    println!("▸ Hex Encoding / Entropy");
    let hex = utils::bytes_to_hex(&[0xDE, 0xAD, 0xBE, 0xEF]);
    let ent = utils::calculate_entropy(b"aaaaabbbcc");
    println!("  bytes_to_hex → {hex}");
    println!("  entropy(\"aaaaabbbcc\") → {ent:.4} bits/byte\n");

    println!("══════════════════════════════════════");
    println!("  All demos completed successfully ✅");
    println!("══════════════════════════════════════");
}
