#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::common::Channel;
use board::hal::ftm::*;
use board::hal::pcc;
use board::hal::clock::Clock;
use board::clock::CLK;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    println!("FTM Test");    
    let ch = FTM0_CH2;
    let t0 = ch.periph();

    t0
        .pcc_set_clock_source(pcc::ClockSource::SOSCDIV2)
        .pcc_set_enabled(true);

    let prescale = 128;
    let period = (t0.clock(&CLK).unwrap() / prescale) as u16;

    println!("{} / {}", period, prescale);

    t0.set_prescale(prescale as u16);
    ch
        .with_csc(|r| r.set_chie(0).set_msb(0).set_msa(1).set_elsb(0).set_elsa(0))
        .set_compare(period >> 1);
    
    t0.start(period as u16);
    loop {
        println!("{} - Wait Compare", t0.counter());
        ch.clr_compare_flag().wait_compare_flag();
        println!("{} - Wait Timeout", t0.counter());
        t0.clr_timeout_flag().wait_timeout_flag();
    }
}
