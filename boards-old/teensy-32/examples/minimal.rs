#![no_std]
#![no_main]

extern crate teensy_32 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}