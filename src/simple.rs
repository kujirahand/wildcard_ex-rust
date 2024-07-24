/** Wildcard library simple implementation */

 /// check if the pattern matches the text with wildcard characters '*' and '?'
 pub fn is_match(pattern: &str, text: &str) -> bool {
    let pattern_chars = pattern.chars().collect::<Vec<char>>();
    let text_chars = text.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut j = 0;
    while i < pattern_chars.len() && j < text_chars.len() {
        if pattern_chars[i] == text_chars[j] {
            i += 1;
            j += 1;
            continue;
        }
        if pattern_chars[i] == '?' {
            i += 1;
            j += 1;
            continue;
        }
        if pattern_chars[i] == '*' {
            i += 1;
            if pattern_chars.len() == i { // match the rest of the text
                return true;
            }
            while j < text_chars.len() && pattern_chars[i] != text_chars[j] {
                j += 1;
            }
        } else {
            return false;
        }
    }
    (i == pattern_chars.len()) && (j == text_chars.len())
 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_match_simple() {
        // simple pattern
        assert_eq!(is_match("a", "a"), true);
        assert_eq!(is_match("a", "b"), false);
        assert_eq!(is_match("a", "aa"), false);
        assert_eq!(is_match("a", "aaa"), false);
        assert_eq!(is_match("a", "ab"), false);
        assert_eq!(is_match("a", "ba"), false);
        // wildcard '*' tail
        assert_eq!(is_match("a*", "ba"), false);
        assert_eq!(is_match("a*", "ab"), true);
        // wildcard '*' head
        assert_eq!(is_match("*.txt", "ab.txt"), true);
        assert_eq!(is_match("*.txt", "ab.t"), false);
        // wildcard '*' middle
        assert_eq!(is_match("a*.txt", "abc.txt"), true);
        assert_eq!(is_match("a*.tts", "abc.txt"), false);
        assert_eq!(is_match("a*c.txt", "abc.txt"), true);
        assert_eq!(is_match("a*d.txt", "abc.txt"), false);
        // wildcard '?' middle
        assert_eq!(is_match("a??.txt", "abc.txt"), true);
        assert_eq!(is_match("a?c.txt", "abbc.txt"), false);
        // wildcard '*' and '?'
        assert_eq!(is_match("a*c.txt", "abbc.txt"), true);
        assert_eq!(is_match("abc.*", "abc.txt"), true);
        assert_eq!(is_match("abc.???", "abc.txt"), true);
        assert_eq!(is_match("*abc.txt", "abc.txt"), true);
        assert_eq!(is_match("abc.*", "abc.txt"), true);
        // multibytes characters
        assert_eq!(is_match("*.txt", "格言.txt"), true);
        assert_eq!(is_match("??.txt", "格言.txt"), true);
        assert_eq!(is_match("格言.*", "格言.txt"), true);
        assert_eq!(is_match("迷言.*", "格言.txt"), false);
        assert_eq!(is_match("頓珍漢.*", "格言.txt"), false);

    }
}

