/// Simple XOR Cipher
pub struct XorCipher {
    key: Vec<u8>,
}

impl XorCipher {
    pub fn new(key: &[u8]) -> Self {
        assert!(!key.is_empty(), "XorCipher key must not be empty");
        Self { key: key.to_vec() }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &b)| b ^ self.key[i % self.key.len()])
            .collect()
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        self.encrypt(data)
    }
}

/// Simple Feistel Network (educational)
pub struct SimpleFeistel {
    rounds: usize,
    round_keys: Vec<u32>,
}

impl SimpleFeistel {
    pub fn new(key: u64, rounds: usize) -> Self {
        assert!(rounds > 0, "rounds must be > 0");
        let mut round_keys = Vec::with_capacity(rounds);
        let mut k = key;
        for _ in 0..rounds {
            round_keys.push((k & 0xFFFF_FFFF) as u32);
            k = k.rotate_left(8) ^ 0x9E37_79B9;
        }
        Self { rounds, round_keys }
    }

    #[inline]
    fn f_function(&self, right: u32, round_key: u32) -> u32 {
        let mixed = right ^ round_key;
        mixed.rotate_left(7) ^ mixed.rotate_right(11)
    }

    pub fn encrypt(&self, plaintext: u64) -> u64 {
        let mut left = (plaintext >> 32) as u32;
        let mut right = (plaintext & 0xFFFF_FFFF) as u32;

        for r in 0..self.rounds {
            let tmp = left ^ self.f_function(right, self.round_keys[r]);
            left = right;
            right = tmp;
        }
        ((right as u64) << 32) | (left as u64)
    }

    pub fn decrypt(&self, ciphertext: u64) -> u64 {
        let mut left = (ciphertext >> 32) as u32;
        let mut right = (ciphertext & 0xFFFF_FFFF) as u32;

        for r in (0..self.rounds).rev() {
            let tmp = left ^ self.f_function(right, self.round_keys[r]);
            left = right;
            right = tmp;
        }
        ((right as u64) << 32) | (left as u64)
    }
}
