use std::panic;
use wasm_bindgen::prelude::*;

pub mod wasm_gui;

// Import progress update function
#[wasm_bindgen]
extern "C" {
    // TODO: currently this only works if called from the worker_generic.js context,
    // it will break if called from other workers, or from the main thread (without using workers)
    pub fn post_progress_message(s: &str);
}

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
