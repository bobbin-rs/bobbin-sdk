#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::hal::pit::*;
use board::hal::clock::*;
use board::clock::CLK;

// Assume PIT bus clock is 60Mhz

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    let t0 = PIT;
    let ch = PIT_CH0;

    t0.sim_set_enabled(true);

    let period = t0.clock(&CLK).unwrap();

    loop {
        ch.delay(period);
        println!("tick");
    }
}
