/** VB Like wildcard */

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
    /// skip next character
    pub fn next_n(&mut self, n: usize) {
        self.index += n;
    }
    /// get next character and skip escape character '\'
    pub fn next_char_esc(&mut self) -> char {
        let c = self.next();
        if c == '\\' {
            self.next()
        } else {
            c
        }
    }
    /// compare the next characters with the string
    pub fn eq_str(&self, s: &str) -> bool {
        let ss = s.chars().collect::<Vec<char>>();
        for i in 0..ss.len() {
            if self.index + i >= self.chars.len() {
                return false;
            }
            if self.chars[self.index + i] != ss[i] {
                return false;
            }
        }
        true
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
    Wildcard(char), // '*'
    CharList(Vec<CharRange>), // [charlist]
    NotCharList(Vec<CharRange>), // [!charlist]
    CharListRepeat(Vec<CharRange>), // [*charlist]
    NotCharListRepeat(Vec<CharRange>), // [-charlist]
    Selector(Vec<String>), // [=str1|str2|str3]
}

impl PatternChar {
    /// check if the pattern character matches the text
    pub fn is_match(&mut self, cur: &mut StrCursor) -> bool {
        match self {
            PatternChar::Char(ch) => {
                if cur.has_next() && cur.peek() == *ch {
                    cur.next();
                    return true;
                }
            }
            PatternChar::Number => {
                if cur.has_next() && cur.peek().is_ascii_digit() {
                    cur.next();
                    return true;
                }
            }
            PatternChar::Question => {
                if cur.has_next() {
                    cur.next();
                    return true;
                }
            }
            PatternChar::Wildcard(next_char) => {
                if *next_char == '\0' {
                    cur.index = cur.chars.len();
                    return true;
                }
                while cur.has_next() {
                    if cur.peek() == *next_char {
                        return true;
                    }
                    cur.next();
                }
                return true;
            }
            PatternChar::CharList(charlist) => {
                if cur.has_next() {
                    let ch = cur.peek();
                    if charlist.iter().any(|r| r.contains(ch)) {
                        cur.next();
                        return true;
                    }
                }
            }
            PatternChar::NotCharList(charlist) => {
                if cur.has_next() {
                    let ch = cur.peek();
                    if !charlist.iter().any(|r| r.contains(ch)) {
                        cur.next();
                        return true;
                    }
                }
            }
            PatternChar::CharListRepeat(charlist) => {
                while cur.has_next() {
                    let ch = cur.peek();
                    if !charlist.iter().any(|r| r.contains(ch)) {
                        break;
                    }
                    cur.next();
                }
                return true;
            }
            PatternChar::NotCharListRepeat(charlist) => {
                while cur.has_next() {
                    let ch = cur.peek();
                    if charlist.iter().any(|r| r.contains(ch)) {
                        break;
                    }
                    cur.next();
                }
                return true;
            }
            PatternChar::Selector(selector) => {
                for pat in selector.iter() {
                    if cur.eq_str(pat) {
                        cur.next_n(pat.len());
                        return true;
                    }
                }
            }
        }
        false
    }

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
    pub index: usize,
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
                '*' => {
                    let next_char = pattern_cur.peek();
                    pattern.push(PatternChar::Wildcard(next_char));
                },
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
                        '*' => {
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
            index: 0,
            pattern,
        }
    }
    /// check if the pattern matches the text
    pub fn has_next(&self) -> bool {
        self.index < self.pattern.len()
    }
    /// get next pattern character
    pub fn next(&self) -> PatternChar {
        self.pattern[self.index].clone()
    }
}

/// check if the pattern matches the text
pub fn is_match(pattern: &str, text: &str) -> bool {
    let mut pattern = Pattern::new(pattern);
    let mut text_cur = StrCursor::new(text);
    while pattern.has_next() {
        let mut pat = pattern.next();
        if !pat.is_match(&mut text_cur) {
            return false;
        }
        pattern.index += 1;
    }
    !text_cur.has_next() && !pattern.has_next()
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
    }
}