#![no_std]
#![no_main]

#[macro_use]
extern crate onebit_elle0 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    iprintln!("Running ITM Test");
    let mut i = 0u32;
    loop {
        iprintln!("ITM: Hello, World! {}", i);
        i = i.wrapping_add(1);
        board::delay(1000);
    }
}
