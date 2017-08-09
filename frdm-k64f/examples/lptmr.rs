#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::hal::lptmr::*;
use board::hal::clock::*;
use board::clock::CLK;

// Assume PIT bus clock is 60Mhz

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("LPTMR Test");
    println!("{:?}", CLK);
    let t0 = LPTMR0;

    t0
        .sim_set_enabled(true)
        .set_pcs(PrescalerClock::McgIrClk);

    // Prescaler is /2 by default
    let period = t0.clock(&CLK).unwrap() / 2;
    println!("Period: {}", period);
    let mut i = 0;
    loop {
        t0.delay(period as u16);
        println!("tick {}", i);
        i += 1;
    }
}
