///
/// # VBLike Pattern Matching
///

/// String Cursor
#[derive(Clone, Debug)]
pub struct StrCursor {
    pub index: usize,
    pub chars: Vec<char>,
}
impl StrCursor {
    /// create a new string cursor
    pub fn new(s: &str) -> Self {
        StrCursor {
            index: 0,
            chars: s.chars().collect(),
        }
    }
    /// check if there is next character
    pub fn has_next(&self) -> bool {
        self.index < self.chars.len()
    }
    /// peek next character
    pub fn peek(&self) -> char {
        if self.has_next() {
            return self.chars[self.index];
        }
        '\0'
    }
    /// get next character
    pub fn next(&mut self) -> char {
        if !self.has_next() {
            return '\0';
        }
        let c = self.chars[self.index];
        self.index += 1;
        c
    }
    /// get next character and skip escape character '\'
    pub fn next_char_esc(&mut self) -> char {
        let c = self.next();
        if c == '\\' {
            let c2 = self.next();
            match c2 {
                't' => '\t',
                'n' => '\n',
                'r' => '\r',
                '0' => '\0',
                'x' | 'u' => {
                    let mut hex = String::new();
                    while self.has_next() {
                        let c3 = self.peek();
                        if c3.is_ascii_hexdigit() {
                            hex.push(c3);
                            self.next();
                            continue;
                        }
                        break;
                    }
                    let n = u32::from_str_radix(&hex, 16).unwrap_or(0);
                    std::char::from_u32(n).unwrap_or('\0')
                }
                _ => c2,
            }
        } else {
            c
        }
    }
}

/// Range of characters
#[derive(Clone, Debug, Copy)]
pub struct CharRange {
    pub start: char,
    pub end: char,
}
impl CharRange {
    /// create a new character range
    pub fn new(start: char, end: char) -> Self {
        CharRange { start, end }
    }
    /// check if the character is in the range
    pub fn contains(&self, ch: char) -> bool {
        ch >= self.start && ch <= self.end
    }
}

/// Pattern Character
#[derive(Debug,Clone)]
pub enum PatternChar {
    Char(char),
    Number, // '#'
    Question, // '?'
    Wildcard, // '*'
    CharList(Vec<CharRange>), // [charlist]
    NotCharList(Vec<CharRange>), // [!charlist]
    CharListRepeat(Vec<CharRange>), // [*charlist]
    NotCharListRepeat(Vec<CharRange>), // [-charlist]
    Selector(Vec<String>), // [=str1|str2|str3]
}

impl PatternChar {
    /// read character list
    fn read_charlist(pattern_cur: &mut StrCursor) -> Vec<CharRange> {
        let mut charlist = vec![];
        let mut start_char;
        let mut end_char;
        loop {
            let c = pattern_cur.next_char_esc();
            if c == ']' { break; }
            // next char is '-' ?
            if pattern_cur.peek() == '-' {
                start_char = c;
                pattern_cur.next(); // skip '-'
                end_char = pattern_cur.next_char_esc();
                if end_char == ']' {
                    charlist.push(CharRange::new(start_char, start_char));
                    charlist.push(CharRange::new(end_char, end_char));
                    break;
                }
                charlist.push(CharRange::new(start_char, end_char));
                continue;
            }
            charlist.push(CharRange::new(c, c));
        }
        charlist
    }
}

