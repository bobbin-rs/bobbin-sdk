#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_kl26z as board;

use board::common::bits::*;
use board::pin::*;
use board::hal::port::*;
use board::hal::adc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");

    let a0 = A0; // PTB0 / ADC0_SE8   
    let adc0 = ADC0;
    let ch8 = ADC0_CH8;
    let ch16 = ADC0_TEMP;

    println!("Setting up pins");

    a0.port().sim_set_enabled(true);
    a0.mode_adc(&ch8);

    println!("Setting up ADC");

    adc0        
        .sim_set_enabled(true)
        .init();    

    println!("Initialization Complete");

    loop {
        // Read Temperature Sensor on AD26 (value ~ 55)
        let v0: U8 = ch16.analog_read();
        // ch0.set_input_channel(26.into());        
        // while !ch0.conversion_complete() {}
        // let v0 = ch0.result();

        // Read A0 on AD8
        let v1: U8 = ch8.analog_read();

        // ch0.set_input_channel(8.into());        
        // while !ch0.conversion_complete() {}
        // let v1 = ch0.result();

        println!("{} {}", v0, v1);
        board::delay(1000);
    }
}
