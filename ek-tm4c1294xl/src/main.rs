#![no_std]
#![no_main]

#[macro_use]
extern crate ek_tm4c1294xl as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for ek-tm4c1294xl");
    for i in 0..5 {
        println!("[pass] {}", i);
        board::delay(100);
    }
    println!("[done] All tests passed");
    loop {}
}
