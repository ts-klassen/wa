use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen]
pub fn is_match(pattern: &str, subject: &str) -> bool {
    let re = Regex::new(pattern).unwrap();
    re.is_match(subject)
}

