use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
use serde::{
    Serialize,
    Deserialize
};
use crate::lexer::Token;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Term {
    Abstraction {
        param: Box<Term>,   // TODO: this should only be Identifier if possible
        body: Box<Term>,
    },
    Application {
        func: Box<Term>,
        arg: Box<Term>
    },
    Identifier(String)
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize)]
pub enum ParserError {
    Error
}

impl Term {
    pub fn abs(param: Term, body: Term) -> Self {
        Term::Abstraction {
            param: Box::new(param),
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

    fn term(&mut self) -> Result<Term, ParserError> {
        Ok(Term::id("x"))
    }

    pub fn parse(&mut self) -> Result<Term, ParserError> {
        Ok(self.term()?)
    }
}

pub fn parse(tokens: VecDeque<Token>) -> Result<Term, ParserError> {
    let mut parser = Parser::new(tokens);   // TODO: must be mutable?
    parser.parse()
}

