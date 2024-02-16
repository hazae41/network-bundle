extern crate alloc;

use alloc::string::String;

use wasm_bindgen::prelude::*;

use crate::Memory;

#[wasm_bindgen]
pub fn base16_encode_lower(bytes: &Memory) -> String {
    base16ct::lower::encode_string(&bytes.inner)
}

#[wasm_bindgen]
pub fn base16_encode_upper(bytes: &Memory) -> String {
    base16ct::upper::encode_string(&bytes.inner)
}

#[wasm_bindgen]
pub fn base16_decode_mixed(text: &str) -> Result<Memory, JsError> {
    base16ct::mixed::decode_vec(text)
        .map(Memory::new)
        .map_err(|_| JsError::new("base16_decode_mixed"))
}

#[wasm_bindgen]
pub fn base16_decode_lower(text: &str) -> Result<Memory, JsError> {
    base16ct::lower::decode_vec(text)
        .map(Memory::new)
        .map_err(|_| JsError::new("base16_decode_lower"))
}

#[wasm_bindgen]
pub fn base16_decode_upper(text: &str) -> Result<Memory, JsError> {
    base16ct::upper::decode_vec(text)
        .map(Memory::new)
        .map_err(|_| JsError::new("base16_decode_upper"))
}
