#![no_std]
#![no_main]

extern crate nucleo_f429zi as board;
extern crate examples;

use board::prelude::*;
use examples::leds::BlinkLeds;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init().run(|brd| {
        let leds = [brd.led0(), brd.led1(), brd.led2()];
        let tick = brd.tick();
        BlinkLeds::new(&leds, tick, 500).run()
    })
}