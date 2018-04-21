#![no_std]
#![no_main]

#[macro_use]
extern crate silabs_slstk3401a as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for silabs-slstk3401a");
    test_systick();
    println!("[done] All tests passed");
    loop {}
}

fn test_systick() {
    use board::hal::systick::*;

    println!("# Testing SYSTICK");
    test_systick(&SYSTICK, ClockSource::Internal);
    println!("[pass] SYSTICK OK");
}