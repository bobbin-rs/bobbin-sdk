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
    println!("{:?}", CLK);
    let ch = FTM0_CH2;
    let t0 = ch.periph();

    t0
        .sim_set_enabled(true);

    println!("t0: {:?}", t0.clock(&CLK));
    let prescale = 128;
    let period = ((t0.clock(&CLK).unwrap() / prescale) / 1000) as u16;

    println!("{} / {}", period, prescale);

    t0.set_prescale(prescale as u16);
    ch
        .with_csc(|r| r.set_chie(0).set_msb(0).set_msa(1).set_elsb(0).set_elsa(0))
        .set_compare(period >> 1);

    loop {
        t0.delay(1000);
        println!("tick");
    }


    // t0.start(period as u16);
    // loop {
    //     println!("{} - Wait Compare", t0.counter());
    //     ch.clr_compare_flag().wait_compare_flag();
    //     println!("{} - Wait Timeout", t0.counter());
    //     t0.clr_timeout_flag().wait_timeout_flag();
    // }
}
