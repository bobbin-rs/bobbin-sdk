#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running");
    loop {}
}