/// Pattern structure
#[derive(Clone, Debug)]
pub struct Pattern {
    pub pattern: Vec<PatternChar>,
}
impl Pattern {
    pub fn new(pattern_str: &str) -> Self {
        let mut pattern_cur = StrCursor::new(pattern_str);
        let mut pattern = vec![];
        while pattern_cur.has_next() {
            let c = pattern_cur.next();
            match c {
                '#' => pattern.push(PatternChar::Number),
                '?' => pattern.push(PatternChar::Question),
                '*' => pattern.push(PatternChar::Wildcard),
                '\\' => { // escape
                    let c = pattern_cur.next();
                    pattern.push(PatternChar::Char(c));
                },
                '[' => {
                    let c = pattern_cur.peek();
                    match c {
                        '!' => {
                            pattern_cur.next(); // skip '!'
                            let charlist = PatternChar::read_charlist(&mut pattern_cur);
                            pattern.push(PatternChar::NotCharList(charlist));
                        },
                        '+' => {
                            pattern_cur.next(); // skip '*'
                            let charlist = PatternChar::read_charlist(&mut pattern_cur);
                            pattern.push(PatternChar::CharListRepeat(charlist));
                        },
                        '-' => {
                            pattern_cur.next(); // skip '-'
                            let charlist = PatternChar::read_charlist(&mut pattern_cur);
                            pattern.push(PatternChar::NotCharListRepeat(charlist));
                        },
                        '=' => {
                            pattern_cur.next(); // skip '='
                            let mut selector = vec![];
                            while pattern_cur.has_next() {
                                let mut str = String::new();
                                let mut c = '\0';
                                while pattern_cur.has_next() {
                                    c = pattern_cur.next();
                                    if c == '|' || c == ']' { break; }
                                    if c == '\\' { c = pattern_cur.next(); }
                                    str.push(c);
                                }
                                selector.push(str);
                                if c == ']' { break; }
                            }
                            pattern.push(PatternChar::Selector(selector));
                        }
                        _ => {
                            let charlist = PatternChar::read_charlist(&mut pattern_cur);
                            pattern.push(PatternChar::CharList(charlist));
                        }
                    }
                }
                _ => pattern.push(PatternChar::Char(c)),
            }
        }
        Pattern {
            pattern,
        }
    }
    /// check if the pattern matches the text
    #[allow(dead_code)]
    pub fn is_match(&self, text: &str) -> bool {
        let text_vec = text.chars().collect::<Vec<char>>();
        is_match_slice(&self.pattern[..], &text_vec[..])
    }
    #[allow(dead_code)]
    pub fn is_match_all(&self, str_list: &[String]) -> Vec<bool> {
        let mut result = vec![];
        for text in str_list {
            let text_vec = text.chars().collect::<Vec<char>>();
            result.push(is_match_slice(&self.pattern[..], &text_vec[..]));
        }
        result
    }
    #[allow(dead_code)]
    pub fn filter(&self, str_list: &[String]) -> Vec<String> {
        let mut result = vec![];
        for text in str_list {
            let text_vec = text.chars().collect::<Vec<char>>();
            let b = is_match_slice(&self.pattern[..], &text_vec[..]);
            if b {
                result.push(text.clone());
            }
        }
        result
    }
}

/// check if the pattern matches the text
pub fn is_match(pattern: &str, text: &str) -> bool {
    let pattern_vec = Pattern::new(pattern);
    let text_vec = text.chars().collect::<Vec<char>>();
    is_match_slice(&pattern_vec.pattern[..], &text_vec[..])
}

/// check if the pattern matches the text
pub fn is_match_slice(pattern: &[PatternChar], text: &[char]) -> bool {
    let mut i = 0;
    let mut j = 0;
    while i < pattern.len() && j < text.len() {
        let pattern_char = &pattern[i];
        match pattern_char {
            PatternChar::Char(ch) => {
                if text[j] == *ch {
                    i += 1;
                    j += 1;
                    continue;
                }
                return false;
            }
            PatternChar::Number => {
                if text[j].is_ascii_digit() {
                    i += 1;
                    j += 1;
                    continue;
                }
                return false;
            }
            PatternChar::Question => {
                i += 1;
                j += 1;
                continue;
            }
            PatternChar::Wildcard => {
                i += 1; // skip '*'
                if pattern.len() == i { // match until the end of the string
                    return true;
                }
                let sub_pattern = &pattern[i..];
                for j2 in j..text.len() {
                    if is_match_slice(sub_pattern, &text[j2..]) {
                        return true;
                    }
                }
                return false;
            }
            PatternChar::CharList(charlist) => {
                let ch = text[j];
                if charlist_contains(charlist, ch) {
                    i += 1;
                    j += 1;
                    continue;
                }
                return false;
            }
            PatternChar::NotCharList(charlist) => {
                let ch = text[j];
                if !charlist_contains(charlist, ch) {
                    i += 1;
                    j += 1;
                    continue;
                }
                return false;
            }
            PatternChar::CharListRepeat(charlist) => {
                if !charlist_contains(charlist, text[j]) { return false; }
                i += 1;
                j += 1;
                while j < text.len() {
                    if !charlist_contains(charlist, text[j]) {
                        break;
                    }
                    j += 1;
                }
                continue;
            }
            PatternChar::NotCharListRepeat(charlist) => {
                if charlist_contains(charlist, text[j]) { return false; }
                i += 1;
                j += 1;
                while j < text.len() {
                    if charlist_contains(charlist, text[j]) {
                        break;
                    }
                    j += 1;
                }
                continue;
            }
            PatternChar::Selector(selector) => {
                let mut matched = false;
                i += 1;
                for substr in selector {
                    let substr_chars = substr.chars().collect::<Vec<char>>();
                    let subtext = &text[j..];
                    if subtext.starts_with(substr_chars.as_slice()){
                        j += substr_chars.len();
                        matched = true;
                        break;
                    }
                }
                if matched { continue; }
                return false;
            }
        }
    }
    (i >= pattern.len()) && (j >= text.len())
}

