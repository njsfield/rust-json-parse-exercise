use crate::parser::JSON;
use core::str::FromStr;
use std::fmt::Display;
use std::panic;

pub mod json {
    use crate::lexer::lexer;
    use crate::parser::parser;
    use crate::parser::JSON;
    use crate::parser::JSON::*;
    use std::fmt::Write;

    pub fn parse(s: String) -> JSON {
        let parts = lexer::lex(&s);
        let (json, _) = parser::parse(parts);
        json
    }
    pub fn stringify(j: JSON, depth: usize) -> String {
        let mut res = String::new();
        match j {
            Str(ref str) => {
                write!(res, "\"{str}\"").unwrap();
                res
            }
            Object(m) => {
                write!(res, "{{\n").unwrap();
                for (i, (k, v)) in m.iter().enumerate() {
                    let newdepth = depth + 2;
                    writeln!(
                        res,
                        "{:newdepth$}\"{}\": {}{}",
                        "",
                        k,
                        stringify(v.clone(), newdepth),
                        if i == m.len() - 1 { "" } else { "," }
                    )
                    .unwrap();
                }
                write!(res, "{:depth$}}}", "").unwrap();
                res
            }
            Array(arr) => {
                write!(res, "[\n").unwrap();
                for (i, v) in arr.iter().enumerate() {
                    let newdepth = depth + 2;
                    writeln!(
                        res,
                        "{:newdepth$}{}{}",
                        "",
                        stringify(v.clone(), newdepth),
                        if i == arr.len() - 1 { "" } else { "," }
                    )
                    .unwrap();
                }
                write!(res, "{:depth$}]", "").unwrap();
                res
            }
            Null => {
                write!(res, "null").unwrap();
                res
            }
            Number(n) => {
                write!(res, "{n}").unwrap();
                res
            }
            Boolean(b) => {
                write!(res, "{b}").unwrap();
                res
            }
        }
    }
}

impl FromStr for JSON {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = panic::catch_unwind(|| json::parse(s.to_string()));
        match res {
            Ok(json) => Ok(json),
            Err(p) => Err(*p.downcast::<String>().unwrap()),
        }
    }
}

impl Display for JSON {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json::stringify(self.clone(), 0))
    }
}
