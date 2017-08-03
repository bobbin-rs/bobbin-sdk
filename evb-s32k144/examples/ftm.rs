#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

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

    t0
        .set_prescaler(prescale as u16)
        .set_period(period as u16);
    ch
        .with_csc(|r| r.set_chie(1).set_msb(0).set_msa(1).set_elsb(0).set_elsa(0))
        .set_value(period >> 1);
        // Setup Edge PWM    
    
    // ch.set_pwmen(true);
    // ch.with_csc(|r| r.set_msb(1).set_msa(0).set_elsb(0).set_elsa(1));
    // ch.set_value(1024);

    t0.set_enabled(true);
    loop {
        ch.clr_compare_flag().wait_compare_flag().clr_compare_flag();
        println!("compare: {}", ch.value());
        t0.wait_timeout().clr_timeout();
        println!("timeout");
        // t0.delay(period as u16, prescale as u16);
    }
}
