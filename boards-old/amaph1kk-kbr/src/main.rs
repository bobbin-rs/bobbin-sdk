#![no_std]
#![no_main]

#[macro_use]
extern crate amaph1kk_kbr as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    board::delay(10);
    println!("[start] Running tests for amaph1kk-kbr");
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