#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f3 as board;

use board::led::LED1;
use board::hal::gpio::DigitalOutput;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Console");
    let mut i = 0u32;
    loop {
        LED1.toggle_output();
        println!("Hello, World! {}", i);
        i = i.wrapping_add(1);
        board::delay(1000);
    }
}
