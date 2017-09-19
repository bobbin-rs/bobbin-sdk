#![no_std]
#![no_main]

#[macro_use]
extern crate blue_f4 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for nucleo-f303ze");
    for i in 0..5 {
        println!("[pass] {}", i);
        board::delay(100);
    }
    println!("[done] All tests passed");
    loop {}
}
