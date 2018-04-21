#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::common::bits::*;
use board::hal::adc::*;
use board::hal::pcc::*;
use board::hal::port::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");

    let p0 = PTC14; // PTC14 = ADC0_SE12

    let adc0 = ADC0;
    let ch0 = ADC0_CH12;

    println!("Setting up Pins");

    p0.port().pcc_set_enabled(true);
    p0.mode_adc(&ch0);
    println!("Setting up ADCs");

    adc0
        .pcc_set_clock_source(ClockSource::SPLLDIV2)
        .pcc_set_enabled(true)
        .init();    

    println!("Initialization Complete");

    loop {
        let v0: U8 = ch0.analog_read();
        let v1: U10 = ch0.analog_read();
        let v2: U12 = ch0.analog_read();
        println!("{} {} {}", v0, v1, v2);
        board::delay(1000);
    }
}
