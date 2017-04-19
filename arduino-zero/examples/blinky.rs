#![no_std]
#![no_main]
#![feature(asm)]

//#[macro_use]
extern crate arduino_zero as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::led0();
    loop {
        led0.set(true);
        board::delay(1024);
        led0.set(false);
        board::delay(1024);
    }
}
