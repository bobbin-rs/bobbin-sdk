#![no_std]
#![no_main]

#[macro_use]
extern crate nero_f7 as board;

use board::hal::gpio::{DigitalOutput};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    board::led::LED0.toggle_output();
    println!("Running Console");
    board::led::LED0.toggle_output();
    let mut i = 0u32;
    loop {
        board::led::LED0.toggle_output();
        println!("Hello, World! {}", i);
        i = i.wrapping_add(1);
        board::delay(1000);
    }
}
