//! # Wildcard library wildcard_ex
//! 
//! This is a library for extended wildcards that allows VB-like specifications.
//! It enables the expression of repeating arbitrary strings with simple specifications using wildcards.
//! 
//! ### Example - Basic Usage
//! 
//! ```rust
//! use wildcard_ex::{is_match, ex};
//! fn main() {
//!     // match with wildcard characters ['*', '?', '#', "[...]"]
//!     assert_eq!(is_match("*.txt", "abc.txt"), true);
//!     assert_eq!(is_match("test*.txt", "test1234.txt"), true);
//!     // using Pattern object
//!     let pattern = ex::Pattern::new("*.txt");
//!     assert_eq!(pattern.is_match("abc.txt"), true);
//!     assert_eq!(pattern.is_match("abc.zip"), false);
//! }
//! ```
//! 
//! ### Various pattern matching examples
//! 
//! ```rust
//! use wildcard_ex::*;
//! fn main() {
//!     // simple pattern matching with wildcard characters ['*', '?', '#']
//!     assert_eq!(is_match_simple("*.txt", "abc.txt"), true);
//!     assert_eq!(is_match_simple("*.txt", "abc.zip"), false);
//!     assert_eq!(is_match_simple("a???.txt", "abcd.txt"), true);
//!     assert_eq!(is_match_simple("zip:###-####", "zip:111-2222"), true);
//!     assert_eq!(is_match_simple("zip:###-####", "zip:12345"), false);
//!     // wildcard "[...]"
//!     assert_eq!(is_match("[a-z]1234.txt", "a1234.txt"), true);
//!     assert_eq!(is_match("[a-z][0-9].txt", "b5.txt"), true);
//!     assert_eq!(is_match("[!0-9][0-9].txt", "c3.txt"), true); // [!str] is other than str.
//!     assert_eq!(is_match("[+0-9].txt", "12345.txt"), true); // [+str] is repeated arbitrary string
//!     assert_eq!(is_match("[+a-z0-9].txt", "abc12345.txt"), true);
//!     assert_eq!(is_match("[=cat|dog].txt", "cat.txt"), true); // [=str1|str2] is str1 or str2
//! }
//! ```
//! 
//! Please refer to the following README to see which wildcard patterns can be used.
//! - [README.md](https://github.com/kujirahand/wildcard_ex-rust)
//! 
//! ### Using ex:Pattern object
//! 
//! - `is_match_simple` specifies general wildcards.
//! - `is_match` specifies extended wildcards.
//! 
//! ```rust
//! use wildcard_ex::*;
//! fn main() {
//!     let pattern = ex::Pattern::new("*.txt");
//!     assert_eq!(pattern.is_match("abc.txt"), true);
//!     assert_eq!(pattern.is_match("abc.zip"), false);
//! }
//! ```
//! 
//! ### extract matched part from beginning
//! 
//! ```rust
//! use wildcard_ex::*;
//! fn main() {
//!     // extract_match
//!     assert_eq!(extract_match("*.txt", "abc.txt"), Some("abc.txt".to_string()));
//!     assert_eq!(extract_match("hello*", "hello, world!"), Some("hello, world!".to_string()));
//!     // find_match
//!     let result = find_match("*.txt", "abc.txt").unwrap();
//!     assert_eq!(result.start, 0);
//!     assert_eq!(result.matched, "abc.txt".to_string());
//! }
//! ```

pub mod simple;
pub mod ex;

/// checks if the specified text completely matches the pattern and returns true if it. The pattern can include wildcards such as ['*', '?', '#'].
pub fn is_match_simple(pattern: &str, text: &str) -> bool {
    simple::is_match(pattern, text)
}

/// checks if the specified text completely matches the pattern and returns true if it. The pattern can include wildcards such as ['*', '?', '#', "[...]"].
pub fn is_match(pattern: &str, text: &str) -> bool {
    ex::is_match(pattern, text)
}

/// tests whether the text at the beginning matches the pattern and returns the matched part.
pub fn extract_match(pattern: &str, text: &str) -> Option<String> {
    ex::extract_match(pattern, text)
}

/// searches through the entire text from the beginning to find and extract the part that matches the pattern.
pub fn find_match(pattern: &str, text: &str) -> Option<ex::MatchedResult> {
    ex::find_match(pattern, text)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_match_simple() {
        // simple pattern
        assert_eq!(is_match("a", "b"), false);
        assert_eq!(is_match("a*.txt", "abc.txt"), true);
        assert_eq!(is_match_simple("a", "aa"), false);
        assert_eq!(is_match_simple("a*.txt", "abc.txt"), true);
    }
}
