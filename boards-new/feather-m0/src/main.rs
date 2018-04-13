#![no_std]
#![no_main]

#[macro_use]
extern crate feather_m0 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running");
    loop {}
}