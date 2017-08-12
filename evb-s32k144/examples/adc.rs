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
    let ch0 = ADC0_CH0;

    let adc1 = ADC1;
    let ch1 = ADC1_CH0;

    println!("Setting up Pins");

    p0.port().pcc_set_enabled(true);
    p0.mode_adc_in_12(&adc0);

    a0.port().pcc_set_enabled(true);
    a0.mode_adc_in_6(&adc1);

    a1.port().pcc_set_enabled(true);
    a1.mode_adc_in_7(&adc1);

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
        ch0.set_input_channel(12.into());
        while !ch0.conversion_complete() {}
        let v0 = ch0.result();

        ch1.set_input_channel(6.into());        
        while !ch1.conversion_complete() {}
        let v1 = ch1.result();

        ch1.set_input_channel(7.into());
        while !ch1.conversion_complete() {}
        let v2 = ch1.result();
        

        println!("{} {} {}", v0, v1, v2);
        board::delay(1000);
    }
}
