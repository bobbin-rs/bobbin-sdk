#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f429zi as board;

use board::hal::gpio::PinExt;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    board::led::LED0.toggle_output();
    println!("Running Console");
    iprintln!("Running Console");
    board::led::LED0.toggle_output();
    let mut i = 0u32;
    loop {
        board::led::LED0.toggle_output();
        iprintln!("Hello, World! {}", i);
        println!("Hello, World! {}", i);
        i = i.wrapping_add(1);
        board::delay(1000);
    }
}
