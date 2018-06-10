#![no_std]
#![no_main]
#![feature(asm)]

extern crate discovery_stm32f429i as board;
extern crate examples;

use board::common::board::*;
use board::common::hal::digital::DigitalOutput;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    let brd = board::board();
    let mcu = brd.mcu();
    if let Some(pin) = mcu.pb0() {
        loop {
            pin.toggle_output();
            board::delay(500);
        }
    }
    loop {}
}
