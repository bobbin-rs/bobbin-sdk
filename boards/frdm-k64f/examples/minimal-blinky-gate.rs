#![no_std]
#![no_main]
#![feature(asm)]

extern crate frdm_k64f as board;

use board::common::gate::*;
use board::common::digital::DigitalOutput;
use board::mcu::port::*;
use board::mcu::pin::*;


// FRDM-K64F RED LED = PTB22

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();

    // Enable PORTB
    PORTB.gate_enable();
    // Set PTB22 to GPIO
    PTB22.set_mux_gpio();
    // Set PTB22 to Output
    PB22.set_dir_output();
    loop {
        // Toggle PTB22
        PB22.toggle_output();
        for _ in 0..8_000_000 {
            unsafe { asm!("nop") }
        }
    }
}