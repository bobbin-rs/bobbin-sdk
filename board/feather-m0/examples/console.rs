#![no_std]
#![no_main]

#[macro_use]
extern crate feather_m0 as board;

use board::led::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    LED0.toggle();
    println!("Running Console");
    loop {
        LED0.toggle();
        println!("Tick...");
        board::delay(500);
    }
}
