// research/crypto/src/crypto/utils.rs

use num_bigint::{BigUint, RandBigInt};
use rand::rngs::OsRng;
use std::fmt;

pub type CryptoResult<T> = Result<T, CryptoError>;

#[derive(Debug)]
pub struct CryptoError(pub String);

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CryptoError: {}", self.0)
    }
}
impl std::error::Error for CryptoError {}

/// 바이트 -> 소문자 헥스
pub fn bytes_to_hex(data: &[u8]) -> String {
    let mut s = String::with_capacity(data.len() * 2);
    for b in data {
        use std::fmt::Write as _;
        let _ = write!(&mut s, "{:02x}", b);
    }
    s
}

/// 헥스(공백없음) -> 바이트
pub fn hex_to_bytes(s: &str) -> CryptoResult<Vec<u8>> {
    let s = s.trim();
    if s.len() % 2 != 0 {
        return Err(CryptoError("hex length must be even".into()));
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    for i in (0..s.len()).step_by(2) {
        let byte = u8::from_str_radix(&s[i..i + 2], 16)
            .map_err(|e| CryptoError(format!("invalid hex at {}: {e}", i)))?;
        out.push(byte);
    }
    Ok(out)
}

/// Shannon 엔트로피 (단위: bit/byte)
pub fn calculate_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut cnt = [0usize; 256];
    for &b in data {
        cnt[b as usize] += 1;
    }
    let n = data.len() as f64;
    let mut h = 0.0;
    for c in cnt {
        if c > 0 {
            let p = c as f64 / n;
            h -= p * p.log2();
        }
    }
    h
}

/// 지정 비트수의 랜덤 BigUint 생성 (최상위 비트 보장)
pub fn gen_random_biguint(bits: usize) -> BigUint {
    let mut rng = OsRng;
    let mut cand = rng.gen_biguint(bits as u64);
    if bits > 0 {
        cand.set_bit((bits - 1) as u64, true);
    }
    cand
}
