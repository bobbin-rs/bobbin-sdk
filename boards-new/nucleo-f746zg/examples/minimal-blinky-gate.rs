#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;

// LED0 = PB0;

use board::mcu::pin::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    PB0.port().gate_enable();

    // Set PB3 Mode = Output
    PB0.mode_output();

    loop {
        PB0.toggle_output();
        // Delay approx 1/2 second
        for _ in 0..4_000_000 { unsafe { asm!("nop") } } 
    }
}