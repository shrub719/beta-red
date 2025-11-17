use wasm_bindgen::prelude::*;
use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    Lambda,
    Dot,
    LParen,
    RParen,
    Identifier(String)
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize)]
pub enum LexerError {
    InvalidCharacter,   // TODO: add invalid char
}

#[derive(Debug)]
struct Lexer<'input> {
    chars: std::iter::Peekable<std::str::Chars<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.skip_whitespace();
        
        let Some(ch) = self.chars.next() else {
            return Ok(None);
        };
        
        
        let token = match ch {
            'λ' | '\\' => Token::Lambda,
            '.' => Token::Dot,
            '(' => Token::LParen,
            ')' => Token::RParen,
            c if c.is_alphabetic() || c == '_' => {
                let id = self.lex_identifier(c);
                Token::Identifier(id)
            }
            _ => return Err(LexerError::InvalidCharacter)
        };

        Ok(Some(token))
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn lex_identifier(&mut self, start: char) -> String {
        let mut id = start.to_string();
        
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                id.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        
        id
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.next_token() {
                Ok(Some(token)) => return Some(Ok(token)),
                Ok(None) => return None,
                Err(e) => return Some(Err(e))
            }
        }
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
    let lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();

    for token in lexer {
        tokens.push(token?);
    }

    Ok(tokens)
}
