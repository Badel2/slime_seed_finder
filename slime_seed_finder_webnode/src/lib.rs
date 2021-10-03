//#[macro_use]
//extern crate stdweb;
use node_bindgen::core::val::JsCallbackFunction;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::val::JsObject;
use node_bindgen::core::NjError;
use node_bindgen::derive::node_bindgen;
use node_bindgen::sys::napi_value;
use serde_json::Value;
use std::panic;

mod node_bindgen_logger;
pub mod wasm_gui;
pub use wasm_gui::*;

////use stdweb::print_error_panic;
//// https://github.com/koute/stdweb/blob/4d337ee9a0a4542ea5803b46b5124d9bc166dcb7/src/webcore/promise_future.rs#L127
///// Prints an error to the console and then panics.
/////
///// If you're using Futures, it's more convenient to use [`unwrap_future`](fn.unwrap_future.html) instead.
/////
///// # Panics
///// This function *always* panics.
//#[inline]
//pub fn print_error_panic<A: stdweb::JsSerialize>(value: A) -> ! {
//    js! { @(no_return)
//        console.error( @{value} );
//    }
//    panic!();
//}
//
///// initialize
//pub fn init() {
//    // Set panic hook so we get backtrace in console
//    panic::set_hook(Box::new(|info| {
//        print_error_panic(&info.to_string());
//    }));
//    // Init console logger
//    // TODO: provide some way to change the log level
//    // It cannot be changed at runtime because of limitations of the log crate
//    stdweb_logger::Logger::init_with_level(::log::LevelFilter::Debug);
//    // Don't start, wait for user to press button
//}

#[node_bindgen(name = "init")]
pub fn init<F: Fn(Value) + Send + Sync + 'static>(console: F) -> bool {
    // Enable logging
    node_bindgen_logger::Logger::init_with_level(move |level, msg, fmt1, fmt2, fmt3| {
        // TODO: node_bindgen breaks if F has more than one argument, so here we serialize the
        // arguments into a serde_json array, and in javascript we can simply do
        // console.log(...args)
        let args = Value::Array(vec![
            Value::from(level),
            Value::String(msg),
            Value::String(fmt1.to_string()),
            Value::String(fmt2.to_string()),
            Value::String(fmt3.to_string()),
        ]);
        console(args)
    }, ::log::LevelFilter::Debug);

    log::info!("Initialized logger");

    true
}
