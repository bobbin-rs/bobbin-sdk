#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

use board::hal::gclk;
use board::hal::tc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("Running TC Test");
    gclk::set_clk(gclk::GenericClock::TC4_TC5, gclk::GenericClockGen::GClkGen2);

    let tc = TC4;

    tc.pm_set_enabled(true);
    tc.configure_16bit(Config {
        prescsync: Prescsync::GCLK,
        runstdby: false,
        prescaler: Prescaler::Div1,
        wavegen: Wavegen::NFRQ,
    });    
    

    tc
        .set_compare(512)
        .start(1024);
    loop {
        tc.clr_compare_flag().wait_compare_flag();
        println!("{} compare", tc.counter());
        tc.clr_timeout_flag().wait_timeout_flag();
        println!("{} timeout", tc.counter());
    }
}

pub const TC: Tc3 = TC3;

pub fn init() {
    
}
