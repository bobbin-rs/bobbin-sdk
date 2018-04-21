#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate nucleo_f302r8 as board;

use board::logger;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {  
    board::init();
    logger::set_log_level(log::LogLevelFilter::Info).unwrap();
    println!("Running...");
    error!("Error message");
    warn!("Warning message");
    info!("Info message");    
    debug!("Debug message");
    trace!("Trace message");
    loop {}
}
