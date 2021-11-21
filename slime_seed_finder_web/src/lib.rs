use wasm_bindgen::prelude::*;

pub mod wasm_gui;

#[wasm_bindgen(start)]
pub fn init() {
    wasm_logger::init(wasm_logger::Config::default());

    // Logging
    log::debug!("Logger enabled, hello from Rust!");
}
