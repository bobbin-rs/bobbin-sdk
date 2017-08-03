#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

use board::hal::gclk;
use board::hal::tcc::*;
use board::hal::clock::Clock;
use board::clock::CLK;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("Running TCC Test");
    gclk::set_clk(gclk::GenericClock::TCC0_TCC1, gclk::GenericClockGen::GClkGen2);    

    let ch0 = TCC0_CH0;
    let ch1 = TCC0_CH1;
    let ch2 = TCC0_CH2;
    let ch3 = TCC0_CH3;
    let tcc = TCC0;
    
    tcc.pm_set_enabled(true);   
    println!("{:?}", CLK);
    println!("TCC Clock: {:?}", tcc.clock(&CLK));
    tcc.start(1024);
    ch0.set_compare(200);
    ch1.set_compare(400);
    ch2.set_compare(600);
    ch3.set_compare(800);
    
    loop {
        ch0.clr_compare_flag().wait_compare_flag();
        println!("{} compare 0", tcc.counter());
        ch1.clr_compare_flag().wait_compare_flag();
        println!("{} compare 1", tcc.counter());
        ch2.clr_compare_flag().wait_compare_flag();
        println!("{} compare 2", tcc.counter());
        ch3.clr_compare_flag().wait_compare_flag();
        println!("{} compare 3", tcc.counter());
        tcc.clr_timeout_flag().wait_timeout_flag();
        println!("{} timeout", tcc.counter());
    }
}
