#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

use board::hal::gclk;
use board::hal::tcc::*;
use board::hal::port::{PmEnabled, ModeWo1};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0;

    let ch = TCC2_CH1;
    let tcc = ch.periph();
    tcc.pm_set_enabled(true);
    led0.mode_wo_1(tcc);

    println!("Running PWM");    

    gclk::set_clk(gclk::GenericClock::TCC2_TC3, gclk::GenericClockGen::GClkGen2);


    tcc.with_per(|r| r.set_per(64));
    tcc.with_cc(1, |r| r.set_cc(32));

    tcc.with_wave(|r| r.set_wavegen(0x02));
    tcc.with_ctrla(|r| r.set_cpten1(1).set_enable(1));

    println!("Setup Complete");

    loop {
        //println!("period: {} cc: {}, count: {}", tcc.per().per(), tcc.cc(1).cc(), tcc.count().count());
        board::delay(1000);
    }
}
