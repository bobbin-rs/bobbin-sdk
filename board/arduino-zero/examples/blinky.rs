#![no_std]
#![no_main]
#![feature(asm)]

extern crate arduino_zero as board;
extern crate examples;

use board::prelude::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init().run(|brd| {
        let led = brd.led0();
        let app = examples::led::BlinkLed::new(led, brd.tick(), 500);
        app.run()
    })
}