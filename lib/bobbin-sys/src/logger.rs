use log::{self, Record, Level, Metadata};
pub use log::LevelFilter;

pub const LOGGER: Logger = Logger {};

#[derive(Default)]
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

impl Logger {
    pub fn init() {
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Info);    
    }

    pub fn set_max_level(level_filter: log::LevelFilter) {
        log::set_max_level(level_filter)
    }

    pub fn set_max_level_off(&self) {
        log::set_max_level(log::LevelFilter::Off);
    }

    pub fn set_max_level_error(&self) {
        log::set_max_level(log::LevelFilter::Error);
    }

    pub fn set_max_level_warn(&self) {
        log::set_max_level(log::LevelFilter::Warn);
    }

    pub fn set_max_level_info(&self) {
        log::set_max_level(log::LevelFilter::Info);
    }

    pub fn set_max_level_debug(&self) {
        log::set_max_level(log::LevelFilter::Debug);
    }

    pub fn set_max_level_trace(&self) {
        log::set_max_level(log::LevelFilter::Trace);
    }

}

