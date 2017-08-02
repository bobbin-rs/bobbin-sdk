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
    let reload = t0.clock(&CLK).unwrap() / prescale;

    println!("{} / {}", reload, prescale);

    loop {
        println!("tick");
        t0.delay(reload as u16, prescale as u16);
    }
}
