#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f303ze as board;

use board::pin::*;
use board::hal::gpio::ModeAdcIn;
use board::hal::adc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");
    
    let a0 = A0;    
    let a1 = A1;

    let ch1 = ADC1_CH1;
    let ch2 = ADC1_CH2;
    

    a0.mode_adc_in(&ch1);
    a1.mode_adc_in(&ch2);

    let adc = ch1.periph();
    adc
        .rcc_set_enabled(true)
        .init()
        .set_sequence_channel(1, 1)
        .set_sequence_channel(2, 2)
        .set_sequence_length(2);

    println!("Initialization Complete");

    loop {
        adc.start();

        while !adc.end_of_conversion() {}
        let v0 = adc.data();
        while !adc.end_of_conversion() {}
        let v1 = adc.data();
        println!("{} {}", v0, v1);
        board::delay(1_000);
    }
}
