#![no_std]
#![no_main]
#![feature(asm)]

extern crate discovery_stm32f3 as board;

// LED0 = PE9;

use board::mcu::pin::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    PE9.port().gate_enable();

    // Set PB3 Mode = Output
    PE9.mode_output();

    loop {
        PE9.toggle_output();
        // Delay approx 1/2 second
        for _ in 0..4_000_000 { unsafe { asm!("nop") } } 
    }
}