use wasm_bindgen::prelude::*;

mod lexer;
mod errors;

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<String, errors::ParserError> {
    let mut input_chars = input.chars();
    lexer::lex(&mut input_chars)?;
    Ok(input.into())
}
