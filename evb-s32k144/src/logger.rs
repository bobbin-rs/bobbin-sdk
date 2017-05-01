use log::{self, LogRecord, LogLevel, LogMetadata, SetLoggerError, LogLevelFilter};

pub const LOGGER: Logger = Logger {};

pub struct Logger {}

impl log::Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }
    fn log(&self, record: &LogRecord) {
        println!("{} - {}", record.level(), record.args());
    }        
}

pub fn set_log_level(level: LogLevelFilter) -> Result<(), SetLoggerError> {
    unsafe {
        log::set_logger_raw(|max_log_level| {            
            max_log_level.set(level);
            &LOGGER
        })
    }
}