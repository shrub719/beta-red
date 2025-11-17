use wasm_bindgen::prelude::*;

mod lexer;
mod parser;

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
    let tokens = lexer::lex(input)?;
    let expr = parser::parse(tokens).unwrap();

    Ok(serde_wasm_bindgen::to_value(&expr)?)
}

