#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::common::analog::*;
use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::sim::*;
use board::mcu::adc::*;
use board::mcu::port::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");

    let a0 = PTB2; // A0 // PTB2 / ADC0_SE12
    let a1 = PTB3; // A1 // PTB3 / ADC0_SE13
    
    let adc0 = ADC0;
    let ch12 = ADC0_CH12;
    let ch13 = ADC0_CH13;

    println!("Setting up Pins");

    a0.port().gate_enable();
    a0.connect_to(ch12);

    a1.port().gate_enable();
    a1.connect_to(ch13);

    println!("Setting up ADCs");

    adc0        
        .gate_enable()
        .init();    

    println!("Initialization Complete");

    loop {
        let v0: U16 = ch12.analog_read();
        let v1: U16 = ch13.analog_read();
        // ch0.set_input_channel(12.into());        
        // while !ch0.conversion_complete() {}
        // let v0 = ch0.result();

        // ch0.set_input_channel(13.into());        
        // while !ch0.conversion_complete() {}
        // let v1 = ch0.result();

        println!("{} {}", v0, v1);
        board::delay(1000);
    }
}