fn charlist_contains(charlist: &[CharRange], ch: char) -> bool {
    for range in charlist {
        if range.contains(ch) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_match_vblike() {
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
        assert_eq!(is_match("a?*.txt", "a1.txt"), true);
        assert_eq!(is_match("a?*.txt", "a1t"), false);
        assert_eq!(is_match("a?*.txt", "a1t.txt"), true);
        assert_eq!(is_match("a?*.txt", "a1t.t"), false);
        assert_eq!(is_match("a?*.txt", "a1t.tt"), false);
        assert_eq!(is_match("a?*.txt", "a1t.txtt"), false);
        assert_eq!(is_match("a?*.txt", "a1t.txt"), true);
        assert_eq!(is_match("a?*.txt", "a1t.txtt"), false);
        assert_eq!(is_match("a?*.txt", "a1t.txt"), true);
        // recursive wildcard '*'
        assert_eq!(is_match("abc.*.txt", "abc.txt.zip"), false);
        assert_eq!(is_match("a*.zip", "abc.zip.zip"), true);
        assert_eq!(is_match("a*.zip", "abc.zip.zip.txt"), false);
        assert_eq!(is_match("a*.zip", "aaaaa.zip.zip.txt"), false);
        assert_eq!(is_match("a*.zip", "aaaaa.zip.zip.txt.zip"), true);
    }

    #[test]
    fn test_is_match_vblike_charlist() {
        // wildcard [str]
        assert_eq!(is_match("abc[0-3].zip", "abc3.zip"), true);
        assert_eq!(is_match("abc[0-3].zip", "abc4.zip"), false);
        assert_eq!(is_match("abc[0-3].zip", "abc4.zip"), false);
        assert_eq!(is_match("abc[123].zip", "abc3.zip"), true);
        assert_eq!(is_match("abc[123].zip", "abc4.zip"), false);
        // wildcard [!str]
        assert_eq!(is_match("abc[!0-3].zip", "abc3.zip"), false);
        assert_eq!(is_match("abc[!0-3].zip", "abc4.zip"), true);
        assert_eq!(is_match("abc[!123].zip", "abc4.zip"), true);
        assert_eq!(is_match("abc[!123].zip", "abc2.zip"), false);
        // wildcard [+str]
        assert_eq!(is_match("abc[+0-9].zip", "abc123.zip"), true);
        assert_eq!(is_match("abc[+0-9\\-].zip", "abc123-456.zip"), true);
        // wildcard [-str]
        assert_eq!(is_match("abc[-\\.].zip", "abcABC.zip"), true);
        assert_eq!(is_match("a[-\\-]-[+0-9].zip", "abc123-456.zip"), true);
    }
    #[test]
    fn test_is_match_vblike_selector() {
        // wildcard [str]
        assert_eq!(is_match("[=cat|dog|penguin].zip", "cat.zip"), true);
        assert_eq!(is_match("[=cat|dog|penguin].zip", "pen.zip"), false);
    }
    #[test]
    fn test_is_match_vblike_esc() {
        // escape pattern
        assert_eq!(is_match("a[\\t]b", "a\tb"), true);
        assert_eq!(is_match("a[\\x09]b", "a\tb"), true);
        assert_eq!(is_match("a[+\\x09]b", "a\t\tb"), true);
    }
    #[test]
    fn test_is_match_multibytes() {
        assert_eq!(is_match("魚[あ-ん]ち.zip", "魚いち.zip"), true);
        assert_eq!(is_match("日本[!あ-ん].zip", "日本酒.zip"), true);
        assert_eq!(is_match("[魚牛豚]肉.zip", "魚肉.zip"), true);
        // repated pattern
        assert_eq!(is_match("[+あ-ん].zip", "いろは.zip"), true);
        assert_eq!(is_match("[+あ-ん].zip", "魚エラー.zip"), false);
        assert_eq!(is_match("魚[+ア-ン].zip", "魚図鑑.zip"), false);
        assert_eq!(is_match("[+いろはにほへと]うた.zip", "いろはうた.zip"), true);
        // selector
        assert_eq!(is_match("[=図鑑|資料|市場].zip", "市場.zip"), true);
        assert_eq!(is_match("魚[=図鑑|資料|市場]売店.zip", "魚市場売店.zip"), true);
    }

    #[test]
    fn test_is_match_strcut() {
        let pattern = Pattern::new("*.txt");
        assert_eq!(pattern.is_match("abc.txt"), true);
        assert_eq!(pattern.is_match("abc.zip"), false);
        assert_eq!(pattern.is_match("豚に真珠.txt"), true);
    }
    #[test]
    fn test_is_match_strcut2() {
        let pattern = Pattern::new("*.txt");
        let str_list = vec![
            "abc.txt".to_string(),
            "abc.zip".to_string(),
            "豚に真珠.txt".to_string(),
        ];
        assert_eq!(pattern.is_match_all(str_list.as_slice()), [true, false, true]);
    }
    #[test]
    fn test_is_match_strcut3() {
        let pattern = Pattern::new("*.txt");
        let str_list = vec![
            "abc.txt".to_string(),
            "abc.zip".to_string(),
            "豚に真珠.txt".to_string(),
        ];
        assert_eq!(pattern.filter(str_list.as_slice()), [
            "abc.txt".to_string(),
            "豚に真珠.txt".to_string(),
        ]);
    }

}