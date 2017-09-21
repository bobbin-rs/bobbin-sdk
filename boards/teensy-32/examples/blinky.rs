#![no_std]
#![no_main]

extern crate teensy_32 as board;

use board::led::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    loop {
        LED0.toggle_output();
        board::delay(500);
    }
}
