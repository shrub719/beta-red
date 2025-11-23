use std::io;
use wasm_bindgen::prelude::*;

mod lexer;
mod parser;
mod evaluator;
mod errors;

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, errors::ParserError> {
    let tokens = lexer::lex(&mut input.chars())?;
    let expr = parser::parse(&tokens)?;

    let js_value = match serde_wasm_bindgen::to_value(&expr) {
        Ok(val) => val,
        Err(_) => return Err(errors::ParserError::CannotConvert)
    };
    Ok(js_value)
}

#[wasm_bindgen]
pub fn print(input: JsValue) -> Result<String, errors::ParserError> {
    let expr: parser::Term = match serde_wasm_bindgen::from_value(input) {
        Ok(res) => res,
        Err(_) => return Err(errors::ParserError::CannotConvert)
    };

    Ok(expr.to_string())
}

#[wasm_bindgen]
pub fn evaluate(input: JsValue) -> Result<JsValue, errors::ParserError> {
    let expr: parser::Term = match serde_wasm_bindgen::from_value(input) {
        Ok(res) => res,
        Err(_) => return Err(errors::ParserError::CannotConvert)
    };
    
    let reduced = evaluator::evaluate(expr);
    
    let js_value = match serde_wasm_bindgen::to_value(&reduced) {
        Ok(val) => val,
        Err(_) => return Err(errors::ParserError::CannotConvert)
    };
    Ok(js_value)
}

#[allow(dead_code)]
fn main() {
    loop {
        let mut buf = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buf).expect("could not read from stdin");

        if buf.starts_with("quit") {
            break;
        }
            
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
        let reduced = evaluator::reduce(expr);

        println!("-> {}", reduced);
    }
}
