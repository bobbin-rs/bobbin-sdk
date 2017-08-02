#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l031k6 as board;

use board::hal::tim::*;
use board::hal::clock::Clock;
use board::clock::CLK;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Compare Test");

    let ch = TIM2_CH2;

    let t = ch.periph();
    t
        .rcc_set_enabled(true)
        .set_period(2000)
        .set_prescaler(((t.clock(&CLK).unwrap() / 1000) - 1) as u16)
        .set_enabled(true);
    ch.set_compare(1000);

    loop {
        ch.wait_compare_flag().clr_compare_flag();
        println!("Compare");
        t.wait_timeout().clr_timeout();
        println!("Timeout");
    }
}
