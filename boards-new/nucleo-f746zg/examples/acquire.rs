#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;
extern crate examples;

use board::common::board::*;
use board::common::digital::DigitalOutput;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let brd = board::board();
    let mcu = brd.mcu();
    let pin = mcu.gpiob().and_then(|gpio| gpio.pb0());
    if let Some(pin) = pin {
        loop {
            pin.toggle_output();
            board::delay(500);
        }
    }
    loop {}
}
