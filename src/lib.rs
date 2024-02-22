use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn hello() {
    console::log_1(&"Hello from Rust!".into());
}