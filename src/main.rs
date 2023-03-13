use std::fs;

mod json;
mod lexer;
mod parser;
use crate::parser::JSON;

fn main() {
    let res: Result<JSON, String> = fs::read_to_string("src/test.json").unwrap().parse();
    match res {
        Ok(j) => {
            println!("Success!\n======\n{}", j);
        }
        Err(e) => {
            println!("Error!\n======\n{}", e);
        }
    }
}
