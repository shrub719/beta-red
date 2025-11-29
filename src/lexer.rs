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
        let mut next_token = None;

        match ch {
            'λ' | '\\' => next_token = Some(Token::Lambda(pos)),
            '(' => next_token = Some(Token::LParen(pos)),
            ')' => next_token = Some(Token::RParen(pos)),
            l if l == 'L' && running_id.is_empty() => next_token = Some(Token::Lambda(pos)),
            c if c.is_alphanumeric() || c == '_' => {
                running_id.push(c);
                continue;
            },
            c if c.is_whitespace() || c == '.' => (),
            c => return Err(ParserError::InvalidCharacter(pos, c))
        };

        if running_id.len() > 0 {
            tokens.push(Token::Identifier(
                pos - running_id.len(),
                running_id.clone(),
            ));
            running_id.clear();
        }

        match next_token {
            Some(token) => tokens.push(token),
            _ => ()
        };
    }

    Ok(tokens)
}
