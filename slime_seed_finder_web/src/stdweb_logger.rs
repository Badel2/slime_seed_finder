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
use std::fmt;
use std::fmt::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use stdweb::js;

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
}

impl Logger {
    /// Returns the maximum `LevelFilter` that this logger instance is
    /// configured to output.
    pub fn filter(&self) -> LevelFilter {
        self.filter
    }

    pub fn try_init_with_level(level: LevelFilter) -> Result<(), SetLoggerError> {
        let logger = Self { filter: level };

        log::set_max_level(logger.filter());
        log::set_boxed_logger(Box::new(logger))
    }

    pub fn init_with_level(level: LevelFilter) {
        Self::try_init_with_level(level).unwrap();
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

        js! { @(no_return)
            let console_fn = [console.trace, console.log, console.info, console.warn, console.error][@{js_function_index}];
            console_fn(@{message}, @{format1}, @{format2}, @{format3});
        }
    }

    fn flush(&self) {}
}
