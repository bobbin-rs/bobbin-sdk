#![no_std]
#![no_main]

#[macro_use]
extern crate feather_m0 as board;

use board::prelude::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();
    brd.run(|brd| {
        println!("[start] Running tests for {}", brd.id());
        println!("[done]");
        loop {}
    })
}