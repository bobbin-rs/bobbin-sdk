#![no_std]
#![no_main]

#[macro_use]
extern crate openmv_m7 as board;

use board::clock::CLK;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("{:?}", CLK);
    loop {}
}
