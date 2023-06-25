use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::*;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

pub struct Logger;
impl Logger {
    fn now_format<'a>() -> DelayedFormat<StrftimeItems<'a>> {
        let now = Local::now();
        now.format("%Y-%m-%d %H:%M:%S%.3f")
    }

    pub fn log(level: LogLevel, message: &str) {
        let now = Logger::now_format();
        println!("{} {:?}: {}", now, level, message)
    }

    pub fn debug(message: &str) {
        Logger::log(LogLevel::Debug, message)
    }

    pub fn info(message: &str) {
        Logger::log(LogLevel::Info, message)
    }
}
