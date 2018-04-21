#![no_std]
#![no_main]

#[macro_use]
extern crate arduino_zero as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    println!("Running");
    loop {}
}