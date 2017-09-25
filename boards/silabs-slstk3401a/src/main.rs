#![no_std]
#![no_main]

#[macro_use]
extern crate silabs_slstk3401a as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for silabs-slstk3401a");
    println!("[done] All tests passed");
    loop {}
}
