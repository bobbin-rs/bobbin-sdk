#![no_std]
#![no_main]
#![feature(asm)]

extern crate discovery_stm32f429i as board;

// LED0 = PG13;

use board::mcu::pin::*;
use board::prelude::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    PG13.port().gate_enable();

    // Set PG13 Mode = Output
    PG13.mode_output();

    loop {
        PG13.toggle_output();
        // Delay approx 1/2 second
        for _ in 0..4_000_000 { unsafe { asm!("nop") } } 
    }
}