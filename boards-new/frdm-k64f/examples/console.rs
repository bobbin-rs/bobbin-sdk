#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::led::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Console");

    loop {
        println!("Tick...");
        LED0.toggle_output();
        board::delay(500);
    }
}
