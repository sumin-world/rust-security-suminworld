/// Caesar Cipher - simple substitution cipher
pub struct CaesarCipher {
    shift: u8,
}

impl CaesarCipher {
    pub fn new(shift: u8) -> Self {
        Self { shift: shift % 26 }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        plaintext
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shifted = ((c as u8 - base + self.shift) % 26) + base;
                    shifted as char
                } else {
                    c
                }
            })
            .collect()
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {
        let reverse_shift = (26 - self.shift) % 26;
        CaesarCipher::new(reverse_shift).encrypt(ciphertext)
    }
}

/// Vigenere Cipher - polyalphabetic substitution (optimized)
pub struct VigenereCipher {
    shifts: Vec<u8>, // 0..=25
}

impl VigenereCipher {
    pub fn new(key: &str) -> Self {
        let shifts: Vec<u8> = key
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_uppercase() as u8 - b'A')
            .collect();
        assert!(
            !shifts.is_empty(),
            "VigenereCipher key must contain at least one alphabetic character"
        );
        Self { shifts }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let mut out = String::with_capacity(plaintext.len());
        let mut i = 0usize;
        for c in plaintext.chars() {
            if c.is_ascii_alphabetic() {
                let shift = self.shifts[i % self.shifts.len()];
                let is_lower = c.is_ascii_lowercase();
                let up = c.to_ascii_uppercase() as u8;
                let enc = ((up - b'A' + shift) % 26) + b'A';
                out.push(if is_lower { (enc + 32) as char } else { enc as char });
                i += 1;
            } else {
                out.push(c);
            }
        }
        out
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {
        let mut out = String::with_capacity(ciphertext.len());
        let mut i = 0usize;
        for c in ciphertext.chars() {
            if c.is_ascii_alphabetic() {
                let shift = self.shifts[i % self.shifts.len()];
                let is_lower = c.is_ascii_lowercase();
                let up = c.to_ascii_uppercase() as u8;
                let dec = ((up - b'A' + 26 - shift) % 26) + b'A';
                out.push(if is_lower { (dec + 32) as char } else { dec as char });
                i += 1;
            } else {
                out.push(c);
            }
        }
        out
    }
}
