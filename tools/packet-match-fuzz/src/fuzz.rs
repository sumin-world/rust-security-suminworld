/// Simple byte-level mutation fuzzer for packet payloads.
///
/// Applies random mutations to a seed payload so that downstream matchers
/// can be stress-tested against malformed or adversarial input.
use rand::Rng;

/// Available mutation strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationStrategy {
    /// Flip random bits.
    BitFlip,
    /// Replace random bytes with random values.
    ByteReplace,
    /// Insert random bytes at random positions.
    ByteInsert,
    /// Delete random bytes.
    ByteDelete,
    /// Shuffle a random sub-range.
    ChunkShuffle,
}

impl MutationStrategy {
    /// Return all available strategies.
    pub fn all() -> &'static [Self] {
        &[
            Self::BitFlip,
            Self::ByteReplace,
            Self::ByteInsert,
            Self::ByteDelete,
            Self::ChunkShuffle,
        ]
    }
}

/// Payload fuzzer that mutates a seed byte vector.
pub struct Fuzzer {
    seed: Vec<u8>,
    strategies: Vec<MutationStrategy>,
    mutations_per_round: usize,
}

impl Fuzzer {
    /// Create a fuzzer from a seed payload.
    pub fn new(seed: &[u8]) -> Self {
        Self {
            seed: seed.to_vec(),
            strategies: MutationStrategy::all().to_vec(),
            mutations_per_round: 3,
        }
    }

    /// Limit the strategies to use.
    pub fn with_strategies(mut self, strategies: &[MutationStrategy]) -> Self {
        self.strategies = strategies.to_vec();
        self
    }

    /// Set how many mutations to apply per generated payload.
    pub fn with_mutations_per_round(mut self, n: usize) -> Self {
        self.mutations_per_round = n.max(1);
        self
    }

    /// Generate `count` mutated payloads.
    pub fn generate(&self, count: usize) -> Vec<Vec<u8>> {
        let mut rng = rand::thread_rng();
        (0..count).map(|_| self.mutate_once(&mut rng)).collect()
    }

    fn mutate_once(&self, rng: &mut impl Rng) -> Vec<u8> {
        let mut data = self.seed.clone();
        for _ in 0..self.mutations_per_round {
            if data.is_empty() {
                // Can only insert into an empty payload
                data.push(rng.gen());
                continue;
            }
            let strategy = self.strategies[rng.gen_range(0..self.strategies.len())];
            match strategy {
                MutationStrategy::BitFlip => {
                    let idx = rng.gen_range(0..data.len());
                    let bit = 1u8 << rng.gen_range(0..8u32);
                    data[idx] ^= bit;
                }
                MutationStrategy::ByteReplace => {
                    let idx = rng.gen_range(0..data.len());
                    data[idx] = rng.gen();
                }
                MutationStrategy::ByteInsert => {
                    let idx = rng.gen_range(0..=data.len());
                    data.insert(idx, rng.gen());
                }
                MutationStrategy::ByteDelete => {
                    let idx = rng.gen_range(0..data.len());
                    data.remove(idx);
                }
                MutationStrategy::ChunkShuffle => {
                    if data.len() >= 2 {
                        let a = rng.gen_range(0..data.len());
                        let b = rng.gen_range(0..data.len());
                        data.swap(a, b);
                    }
                }
            }
        }
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_requested_count() {
        let f = Fuzzer::new(b"AAAA");
        let payloads = f.generate(10);
        assert_eq!(payloads.len(), 10);
    }

    #[test]
    fn mutations_change_payload() {
        let seed = b"HELLO WORLD";
        let f = Fuzzer::new(seed).with_mutations_per_round(5);
        let payloads = f.generate(20);
        // At least some payloads should differ from the seed
        let differs = payloads.iter().filter(|p| p.as_slice() != seed).count();
        assert!(differs > 0, "expected at least one mutated payload");
    }

    #[test]
    fn single_strategy() {
        let f = Fuzzer::new(b"\x00\x00\x00\x00")
            .with_strategies(&[MutationStrategy::BitFlip])
            .with_mutations_per_round(1);
        let payloads = f.generate(5);
        // All payloads should still be 4 bytes (bit-flip doesn't change length)
        for p in &payloads {
            assert_eq!(p.len(), 4);
        }
    }

    #[test]
    fn empty_seed() {
        let f = Fuzzer::new(b"");
        let payloads = f.generate(5);
        // Should not panic; payloads will have some bytes from inserts
        assert_eq!(payloads.len(), 5);
    }
}
