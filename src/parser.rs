use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
use serde::{
    Serialize,
    Deserialize
};
use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Term {
    Abstraction {
        param: String,
        body: Box<Term>,
    },
    Application {
        func: Box<Term>,
        arg: Box<Term>
    },
    Identifier(String)
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ParserError {
    NoToken,
    UnexpectedToken,
    NoAtom,
    Error
}

impl Term {
    pub fn abs(param: impl Into<String>, body: Term) -> Self {
        Term::Abstraction {
            param: param.into(),
            body: Box::new(body),
        }
    }

    pub fn app(func: Term, arg: Term) -> Self {
        Term::Application {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }

    pub fn id(name: impl Into<String>) -> Self {
        Term::Identifier(name.into())
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: VecDeque<Token>
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self {
            tokens
        }
    }

    fn consume_if(&mut self, token: Token) -> bool {
        match self.tokens.front() {
            Some(front) if *front == token => {
                self.tokens.pop_front();
                true
            },
            _ => false
        }
    }

    fn consume_expect(&mut self, token: Token) -> Result<Token, ParserError> {
        let Some(front) = self.tokens.front() else {
            return Err(ParserError::NoToken)
        };
        if *front == token {
            let Some(front) = self.tokens.pop_front() else {
                return Err(ParserError::NoToken)
            };
            return Ok(front)
        } else {
            return Err(ParserError::UnexpectedToken)
        }
    }

    fn consume_identifier(&mut self) -> Result<String, ParserError> {
        let Some(front) = self.tokens.front() else {
            return Err(ParserError::NoToken)
        };
        let name = match front {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParserError::UnexpectedToken)
        };
        self.tokens.pop_front();
        Ok(name)
    }

    fn is_identifier(&mut self) -> bool {
        match self.tokens.front() {
            Some(Token::Identifier(_)) => true,
            _ => false
        }
    }

    fn term(&mut self) -> Result<Term, ParserError> {
        if self.consume_if(Token::Lambda) {
            let param = self.consume_identifier()?;
            self.consume_expect(Token::Dot)?;
            let body = self.term()?;
            return Ok(Term::abs(param, body))
        } else {
            return Ok(self.application()?)
        }
    }


    fn application(&mut self) -> Result<Term, ParserError> {
        let Some(mut lhs) = self.atom()? else {
            return Err(ParserError::NoAtom)
        };
        loop {
            let Some(rhs) = self.atom()? else {
                return Ok(lhs)
            };
            lhs = Term::app(lhs, rhs);
        }
    }

    fn atom(&mut self) -> Result<Option<Term>, ParserError> {
        if self.consume_if(Token::LParen) {
            let term = self.term()?;
            self.consume_expect(Token::RParen)?;
            return Ok(Some(term))
        } else if self.is_identifier() {
            let name = self.consume_identifier()?;
            return Ok(Some(Term::id(name)))
        } else {
            return Ok(None)
        }
    }

    pub fn parse(&mut self) -> Result<Term, ParserError> {
        Ok(self.term()?)
    }
}

pub fn parse(tokens: VecDeque<Token>) -> Result<Term, ParserError> {
    let mut parser = Parser::new(tokens);   // TODO: must be mutable?
    parser.parse()
}

