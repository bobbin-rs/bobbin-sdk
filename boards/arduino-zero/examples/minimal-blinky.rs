#![no_std]
#![no_main]
#![feature(asm)]

extern crate arduino_zero as board;

use board::mcu::port::*;

// LED0 = PA17

#[no_mangle]
pub extern "C" fn main() -> ! {    
    let _ = board::init();  

    // Set PA17 DIR = OUTPUT
    PORTA.with_dir(|r| r.set_dir(17, 1));
    loop {
        // Toggle PA17 Output
        PORTA.set_outtgl(|r| r.set_outtgl(17, 1));
        // Delay approx 1/2 second
        for _ in 0..200_000 { unsafe { asm!("nop") } }
    }
}
