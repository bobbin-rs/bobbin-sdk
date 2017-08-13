#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::pin::*;
use board::hal::sim::*;
use board::hal::adc::*;
use board::hal::port::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");

    let a0 = A0; // PTB2 / ADC0_SE12
    let a1 = A1; // PTB3 / ADC0_SE13
    
    let adc0 = ADC0;
    let ch0 = ADC0_CH0;

    println!("Setting up Pins");

    a0.port().sim_set_enabled(true);
    a0.mode_adc_se12(&adc0);

    a1.port().sim_set_enabled(true);
    a1.mode_adc_se13(&adc0);

    println!("Setting up ADCs");

    adc0        
        .sim_set_enabled(true)
        .init();    

    println!("Initialization Complete");

    loop {
        ch0.set_input_channel(12.into());        
        while !ch0.conversion_complete() {}
        let v0 = ch0.result();

        ch0.set_input_channel(13.into());        
        while !ch0.conversion_complete() {}
        let v1 = ch0.result();

        println!("{} {}", v0, v1);
        board::delay(1000);
    }
}
