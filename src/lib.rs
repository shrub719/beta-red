use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn upper(string: &str) -> String {
    let string_upper = string.to_ascii_uppercase();
    string_upper
}
