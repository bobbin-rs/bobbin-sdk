use log::{self, Record, Level, Metadata, LevelFilter};
use common::{println, print};
pub const LOGGER: Logger = Logger {};

pub struct Logger {}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        println!("{} - {}", record.level(), record.args());
    }        
    fn flush(&self) {}
}

pub fn set_log_level(level: LevelFilter) {
    log::set_max_level(level)

}