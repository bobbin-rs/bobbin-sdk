#![no_std]
#![no_main]

#[macro_use]
extern crate arduino_zero as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("Running Console");
    loop {
        println!("Tick...");
        board::delay(500);
    }
}
