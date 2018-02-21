#![no_std]
#![no_main]

#[macro_use] extern crate nucleo_l432kc as board;

use board::hal::gpio::*;
use board::led::LED0;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    loop {
        println!("Tick...");
        LED0.toggle_output();
        board::delay(500);
    }
}

