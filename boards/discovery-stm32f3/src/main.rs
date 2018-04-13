#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f3 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running");
    loop {}
}