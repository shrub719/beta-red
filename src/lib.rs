use wasm_bindgen::prelude::*;

mod lexer;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let mut input_chars = input.chars();
    lexer::lex(&mut input_chars);
    input.into()
}
