use std::io;
use wasm_bindgen::prelude::*;

mod lexer;
mod parser;
mod errors;

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<String, errors::ParserError> {
    lexer::lex(&mut input.chars())?;
    Ok(input.into())
}

fn main() {
    loop {
        let mut buf = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buf).expect("could not read from stdin");
            
        let tokens = match lexer::lex(&mut buf.chars()) {
            Ok(res) => res,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        let expr = match parser::parse(&tokens) {
            Ok(res) => res,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        println!("-> {}", expr);
    }
}
