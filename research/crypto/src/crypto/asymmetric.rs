use num_bigint::{BigInt, BigUint, RandBigInt, Sign, ToBigInt};
use num_traits::{One, Zero};

pub struct SimpleRSA {
    pub n: BigUint,
    pub e: BigUint,
    pub d: Option<BigUint>, // private exponent
}

impl SimpleRSA {
    /// Generate tiny RSA keypair (educational)
    pub fn generate_keypair(bits: usize) -> (Self, Self) {
        let mut rng = rand::thread_rng();
        let p = Self::generate_prime(&mut rng, bits / 2);
        let q = Self::generate_prime(&mut rng, bits / 2);

        let n = &p * &q;
        let phi = (&p - 1u32) * (&q - 1u32);

        let e = BigUint::from(65_537u32);
        let d = Self::mod_inverse(&e, &phi).expect("failed to compute modular inverse");

        let public = Self {
            n: n.clone(),
            e: e.clone(),
            d: None,
        };
        let private = Self { n, e, d: Some(d) };
        (public, private)
    }

    fn generate_prime(rng: &mut impl RandBigInt, bits: usize) -> BigUint {
        // gen_biguint / set_bit 은 u64 인자를 받음
        let bits_u64 = bits as u64;

        loop {
            let mut cand = rng.gen_biguint(bits_u64); // <- u64
            cand.set_bit(0, true); // 홀수 보장
            cand.set_bit(bits_u64.saturating_sub(1), true); // 최상위 비트 보장
            if Self::is_probably_prime(&cand) {
                return cand;
            }
        }
    }

    /// Very small Miller–Rabin with a few deterministic bases (OK for small bits, educational)
    fn is_probably_prime(n: &BigUint) -> bool {
        if n < &BigUint::from(2u32) {
            return false;
        }
        if n == &BigUint::from(2u32) || n == &BigUint::from(3u32) {
            return true;
        }
        if n % 2u32 == BigUint::zero() {
            return false;
        }

        // write n-1 as d * 2^s
        let mut d = n - 1u32;
        let mut s = 0u32;
        while &d % 2u32 == BigUint::zero() {
            d >>= 1u32;
            s += 1;
        }

        let bases: [u32; 7] = [2, 3, 5, 7, 11, 13, 17];
        'outer: for &a in &bases {
            if BigUint::from(a) >= *n {
                continue;
            }
            let mut x = Self::mod_pow(&BigUint::from(a), &d, n);
            if x == BigUint::one() || x == n - 1u32 {
                continue 'outer;
            }
            for _ in 0..(s - 1) {
                x = (&x * &x) % n;
                if x == n - 1u32 {
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }

    /// Extended GCD over BigInt to handle negative coefficients
    fn egcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
        if b.is_zero() {
            (a.clone(), BigInt::one(), BigInt::zero())
        } else {
            let (g, x, y) = Self::egcd(b, &(a % b));
            (g, y.clone(), x - (a / b) * y)
        }
    }

    fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
        let (ai, mi) = (a.to_bigint()?, m.to_bigint()?);
        let (g, mut x, _) = Self::egcd(&ai, &mi);
        if g != BigInt::one() {
            return None;
        }
        x %= &mi;
        if x.sign() == Sign::Minus {
            x += &mi;
        }
        x.to_biguint()
    }

    fn mod_pow(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
        let mut result = BigUint::one();
        let mut b = base % modulus;
        let mut e = exp.clone();
        while e > BigUint::zero() {
            if &e % 2u32 == BigUint::one() {
                result = (result * &b) % modulus;
            }
            e >>= 1u32;
            b = (&b * &b) % modulus;
        }
        result
    }

    pub fn encrypt(&self, message: &BigUint) -> BigUint {
        Self::mod_pow(message, &self.e, &self.n)
    }

    pub fn decrypt(&self, ciphertext: &BigUint) -> Option<BigUint> {
        self.d
            .as_ref()
            .map(|d| Self::mod_pow(ciphertext, d, &self.n))
    }
}
