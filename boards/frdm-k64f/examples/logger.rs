#![no_std]
#![no_main]
#![feature(asm, use_extern_macros)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate frdm_k64f as board;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {  
    let sys = board::init();
    println!("Running...");
    
    sys.logger().set_max_level_info();
    println!("---");
    println!("Max Level Info");
    println!("---");

    error!("Error message");
    warn!("Warning message");
    info!("Info message");    
    debug!("Debug message");
    trace!("Trace message");
    println!("");

    sys.logger().set_max_level_trace();
    println!("---");
    println!("Max Level Trace");
    println!("---");

    error!("Error message");
    warn!("Warning message");
    info!("Info message");    
    debug!("Debug message");
    trace!("Trace message");


    loop {}
}
