#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_l432kc as board;

// LED0 = PB3;

use board::mcu::pin::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    PB3.port().gate_enable();

    // Set PB3 Mode = Output
    PB3.mode_output();

    loop {
        PB3.toggle_output();
        // Delay approx 1/2 second
        for _ in 0..1_000_000 { unsafe { asm!("nop") } } 
    }
}