use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum JSON {
    Str(String),
    Object(HashMap<String, JSON>),
    Array(Vec<JSON>),
    Null,
    Number(f32),
    Boolean(bool),
}

pub mod parser {
    use super::JSON;
    use super::JSON::*;
    use crate::lexer::LexToken;
    use std::collections::HashMap;

    pub fn parse_obj(_tokens: Vec<LexToken>) -> (JSON, Vec<LexToken>) {
        let mut tokens = _tokens;
        let mut map: HashMap<String, JSON> = HashMap::new();
        let mut t = tokens.get(0);
        if let Some(LexToken::RightCurly) = t {
            tokens.remove(0);
            return (Object(map), tokens.to_vec());
        }

        loop {
            let Some(LexToken::Str(key)) = tokens.get(0).map(|x|x.clone()) else {
              panic!("Expected object key, found: {}", tokens.get(0).unwrap());
            };
            tokens.remove(0);
            let Some(LexToken::Colon) = tokens.get(0) else {
                panic!("Expected ':', found: {}", tokens.get(0).unwrap());
              };
            tokens.remove(0);
            let (value, _new_tokens) = parse(tokens.to_vec());
            tokens = _new_tokens;

            map.insert(key.to_string(), value);

            t = tokens.get(0);

            match t {
                Some(LexToken::RightCurly) => {
                    tokens.remove(0);
                    return (Object(map), tokens.to_vec());
                }
                Some(LexToken::Comma) => {
                    tokens.remove(0);
                }
                _ => panic!("Expected ',', found: {}", tokens.get(0).unwrap()),
            }
        }
    }
    pub fn parse_array(_tokens: Vec<LexToken>) -> (JSON, Vec<LexToken>) {
        let mut tokens = _tokens;
        let mut arr: Vec<JSON> = Vec::new();
        let mut t = tokens.get(0);
        if let Some(LexToken::RightBracket) = t {
            tokens.remove(0);
            return (Array(arr), tokens.to_vec());
        }
        loop {
            let (value, _new_tokens) = parse(tokens.to_vec());
            tokens = _new_tokens;
            arr.push(value);
            t = tokens.get(0);
            match t {
                Some(LexToken::RightBracket) => {
                    tokens.remove(0);
                    return (Array(arr), tokens.to_vec());
                }
                Some(LexToken::Comma) => {
                    tokens.remove(0);
                }
                _ => panic!("Expected ',', found: {}", tokens.get(0).unwrap()),
            }
        }
    }
    pub fn parse(_tokens: Vec<LexToken>) -> (JSON, Vec<LexToken>) {
        let mut tokens = _tokens;
        let Some(t) = tokens.get(0).map(|x| x.clone()) else {
            return (Null, Vec::new())
        };
        tokens.remove(0);
        let new_tokens = tokens.to_vec();
        match t {
            LexToken::LeftCurly => parse_obj(new_tokens),
            LexToken::LeftBracket => parse_array(new_tokens),
            LexToken::True => (Boolean(true), new_tokens),
            LexToken::False => (Boolean(false), new_tokens),
            LexToken::Int(int) => (Number(int as f32), new_tokens),
            LexToken::Str(str) => (Str(str), new_tokens),
            LexToken::Float(f) => (Number(f), new_tokens),
            _ => (Null, new_tokens),
        }
    }
}
