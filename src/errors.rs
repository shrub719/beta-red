use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum ParserError {
    InvalidCharacter(usize, char),
    Empty,
    UnmatchedLParen(usize),
    UnmatchedRParen(usize),
    EmptyFunctionParam(usize),
    EmptyFunctionBody(usize),
    CannotConvert
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParserError::*;

        match self {
            InvalidCharacter(pos, ch) => write!(f, "invalid character at position {}: {}", pos, ch),
            Empty => write!(f, "empty expression"),
            UnmatchedRParen(pos) => write!(f, "unmatched closing parenthesis at position {}", pos),
            UnmatchedLParen(pos) => write!(f, "unmatched opening parenthesis at position {}", pos),
            EmptyFunctionParam(pos) => write!(f, "empty lambda parameter at position {}", pos),
            EmptyFunctionBody(pos) => write!(f, "empty lambda body at position {}", pos),
            CannotConvert => write!(f, "cannot convert expression to JsValue")
        }
    }
}

impl From<ParserError> for JsValue {
    fn from(error: ParserError) -> Self {
        js_sys::Error::new(&error.to_string()).into()
    }
}
