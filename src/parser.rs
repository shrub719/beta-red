use serde::{
    Serialize,
    Deserialize
};
use crate::lexer::Token;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn parse(tokens: Vec<Token>) -> Result<Term, ()> {
    Ok(Term::app(
        Term::abs("x", Term::id("x")),
        Term::id("a")
    ))
}
