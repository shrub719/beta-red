use std::fmt;
use serde::{
    Serialize,
    Deserialize
};
use crate::{
    errors::ParserError,
    lexer::Token
};

#[derive(Clone, Serialize, Deserialize)]
pub enum Term {
    App(Box<Term>, Box<Term>),
    Abs(String, Box<Term>),
    Var(String)
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::App(left, right) => {
                match left.as_ref() {
                    Term::Abs(_, _) => write!(f, "({}) ", left),
                    _ => write!(f, "{} ", left) 
                }?;
                match right.as_ref() {
                    Term::App(_, _) => write!(f, "({})", right),
                    Term::Abs(_, _) => write!(f, "({})", right),
                    _ => write!(f, "{}", right)
                }
            }
            Term::Abs(param, body) => write!(f, "λ{}.{}", param, body),
            Term::Var(name) => write!(f, "{}", name)
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<Term, ParserError> {
    let mut terms = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::LParen(pos) => {
                let mut open_parens = 0;
                let mut j = i + 1;
                let mut pushed = false;

                while j < tokens.len() {
                    match tokens[j] {
                        Token::LParen(_) => open_parens += 1,
                        Token::RParen(_) => {
                            if open_parens == 0 {
                                let inside = parse(&tokens[i+1..=j-1])?; // what
                                terms.push(inside);
                                pushed = true;
                                break;
                            } else {
                                open_parens -= 1;
                            }
                        },
                        _ => ()
                    };
                    
                    j += 1;
                }

                if !pushed {
                    return Err(ParserError::UnmatchedLParen(*pos));
                }

                i = j;
            },

            Token::RParen(pos) => return Err(ParserError::UnmatchedRParen(*pos)),

            Token::Lambda(pos) => {
                if tokens.len() <= i+2 {
                    return Err(ParserError::EmptyFunctionBody(*pos));
                }

                if let Some(Token::Identifier(_, name)) = tokens.get(i+1) {
                    let body = parse(&tokens[i+2..])?;
                    terms.push(Term::Abs(name.to_string(), Box::new(body)));
                    i = tokens.len();
                } else {
                    return Err(ParserError::EmptyFunctionParam(*pos));
                }
            },

            Token::Identifier(_, name) => {
                terms.push(Term::Var(name.to_string()));
            }
        };

        i += 1;
    }

    match terms.into_iter().reduce(|left, right| Term::App(Box::new(left), Box::new(right))) {
        Some(ast) => Ok(ast),
        None => Err(ParserError::Empty)
    }
}
