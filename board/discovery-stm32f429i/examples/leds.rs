#![no_std]
#![no_main]

extern crate discovery_stm32f429i as board;
extern crate examples;

use board::prelude::*;
use examples::leds::BlinkLeds;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init().run(|brd| {
        let leds = [brd.led0(), brd.led1()];
        let tick = brd.tick();
        BlinkLeds::new(&leds, tick, 500).run()
    })
}