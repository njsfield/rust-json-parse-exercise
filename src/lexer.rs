use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LexToken {
    Comma,
    Colon,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    True,
    False,
    Float(f32),
    Int(i32),
    Str(String),
    Null,
}

use LexToken::*;

impl Display for LexToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeftCurly => write!(f, "{{"),
            RightCurly => write!(f, "}}"),
            LeftBracket => write!(f, "["),
            RightBracket => write!(f, "]"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            Int(n) => write!(f, "{}", n),
            Float(n) => write!(f, "{}", n),
            Str(s) => write!(f, r#""{}""#, s),
            Comma => write!(f, ","),
            Colon => write!(f, ":"),
            Null => write!(f, "null"),
        }
    }
}

pub mod lexer {
    use super::LexToken;
    use super::LexToken::*;
    use regex::Regex;
    fn lex_get(s: &str) -> (Option<LexToken>, &str) {
        match s {
            _ if s.starts_with('{') => (Some(LeftCurly), &s[1..]),
            _ if s.starts_with('}') => (Some(RightCurly), &s[1..]),
            _ if s.starts_with('[') => (Some(LeftBracket), &s[1..]),
            _ if s.starts_with(']') => (Some(RightBracket), &s[1..]),
            _ if s.starts_with(',') => (Some(Comma), &s[1..]),
            _ if s.starts_with(':') => (Some(Colon), &s[1..]),
            _ if s.starts_with('"') && s[1..].find('"').is_some() => {
                let pos = s[1..].find('"').unwrap();
                (Some(Str(s[1..pos + 1].into())), &s[pos + 2..])
            }
            _ if s.chars().nth(0).unwrap().is_digit(10) => {
                let num_reg = Regex::new(r"^(\d+(\.\d+)?)").unwrap();
                let num_str = num_reg.captures(s).unwrap().get(0).unwrap().as_str();
                if num_str.contains('.') {
                    return (Some(Float(num_str.parse().unwrap())), &s[num_str.len()..]);
                }
                (Some(Int(num_str.parse().unwrap())), &s[num_str.len()..])
            }
            _ if s.starts_with("true") => (Some(True), &s[4..]),
            _ if s.starts_with("false") => (Some(False), &s[5..]),
            _ if s.starts_with("null") => (Some(Null), &s[4..]),
            _ => (None, &s[1..]),
        }
    }

    pub fn lex(s: &str) -> Vec<LexToken> {
        let mut tokens: Vec<LexToken> = Vec::new();
        let mut str = s;
        while str.len() != 0 {
            let (res, new_str) = lex_get(str);
            str = new_str;
            if let Some(t) = res {
                tokens.push(t);
            }
        }
        tokens
    }
}
