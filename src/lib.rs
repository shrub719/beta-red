use wasm_bindgen::prelude::*;

mod lexer;
mod parser;

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
    let tokens = lexer::lex(input)?;
    let expr = parser::parse(tokens).unwrap();

    Ok(serde_wasm_bindgen::to_value(&expr)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_true() {
        use lexer::Token::*;

        let input = "\\x.\\y.x";
        let tokens = lexer::lex(input).unwrap();

        assert_eq!(tokens, vec![
            Lambda, 
            Identifier("x".to_string()), 
            Dot, 
            Lambda, 
            Identifier("y".to_string()), 
            Dot, 
            Identifier("x".to_string())
        ]);
    }

    #[test]
    fn lex_app() {
        use lexer::Token::*;

        let input = "(\\x.x) _3";
        let tokens = lexer::lex(input).unwrap();

        assert_eq!(tokens, vec![
            LParen,
            Lambda,
            Identifier("x".to_string()),
            Dot,
            Identifier("x".to_string()),
            RParen,
            Identifier("_3".to_string())
        ]);
    }
}

