/// Knuth-Morris-Pratt pattern matcher operating on byte slices.
///
/// Pre-computes a failure (partial-match) table for the pattern, then scans
/// input in O(n) time with O(m) space.
#[derive(Debug, Clone)]
pub struct KmpMatcher {
    pattern: Vec<u8>,
    failure: Vec<usize>,
}

impl KmpMatcher {
    /// Build a new matcher for the given `pattern`.
    ///
    /// # Panics
    /// Panics if `pattern` is empty.
    pub fn new(pattern: &[u8]) -> Self {
        assert!(!pattern.is_empty(), "pattern must not be empty");
        let failure = Self::build_failure_table(pattern);
        Self {
            pattern: pattern.to_vec(),
            failure,
        }
    }

    /// Return all starting indices where the pattern occurs in `text`.
    pub fn find_all(&self, text: &[u8]) -> Vec<usize> {
        let mut matches = Vec::new();
        let mut j = 0; // chars matched so far in pattern

        for (i, &byte) in text.iter().enumerate() {
            while j > 0 && self.pattern[j] != byte {
                j = self.failure[j - 1];
            }
            if self.pattern[j] == byte {
                j += 1;
            }
            if j == self.pattern.len() {
                matches.push(i + 1 - j);
                j = self.failure[j - 1];
            }
        }
        matches
    }

    /// Return the index of the first occurrence, or `None`.
    pub fn find_first(&self, text: &[u8]) -> Option<usize> {
        let mut j = 0;
        for (i, &byte) in text.iter().enumerate() {
            while j > 0 && self.pattern[j] != byte {
                j = self.failure[j - 1];
            }
            if self.pattern[j] == byte {
                j += 1;
            }
            if j == self.pattern.len() {
                return Some(i + 1 - j);
            }
        }
        None
    }

    /// Check whether `text` contains the pattern at all.
    pub fn contains(&self, text: &[u8]) -> bool {
        self.find_first(text).is_some()
    }

    /// Return a reference to the underlying pattern.
    pub fn pattern(&self) -> &[u8] {
        &self.pattern
    }

    /// Build the KMP failure / partial-match table.
    fn build_failure_table(pattern: &[u8]) -> Vec<usize> {
        let m = pattern.len();
        let mut table = vec![0usize; m];
        let mut len = 0; // length of previous longest prefix-suffix

        for i in 1..m {
            while len > 0 && pattern[len] != pattern[i] {
                len = table[len - 1];
            }
            if pattern[len] == pattern[i] {
                len += 1;
            }
            table[i] = len;
        }
        table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_match() {
        let m = KmpMatcher::new(b"ABC");
        assert_eq!(m.find_all(b"xxABCyyABC"), vec![2, 7]);
    }

    #[test]
    fn overlapping_matches() {
        let m = KmpMatcher::new(b"AA");
        assert_eq!(m.find_all(b"AAAA"), vec![0, 1, 2]);
    }

    #[test]
    fn no_match() {
        let m = KmpMatcher::new(b"XYZ");
        assert!(m.find_all(b"ABCDEF").is_empty());
        assert!(!m.contains(b"ABCDEF"));
    }

    #[test]
    fn single_byte_pattern() {
        let m = KmpMatcher::new(b"\x00");
        assert_eq!(m.find_all(b"\x00\x01\x00"), vec![0, 2]);
    }

    #[test]
    fn find_first() {
        let m = KmpMatcher::new(b"needle");
        let text = b"hay needle stack needle";
        assert_eq!(m.find_first(text), Some(4));
    }

    #[test]
    fn full_text_is_pattern() {
        let m = KmpMatcher::new(b"ABCD");
        assert_eq!(m.find_all(b"ABCD"), vec![0]);
    }

    #[test]
    #[should_panic(expected = "must not be empty")]
    fn empty_pattern_panics() {
        KmpMatcher::new(b"");
    }
}
