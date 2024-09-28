pub mod engine;

use regex_syntax::parse;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate(pattern: String) -> String{
    let hir = parse(pattern.as_str()).unwrap();

    return engine::generate(hir);
}

#[wasm_bindgen]
pub fn hello(pattern: String) -> String{
    return format!("LOLO: {}", pattern);
}