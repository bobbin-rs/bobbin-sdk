#![no_std]
#![no_main]

extern crate audio_node as board;

use board::hal::gpio::*;
use board::led::LED0;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    loop {
        LED0.toggle_output();
        board::delay(500);
    }
}

