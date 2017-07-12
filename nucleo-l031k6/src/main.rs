#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l031k6 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("# Running Tests");
    println!("[start]");
    for i in 0..5 {
        println!("[pass] {}", i);
        board::delay(500);
    }
    println!("[done]");
    loop {}
}
