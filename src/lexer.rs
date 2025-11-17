use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Token {
    Lambda,
    Dot,
    LParen,
    RParen,
    Identifier(String)
}

pub fn lex(input: &str) -> Result<Vec<Token>, ()> {
    Ok(vec![
        Token::Lambda,
        Token::Identifier("x".to_string()),
        Token::Dot,
        Token::Identifier("y".to_string()),
    ])
}
