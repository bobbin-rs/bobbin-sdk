#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::pin::*;
use board::hal::adc::*;
use board::hal::pcc::*;
use board::hal::port::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");

    let a0 = A0; // ADC1_SE6
    let a1 = A1; // ADC1_SE7
    let p0 = POT0; // PTC14 = ADC0_SE12

    let adc0 = ADC0;
    let ch0 = ADC0_CH12;

    let adc1 = ADC1;
    let ch1 = ADC1_CH6;
    let ch2 = ADC1_CH7;

    println!("Setting up Pins");

    p0.port().pcc_set_enabled(true);
    p0.mode_adc(&ch0);

    a0.port().pcc_set_enabled(true);
    a0.mode_adc(&ch1);

    a1.port().pcc_set_enabled(true);
    a1.mode_adc(&ch2);

    println!("Setting up ADCs");

    adc0
        .pcc_set_clock_source(ClockSource::SPLLDIV2)
        .pcc_set_enabled(true)
        .init();    

    adc1
        .pcc_set_clock_source(ClockSource::SPLLDIV2)
        .pcc_set_enabled(true)
        .init();


    println!("Initialization Complete");

    loop {
        let v0 = ch0.start().wait().read();
        let v1 = ch1.start().wait().read();
        println!("{} {}", v0, v1);
        board::delay(1000);
    }
}
