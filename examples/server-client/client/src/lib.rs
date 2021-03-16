#[macro_use]
mod console;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "handleMessage")]
pub fn handle_message(data: Vec<u8>) -> Vec<u8> {
    vec![1, 2, 3]
}
