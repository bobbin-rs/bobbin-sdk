#![no_std]
#![no_main]
#![feature(asm)]

extern crate frdm_k64f as board;

use board::mcu::sim::*;
use board::mcu::port::*;
use board::mcu::gpio::*;

// FRDM-K64F RED LED = PTB22

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();

    // Enable PORTB
    SIM.with_scgc5(|r| r.set_portb(1));
    // Set PTB22 to GPIO
    PORTB.set_pcr(22, |r| r.set_mux(0b001));
    // Set PTB22 to Output
    GPIOB.set_pddr(|r| r.set_pdd(22, 1));

    loop {
        // Toggle PTB22
        GPIOB.set_ptor(|r| r.set_ptto(22, 1));
        for _ in 0..8_000_000 {
            unsafe { asm!("nop") }
        }
    }
}