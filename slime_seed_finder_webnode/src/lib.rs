//#[macro_use]
//extern crate stdweb;
use node_bindgen::core::val::JsCallbackFunction;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::val::JsObject;
use node_bindgen::core::NjError;
use node_bindgen::derive::node_bindgen;
use node_bindgen::sys::napi_value;
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

#[node_bindgen(mt, name = "init")]
//pub fn init<F: Fn(i32, String) + Send + Sync + 'static>(env: JsEnv, console: F) -> Result<napi_value, NjError> {
pub fn init(console: JsCallbackFunction) -> bool {
//pub fn init() -> Result<JsThen<impl Stream<Item = String>, impl FnMut(String)>, NjError> {
    //pub fn init(env: JsEnv, console: Box<dyn Fn(i32, String) + Send + Sync + 'static>) -> Result<napi_value, NjError> {
    // Hopefully enable logging?
    //node_bindgen::core::init_logger();
    //log::debug!("Current logger: {:p}", log::logger());
    node_bindgen_logger::Logger::force_init_with_level(console, ::log::LevelFilter::Debug);

    log::info!("Initialized logger");
    log::debug!("And it works");

    true
}
