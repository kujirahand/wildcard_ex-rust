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

/// extracts matched text from the beginning of string
pub fn extract_match(pattern: &str, text: &str) -> Option<String> {
    let pattern_chars = pattern.chars().collect::<Vec<char>>();
    let text_chars = text.chars().collect::<Vec<char>>();
    extract_match_slice(&pattern_chars, &text_chars)
}

/// extracts matched text from the beginning of string
pub fn extract_match_slice(pattern: &[char], text: &[char]) -> Option<String> {
    let mut i = 0;
    let mut j = 0;
    let mut matched = String::new();
    while i < pattern.len() && j < text.len() {
        if pattern[i] == text[j] {
            matched.push(text[j]);
            i += 1;
            j += 1;
            continue;
        }
        if pattern[i] == '?' {
            matched.push(text[j]);
            i += 1;
            j += 1;
            continue;
        }
        if pattern[i] == '#' && text[j].is_ascii_digit() {
            matched.push(text[j]);
            i += 1;
            j += 1;
            continue;
        }
        if pattern[i] == '*' {
            i += 1;
            if pattern.len() == i { // match until the end of the string
                let substring: String = text[j..].iter().collect();
                matched.push_str(&substring);
                return Some(matched);
            }
            // check patterns recursively
            let sub_pattern = &pattern[i..];
            for j2 in j..text.len() {
                if let Some(sub_matched) = extract_match_slice(sub_pattern, &text[j2..]) {
                    matched.push_str(&sub_matched);
                    return Some(matched);
                } else {
                    matched.push(text[j2]);
                }
            }
            return None;
        } else {
            return None;
        }
    }
    if i == pattern.len() {
        return Some(matched);
    }
    None
}

/// find_match's result
pub struct MatchedResult {
    pub start: usize,
    pub end: usize,
    pub matched: String,
}
impl MatchedResult {
    pub fn new(start: usize, end: usize, matched: String) -> Self {
        MatchedResult {
            start,
            end,
            matched,
        }
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

/// find a matching substring from the entire string.
pub fn find_match(pattern: &str, text: &str) -> Option<MatchedResult> {
    let pattern_chars = pattern.chars().collect::<Vec<char>>();
    let text_chars = text.chars().collect::<Vec<char>>();
    find_match_slice(&pattern_chars, &text_chars)
}

/// find a matching substring from the entire string.
pub fn find_match_slice(pattern: &[char], text: &[char]) -> Option<MatchedResult> {
    for j in 0..text.len() {
        let sub_text = &text[j..];
        if let Some(sub_matched) = extract_match_slice(pattern, sub_text) {
            let result = MatchedResult {
                start: j,
                end: j + sub_matched.len(),
                matched: sub_matched,
            };
            return Some(result);
        }
    }
    None
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
    #[test]
    fn test_extract_match() {
        assert_eq!(extract_match("a", "a"), Some("a".to_string()));
        assert_eq!(extract_match("abc", "abc"), Some("abc".to_string()));
        assert_eq!(extract_match("hello*", "hello_world"), Some("hello_world".to_string()));
        assert_eq!(extract_match("abc*g", "abcdefghijklmnopqrstuvwxyz"), Some("abcdefg".to_string()));
        assert_eq!(extract_match("hello*z", "hello_world"), None);
        assert_eq!(extract_match("abc*z", "abcdefg"), None);
        assert_eq!(extract_match("###-####", "111-2222"), Some("111-2222".to_string()));
        assert_eq!(extract_match("(###)###-####", "(111)222-3333"), Some("(111)222-3333".to_string()));
        assert_eq!(extract_match("abc", "a"), None);
    }
}

