use wasm_bindgen::prelude::*;

mod lexer;
mod parser;
mod evaluator;

#[cfg(test)]
mod tests;

// um?? input types??

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
    let tokens = lexer::lex(input)?;
    let expr = parser::parse(tokens)?;

    Ok(serde_wasm_bindgen::to_value(&expr)?)
}

#[wasm_bindgen]
pub fn print(input: JsValue) -> Result<String, JsValue> {
    let expr: parser::Term = serde_wasm_bindgen::from_value(input)?;
    Ok(expr.to_string())
}
