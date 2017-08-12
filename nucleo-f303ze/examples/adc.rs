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
    let ch1 = ADC1_CH1;
    let adc = ch1.periph();

    a0.mode_adc_in(&ch1);
    adc
        .rcc_set_enabled(true)
        .init()
        .set_sequence_channel(1, 1)
        .set_sequence_length(1);

    println!("Initialization Complete");

    loop {
        adc.start();

        while !adc.end_of_conversion() {}
        println!("{}", adc.data());
        board::delay(1_000);
    }
}
