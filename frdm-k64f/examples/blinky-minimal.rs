#![no_std]
#![no_main]
#![feature(asm)]
extern crate frdm_k64f as board;

use board::led::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    init();        
    loop {
        LED0.toggle_output();
        for _ in 0..2_000_000 {
            unsafe { asm!("nop"); }
        }
    }
}
