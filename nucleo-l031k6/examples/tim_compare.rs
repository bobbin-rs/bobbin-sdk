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
        .set_prescale((t.clock(&CLK).unwrap() / 1000) as u16);
    ch.set_compare(500);
    t.start(1000);
    
    loop {
        ch.clr_compare_flag().wait_compare_flag();        
        println!("{}: Compare", t.counter());
        t.clr_timeout_flag().wait_timeout_flag();
        println!("{}: Timeout", t.counter());
    }
}
