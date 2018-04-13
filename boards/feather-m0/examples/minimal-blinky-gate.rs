#![no_std]
#![no_main]
#![feature(asm)]

extern crate feather_m0 as board;

use board::common::gate::*;
use board::mcu::pin::*;

// LED0 = PA17

#[no_mangle]
pub extern "C" fn main() -> ! {  
    board::init();  
    
    // Set PA17 DIR = OUTPUT
    PA17.port().gate_enable();
    PA17.set_mode_output();
    loop {
        // Toggle PA17 Output
        PA17.toggle_output();
        // Delay approx 1/2 second
        for _ in 0..8_000_000 { unsafe { asm!("nop") } }
    }
}
