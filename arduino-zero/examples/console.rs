#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Console");
    let mut i = 0u32;
    loop {
        println!("Hello, World! {}", i);
        i = i.wrapping_add(1);
        board::delay(1024);
    }
}
