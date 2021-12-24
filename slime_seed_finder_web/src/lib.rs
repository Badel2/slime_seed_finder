use std::panic;
use wasm_bindgen::prelude::*;

pub mod wasm_gui;

#[wasm_bindgen(start)]
pub fn init() {
    wasm_logger::init(wasm_logger::Config::default());

    // Set panic hook so we get backtrace in console
    let next_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        log::error!("PANIC: {}", &info.to_string());
        next_hook(info);
    }));

    // Logging
    log::debug!("Logger enabled, hello from Rust!");
}
