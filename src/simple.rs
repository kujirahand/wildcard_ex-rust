/** Wildcard library simple implementation */

 /// check if the pattern matches the text with wildcard characters ['*', '?', '#']
 pub fn is_match(pattern: &str, text: &str) -> bool {
    let pattern_chars = pattern.chars().collect::<Vec<char>>();
    let text_chars = text.chars().collect::<Vec<char>>();
    is_match_slice(&pattern_chars, &text_chars)
 }

 /// check if the pattern matches the text with wildcard characters ['*', '?', '#']
pub fn is_match_slice(pattern_chars: &[char], text_chars: &[char]) -> bool {
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
        if pattern_chars[i] == '#' && text_chars[j].is_ascii_digit() {
            i += 1;
            j += 1;
            continue;
        }
        if pattern_chars[i] == '*' {
            i += 1;
            if pattern_chars.len() == i { // match until the end of the string
                return true;
            }
            // check patterns recursively
            let sub_pattern = &pattern_chars[i..];
            for j2 in j..text_chars.len() {
                if is_match_slice(sub_pattern, &text_chars[j2..]) {
                    return true;
                }
            }
            return false;
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
        // wildcard '#'
        assert_eq!(is_match("###.zip", "123.zip"), true);
        assert_eq!(is_match("###.zip", "1234.zip"), false);
        assert_eq!(is_match("a###.zip", "a123.zip"), true);
        assert_eq!(is_match("a###.zip", "a1234.zip"), false);
        assert_eq!(is_match("###-####", "123-4567"), true);
        assert_eq!(is_match("###-####", "123-45678"), false);
        assert_eq!(is_match("###-####", "1234-567"), false);
        // multibytes characters
        assert_eq!(is_match("*.txt", "格言.txt"), true);
        assert_eq!(is_match("??.txt", "格言.txt"), true);
        assert_eq!(is_match("格言.*", "格言.txt"), true);
        assert_eq!(is_match("迷言.*", "格言.txt"), false);
        assert_eq!(is_match("頓珍漢.*", "格言.txt"), false);
        // multiple wildcards
        assert_eq!(is_match("a*t*.zip", "abc.txt.zip"), true);
        // others
        assert_eq!(is_match("abc.*.txt", "abc.txt.zip"), false);
        assert_eq!(is_match("a*.zip", "abc.zip.zip"), true);
        assert_eq!(is_match("a*.zip", "abc.zip.zip.txt"), false);
        assert_eq!(is_match("a*.zip", "aaaaa.zip.zip.txt"), false);
        assert_eq!(is_match("a*.zip", "aaaaa.zip.zip.txt.zip"), true);
    }
}

