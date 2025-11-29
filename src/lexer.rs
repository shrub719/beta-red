use std::str::Chars;
use crate::errors::ParserError;

pub enum Token {
    Lambda(usize),
    LParen(usize),
    RParen(usize),
    #[allow(dead_code)]
    Identifier(usize, String)
}

pub fn lex(input: &mut Chars) -> Result<Vec<Token>, ParserError> {
    let mut tokens = Vec::new();
    let mut pos: usize = 0;
    let mut running_id = String::new();

    while let Some(ch) = input.next() {
        pos += 1;

        let next_token = match ch {
            'λ' | '\\' => Some(Token::Lambda(pos)),
            '(' => Some(Token::LParen(pos)),
            ')' => Some(Token::RParen(pos)),
            l if l == 'L' && running_id.is_empty() => Some(Token::Lambda(pos)),
            c if c.is_alphanumeric() || c == '_' => {
                running_id.push(c);
                continue;
            },
            c if c.is_whitespace() || c == '.' => None,
            c => return Err(ParserError::InvalidCharacter(pos, c))
        };

        if !running_id.is_empty() {
            tokens.push(Token::Identifier(
                pos - running_id.len(),
                running_id.clone(),
            ));
            running_id.clear();
        }

        if let Some(token) = next_token {
            tokens.push(token);
        }
    }

    Ok(tokens)
}
