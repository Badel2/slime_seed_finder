//#[macro_use]
//extern crate stdweb;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::encode::json::JsonEncoder;
use log4rs::Config;
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
pub fn init(log_dir_path: String) -> bool {
    // Set panic hook so we get backtrace in console
    let next_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        log::error!("PANIC: {}", &info.to_string());
        next_hook(info);
    }));

    // TODO: use Path::push instead of format
    let log_file_path = format!("{}/log_pid_{}.txt", log_dir_path, std::process::id());

    // Enable logging
    let logfile = FileAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build(&log_file_path)
        .unwrap();

    // Log Trace level output to file
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config).expect("failed to init logger");

    true
}
