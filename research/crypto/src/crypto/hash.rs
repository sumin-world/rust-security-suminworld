/// Simple hash (FNV-1a style, educational)
pub fn simple_hash(data: &[u8]) -> u32 {
    let mut hash = 0x811C9DC5u32; // FNV offset basis
    for &b in data {
        hash ^= b as u32;
        hash = hash.wrapping_mul(0x0100_0193); // FNV prime
    }
    hash
}

/// Simple hash chain (educational, FNV-based)
pub struct HashChain {
    pub chain: Vec<String>,
    pub reduction: fn(u32, usize) -> String,
}

impl HashChain {
    pub fn new(reduction: fn(u32, usize) -> String) -> Self {
        Self { chain: Vec::new(), reduction }
    }

    pub fn generate_chain(&mut self, start_password: &str, length: usize) {
        self.chain.clear();
        let mut cur = start_password.to_string();
        for i in 0..length {
            self.chain.push(cur.clone());
            let h = simple_hash(cur.as_bytes());
            cur = (self.reduction)(h, i);
        }
    }

    /// Educational lookup: try to regenerate forward from each link
    pub fn lookup(&self, target_hash: u32) -> Option<String> {
        for start_idx in (0..self.chain.len()).rev() {
            let mut cur = self.chain[start_idx].clone();
            for i in start_idx..self.chain.len() {
                let h = simple_hash(cur.as_bytes());
                if h == target_hash {
                    return Some(cur);
                }
                cur = (self.reduction)(h, i);
            }
        }
        None
    }
}
