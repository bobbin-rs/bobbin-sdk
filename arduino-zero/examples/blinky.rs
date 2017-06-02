#![no_std]
#![no_main]
#![feature(asm)]

extern crate arduino_zero as board;

use board::hal::port::PinExt;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0;
    loop {
        led0.set_output(true);
        board::delay(1024);
        led0.set_output(false);
        board::delay(1024);
    }
}
