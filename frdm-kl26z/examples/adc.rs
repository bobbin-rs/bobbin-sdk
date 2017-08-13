#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_kl26z as board;

use board::pin::*;
use board::hal::port::*;
use board::hal::adc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");
   
    let adc0 = ADC0;
    let ch0 = ADC0_CH0;

    let a0 = A0; // PTB0 / ADC0_SE8

    println!("Setting up pins");

    a0.port().sim_set_enabled(true);
    a0.mode_adc_se8(&adc0);

    println!("Setting up ADC");

    adc0        
        .sim_set_enabled(true)
        .init();    

    println!("Initialization Complete");

    loop {
        // Read Temperature Sensor on AD26 (value ~ 55)
        ch0.set_input_channel(26.into());        
        while !ch0.conversion_complete() {}
        let v0 = ch0.result();

        // Read A0 on AD8
        ch0.set_input_channel(8.into());        
        while !ch0.conversion_complete() {}
        let v1 = ch0.result();

        println!("{} {}", v0, v1);
        board::delay(1000);
    }
}
