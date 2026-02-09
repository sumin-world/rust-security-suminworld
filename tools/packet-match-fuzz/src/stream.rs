/// Streaming KMP matcher that retains state across successive `feed()` calls.
///
/// Useful for matching patterns that may span multiple network packets.
use crate::kmp::KmpMatcher;

pub struct StreamMatcher {
    inner: KmpMatcher,
    /// Number of pattern bytes matched so far (carry-over between feeds).
    state: usize,
    /// Total bytes fed so far (for global offset reporting).
    global_offset: usize,
}

impl StreamMatcher {
    /// Create a new streaming matcher for `pattern`.
    pub fn new(pattern: &[u8]) -> Self {
        Self {
            inner: KmpMatcher::new(pattern),
            state: 0,
            global_offset: 0,
        }
    }

    /// Feed a chunk of data and return **global** byte offsets of every match
    /// start found (including matches that span two consecutive chunks).
    pub fn feed(&mut self, chunk: &[u8]) -> Vec<usize> {
        let pattern = self.inner.pattern();
        let failure = build_failure_table(pattern);
        let mut matches = Vec::new();
        let mut j = self.state;

        for (i, &byte) in chunk.iter().enumerate() {
            while j > 0 && pattern[j] != byte {
                j = failure[j - 1];
            }
            if pattern[j] == byte {
                j += 1;
            }
            if j == pattern.len() {
                let global_start = self.global_offset + i + 1 - j;
                matches.push(global_start);
                j = failure[j - 1];
            }
        }

        self.state = j;
        self.global_offset += chunk.len();
        matches
    }

    /// Reset the matcher state (but keep the same pattern).
    pub fn reset(&mut self) {
        self.state = 0;
        self.global_offset = 0;
    }

    /// Total bytes processed so far.
    pub fn bytes_processed(&self) -> usize {
        self.global_offset
    }
}

/// Re-build failure table (we keep the function private here to avoid
/// exposing internals; the cost is negligible compared to I/O).
fn build_failure_table(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let mut table = vec![0usize; m];
    let mut len = 0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_chunk() {
        let mut sm = StreamMatcher::new(b"AB");
        assert_eq!(sm.feed(b"xxAByy"), vec![2]);
    }

    #[test]
    fn pattern_spans_chunks() {
        let mut sm = StreamMatcher::new(b"ABCD");
        assert_eq!(sm.feed(b"xxAB"), Vec::<usize>::new());
        assert_eq!(sm.feed(b"CDyy"), vec![2]); // global offset 2
    }

    #[test]
    fn multiple_matches_across_chunks() {
        let mut sm = StreamMatcher::new(b"XX");
        assert_eq!(sm.feed(b"aXX"), vec![1]);
        assert_eq!(sm.feed(b"bXXc"), vec![4]);
        assert_eq!(sm.bytes_processed(), 7);
    }

    #[test]
    fn reset_clears_state() {
        let mut sm = StreamMatcher::new(b"AB");
        sm.feed(b"A"); // partial
        sm.reset();
        // After reset, the trailing 'A' is forgotten
        assert_eq!(sm.feed(b"Bxx"), Vec::<usize>::new());
    }
}
