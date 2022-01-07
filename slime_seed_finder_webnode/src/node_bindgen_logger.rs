// https://github.com/hobofan/stdweb_logger
//
// Using that crate is impossible because of a conflict between stdweb 0.3
// and stdweb 0.5 when exporting the symbols, there are duplicates like
// "__web_malloc" and "__web_free".
// So I just pasted it here:
//
// But I changed a few thinks, instead of logging to console we log to a
// xtermjs terminal (must have been created before) with the variable
// term2
// And also added color and module info, from the pretty_env_logger crate

use log::{self, Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use node_bindgen::core::val::JsCallbackFunction;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::val::JsObject;
use node_bindgen::core::NjError;
use node_bindgen::derive::node_bindgen;
use node_bindgen::sys::napi_value;
use serde_json::Value;
use std::fmt;
use std::fmt::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

// This struct is from the pretty_env_logger crate
// https://github.com/seanmonstar/pretty-env-logger
struct ColorLevel(Level);

impl ColorLevel {
    // Return the style of the log level that should be applied to the console.log command using
    // the %c modifier
    fn js_style(&self) -> &'static str {
        match self.0 {
            Level::Trace => "color: purple",
            Level::Debug => "color: blue",
            Level::Info => "color: green",
            // At least in firefox, console.warn and console.error have a colored background, so
            // there is no need to paint the level in a different color
            Level::Warn => "",
            Level::Error => "",
        }
    }
}

impl fmt::Display for ColorLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO ",
            Level::Warn => "WARN ",
            Level::Error => "ERROR",
        }
        .fmt(f)
    }
}

static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

pub struct Logger {
    filter: LevelFilter,
    console: Box<dyn Fn(Level, String) + Sync + Send>,
    currently_logging: Mutex<()>,
}

impl Logger {
    /// Returns the maximum `LevelFilter` that this logger instance is
    /// configured to output.
    pub fn filter(&self) -> LevelFilter {
        self.filter
    }

    pub fn try_init_with_level<
        F: Fn(i32, String, &'static str, &'static str, &'static str) + Sync + Send + 'static,
    >(
        console: F,
        level: LevelFilter,
    ) -> Result<(), SetLoggerError> {
        let logger = Self {
            filter: level,
            console: Box::new(move |level: Level, msg: String| {
                eprintln!("Logging message [{}]: {}", level, msg);

                let color_level = ColorLevel(level);
                let format1 = color_level.js_style();
                let format2 = "font-weight: bold";
                let format3 = "";
                let js_function_index = match level {
                    // console.trace prints the stacktrace, we do not want that
                    // console.debug is hidden by default in chrome, so use console.log instead
                    Level::Trace | Level::Debug => 1,
                    Level::Info => 2,
                    Level::Warn => 3,
                    Level::Error => 4,
                };

                //console.call(vec![Value::from(js_function_index), Value::String(msg), Value::String(format1.to_string()), Value::String(format2.to_string()), Value::String(format3.to_string())]).expect("failed to call console");
                console(js_function_index, msg, format1, format2, format3);
            }),
            currently_logging: Mutex::new(()),
        };

        log::set_max_level(logger.filter());
        log::set_boxed_logger(Box::new(logger))
    }

    pub fn init_with_level<
        F: Fn(i32, String, &'static str, &'static str, &'static str) + Sync + Send + 'static,
    >(
        console: F,
        level: LevelFilter,
    ) {
        Self::try_init_with_level(console, level).unwrap();
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter >= metadata.level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // Avoid recursive logging (when there is an error when logging, logging that error may
        // generate another error, which also needs to get logged, and so on)
        let _lock = match self.currently_logging.try_lock() {
            Ok(x) => x,
            Err(_e) => {
                eprintln!("Dropping log message because the previous log call has not completed yet: {:?}", record);
                return;
            }
        };

        // This formatting is from the pretty_env_logger crate
        let target = record.target();
        let mut max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
        if max_width < target.len() {
            MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
            max_width = target.len();
        }
        let color_level = ColorLevel(record.level());
        let mut message = String::new();
        write!(
            &mut message,
            " %c{} %c{: <width$} %c> {}",
            color_level,
            target,
            record.args(),
            width = max_width,
        )
        .unwrap();

        let format1 = color_level.js_style();
        let format2 = "font-weight: bold";
        let format3 = "";
        let js_function_index = match record.level() {
            // console.trace prints the stacktrace, we do not want that
            // console.debug is hidden by default in chrome, so use console.log instead
            Level::Trace | Level::Debug => 1,
            Level::Info => 2,
            Level::Warn => 3,
            Level::Error => 4,
        };

        //let global = self.env.get_global().expect("failed to get global object");
        //dbg!(global);
        //let js_cb = get_console_log_functions(self.env, global).expect("failed to get console log functions");
        //dbg!(js_cb);
        //let args = vec![];
        //dbg!(&args);
        //let console_log = self.console.get_property("log").expect("no log in console").expect("console.log is null");
        //dbg!(console_log);
        //self.env.call_function(global, console_log.napi_value(), args).expect("failed to call console log function");
        (self.console)(record.level(), message)
        // js! { @(no_return)
        //     let console_fn = [console.trace, console.log, console.info, console.warn, console.error][@{js_function_index}];
        //     console_fn(@{message}, @{format1}, @{format2}, @{format3});
        // }
    }

    fn flush(&self) {}
}

use node_bindgen::core::val::JsNapiValue;
use node_bindgen::sys::napi_get_array_length;
use node_bindgen::sys::napi_get_element;
use node_bindgen::sys::napi_get_named_property;
use node_bindgen::sys::napi_get_property_names;
use node_bindgen::sys::napi_status_napi_ok;
use node_bindgen::sys::napi_value__;
use std::ptr;
fn get_console_log_functions(
    js_env: JsEnv,
    global: *mut napi_value__,
) -> Result<*mut napi_value__, NjError> {
    let env = js_env.inner();
    let mut console = ptr::null_mut();
    let status = unsafe {
        napi_get_named_property(
            env,
            global,
            b"console" as *const u8 as *const i8,
            &mut console,
        )
    };

    if status != napi_status_napi_ok {
        panic!("todo: return error");
    }

    eprintln!("console: {:?}", console);

    let mut console_props = ptr::null_mut();
    let status = unsafe { napi_get_property_names(env, console, &mut console_props) };

    if status != napi_status_napi_ok {
        panic!("todo: return error3");
    }

    eprint_properties(js_env, console_props);

    let mut console_log = ptr::null_mut();
    let status = unsafe {
        napi_get_named_property(
            env,
            console,
            b"log" as *const u8 as *const i8,
            &mut console_log,
        )
    };

    if status != napi_status_napi_ok {
        panic!("todo: return error2");
    }

    Ok(console_log)
}

fn eprint_properties(js_env: JsEnv, console_props: *mut napi_value__) {
    let env = js_env.inner();
    let mut arr_len = 0;
    let status = unsafe { napi_get_array_length(env, console_props, &mut arr_len) };

    if status != napi_status_napi_ok {
        panic!("todo: return error4");
    }

    eprintln!("number of properties: {}", unsafe { arr_len });

    for i in 0..arr_len {
        let mut res = ptr::null_mut();
        let status = unsafe { napi_get_element(env, console_props, i, &mut res) };

        if status != napi_status_napi_ok {
            panic!("todo: return error5 {}", i);
        }

        let str_object = JsObject::new(js_env, res);
        eprintln!("* {:?}", str_object.as_value::<String>());
    }
}
