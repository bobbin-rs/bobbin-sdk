#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;
use board::led::*;
use board::common::hal::digital::DigitalOutput;

// use board::console;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Console");
    let mut i = 0u32;
    loop {
        LED0.toggle_output();
        println!("Hello, World! {}", i);
        i = i.wrapping_add(1);
        board::delay(1000);
    }
}
