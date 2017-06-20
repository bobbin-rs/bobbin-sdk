#![no_std]
#![no_main]

#[macro_use]
extern crate teensy_32 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    iprintln!("Running Console");
    let mut i = 0u32;
    loop {
        iprintln!("Hello, World! {}", i);
        i = i.wrapping_add(1);
        board::delay(1000);
    }
}
