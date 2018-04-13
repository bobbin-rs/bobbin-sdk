#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_l432kc as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    println!("Running Console");
    loop {
        println!("Tick...");
        board::delay(500);
    }
}