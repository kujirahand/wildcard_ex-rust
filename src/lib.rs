/** Wildcard library */
pub mod simple;
pub mod vblike;

/// check if the pattern matches the text with wildcard characters '*' and '?'
pub fn is_match_simple(pattern: &str, text: &str) -> bool {
    simple::is_match(pattern, text)
}


