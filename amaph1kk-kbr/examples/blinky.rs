#![no_std]
#![no_main]
#![feature(asm)]

extern crate amaph1kk_kbr as board;

use board::led::{LED0, DigitalOutput};


// LED0 = GPIO17

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    pub const DELAY: usize = 4_000_000;

    loop {
        LED0.toggle_output();
        unsafe {
            for _ in 0..DELAY { asm!("nop") }
        }
    }    
}
