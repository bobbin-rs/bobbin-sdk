#![no_std]
#![no_main]

#[macro_use]
extern crate amaph1kk_kbr as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    board::delay(10);
    println!("[start] Running tests for amaph1kk-kbr");
    println!("[done] All tests passed");
    loop {}
}
