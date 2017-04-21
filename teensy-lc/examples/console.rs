#![no_std]
#![no_main]

#[macro_use]
extern crate teensy_lc as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Console");
    let mut i = 0u32;
    loop {
        println!("Hello, World! {}", i);
        i = i.wrapping_add(1);
        board::delay(1000);
    }
}
