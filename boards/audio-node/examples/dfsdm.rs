#![no_std]
#![no_main]

#[macro_use]
extern crate audio_node as board;

use board::hal::dfsdm::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("DFSDM Test");

    loop {}
}