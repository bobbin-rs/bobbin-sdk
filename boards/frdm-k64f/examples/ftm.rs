#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::hal::ftm::*;
use board::hal::clock::Clock;
use board::clock::CLK;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    println!("FTM Test");    
    let t0 = FTM0;
    t0
        .sim_set_enabled(true);

    println!("t0: {:?}", t0.clock(&CLK));
    let prescale = 128;
    let period = (t0.clock(&CLK).unwrap() / prescale) / 1000;

    println!("{} / {}", period, prescale);

    t0.set_prescale(prescale as u16);

    let mut i = 0;
    loop {
        for _ in 0..1000 {
            t0.delay(period as u16);
        }
        println!("tick {}", i);
        i += 1;
    }


    // t0.start(period as u16);
    // loop {
    //     println!("{} - Wait Compare", t0.counter());
    //     ch.clr_compare_flag().wait_compare_flag();
    //     println!("{} - Wait Timeout", t0.counter());
    //     t0.clr_timeout_flag().wait_timeout_flag();
    // }
}
