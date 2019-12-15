#[macro_use]
extern crate stdweb;
use std::panic;
use stdweb::js;

mod stdweb_logger;
pub mod wasm_gui;

//use stdweb::print_error_panic;
// https://github.com/koute/stdweb/blob/4d337ee9a0a4542ea5803b46b5124d9bc166dcb7/src/webcore/promise_future.rs#L127
/// Prints an error to the console and then panics.
///
/// If you're using Futures, it's more convenient to use [`unwrap_future`](fn.unwrap_future.html) instead.
///
/// # Panics
/// This function *always* panics.
#[inline]
pub fn print_error_panic<A: stdweb::JsSerialize>(value: A) -> ! {
    js! { @(no_return)
        console.error( @{value} );
    }
    panic!();
}

fn main() {
    // Set panic hook so we get backtrace in console
    panic::set_hook(Box::new(|info| {
        print_error_panic(&info.to_string());
    }));
    // Init console logger
    stdweb_logger::Logger::init_with_level(::log::LevelFilter::Debug);
    // Don't start, wait for user to press button
}
