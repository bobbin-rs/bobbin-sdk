#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate feather_m0 as board;

use board::hal::port::DigitalOutput;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0;

    println!("Running Console");    
    let mut i = 0u32;
    loop {
        led0.toggle_output();
        println!("Hello, World! {}", i);
        i = i.wrapping_add(1);
        board::delay(1024);
    }
}
