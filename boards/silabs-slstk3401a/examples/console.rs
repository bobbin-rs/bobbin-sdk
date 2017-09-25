#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

#[macro_use]
extern crate silabs_slstk3401a as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    let mut i: u32 = 0;
    loop {
        println!("Hello, World {}", i);
        i = i.wrapping_add(1);
        board::delay(500);
    }
}
