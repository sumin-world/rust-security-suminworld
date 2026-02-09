//! KMP-based packet pattern matcher with streaming support and payload fuzzing.
//!
//! # Example
//! ```
//! use packet_match_fuzz::{KmpMatcher, StreamMatcher};
//!
//! let matcher = KmpMatcher::new(b"HTTP");
//! assert_eq!(matcher.find_all(b"GET / HTTP/1.1\r\nHTTP"), vec![6, 16]);
//!
//! let mut stream = StreamMatcher::new(b"AB");
//! assert_eq!(stream.feed(b"xxA"), vec![]);
//! assert_eq!(stream.feed(b"Byy"), vec![2]); // global offset 2
//! ```

mod fuzz;
mod kmp;
mod stream;

pub use fuzz::{Fuzzer, MutationStrategy};
pub use kmp::KmpMatcher;
pub use stream::StreamMatcher;
