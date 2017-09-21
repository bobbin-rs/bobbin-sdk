#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_l432kc as board;

use board::hal::gpio::*;
use board::led::LED0;

#[no_mangle]
pub extern "C" fn main() -> ! {    
    board::led::init();    
    loop {
        LED0.toggle_output();
        for _ in 0..1_000_000 {
            unsafe { asm!("nop") }
        }
    }
}

