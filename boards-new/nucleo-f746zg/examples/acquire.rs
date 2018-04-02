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
    if let Some(gpio) = mcu.gpiob() {
        if let Some(pin) = gpio.pb0() {
            let mut value = false;
            loop {
                pin.toggle_output();            
                board::delay(500);
                value = !value;
            }        
        }
    }
    loop {}
}
