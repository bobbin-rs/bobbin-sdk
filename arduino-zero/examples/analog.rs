#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

use board::hal::port::PinExt;
use board::adc::{self, AnalogInput};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Analog Test");
    adc::init();
    println!("Setting Pin Mode");
    let a = adc::A0;
    a.set_mode_analog();
    println!("Reading A0");
    loop {
        let v = a.analog_input();
        println!("v: {}", v);
        board::delay(500);
    }

}
