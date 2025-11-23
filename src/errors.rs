use std::fmt;
use wasm_bindgen::prelude::*;

pub enum ParserError {
    InvalidCharacter(usize, char),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParserError::*;

        match self {
            InvalidCharacter(pos, ch) => write!(f, "invalid character at position {}: {}", pos, ch)
        }
    }
}

impl From<ParserError> for JsValue {
    fn from(error: ParserError) -> Self {
        js_sys::Error::new(&error.to_string()).into()
    }
}
