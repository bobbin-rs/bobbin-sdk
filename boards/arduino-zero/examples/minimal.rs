#![no_std]
#![no_main]

extern crate arduino_zero as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    loop {}
}