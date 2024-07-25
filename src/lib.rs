//! # Wildcard library wildcard_ex
//! 
//! This is a library for extended wildcards that allows VB-like specifications.
//! It enables the expression of repeating arbitrary strings with simple specifications using wildcards.
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
//! ## Using ex:Pattern objects
//! 
//! ```rust
//! use wildcard_ex::*;
//! fn main() {
//!     let pattern = ex::Pattern::new("*.txt");
//!     assert_eq!(pattern.is_match("abc.txt"), true);
//!     assert_eq!(pattern.is_match("abc.zip"), false);
//! }
//! ```

pub mod simple;
pub mod ex;

/// check if the pattern matches the text with wildcard characters ['*', '?', '#']
pub fn is_match_simple(pattern: &str, text: &str) -> bool {
    simple::is_match(pattern, text)
}

/// check if the pattern matches the text with wildcard characters ['*', '?', '#', "[...]"]
pub fn is_match(pattern: &str, text: &str) -> bool {
    ex::is_match(pattern, text)
}

