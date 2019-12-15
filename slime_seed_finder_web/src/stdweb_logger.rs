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

use ansi_term::{Color, Style};
use log::{self, Log, LevelFilter, Level, Record, SetLoggerError, Metadata};
use stdweb::js;
use std::fmt::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt;

// This struct is from the pretty_env_logger crate
// https://github.com/seanmonstar/pretty-env-logger
struct ColorLevel(Level);

impl fmt::Display for ColorLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Level::Trace => Color::Purple.paint("TRACE"),
            Level::Debug => Color::Blue.paint("DEBUG"),
            Level::Info => Color::Green.paint("INFO "),
            Level::Warn => Color::Yellow.paint("WARN "),
            Level::Error => Color::Red.paint("ERROR")
        }.fmt(f)
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
        let logger = Self {
            filter: level,
        };

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
        let mut message = String::new();
        write!(&mut message, " {} {} > {}",
               ColorLevel(record.level()),
               Style::new().bold().paint(format!("{: <width$}", target, width=max_width)),
               record.args()
        ).unwrap();

        js!{
            console.debug(@{message});
        }
    }

    fn flush(&self) {}
}

