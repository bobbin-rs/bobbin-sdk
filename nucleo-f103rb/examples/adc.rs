#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f103rb as board;

use board::pin::*;
use board::hal::gpio::*;
use board::hal::adc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");
    
    let a0 = A0; // PA0 / ADC12_IN0   
    let a1 = A1; // PA1 / ADC12_IN1

    let ch1 = ADC1_CH0;
    let ch2 = ADC1_CH1;
    let adc = ADC1;

    a0.mode_analog();
    a1.mode_analog();

    adc
        .rcc_set_enabled(true)
        .set_enabled(true);

    println!("Calibrating...");
    adc.with_cr2(|r| r.set_cal(true));
    while adc.cr2().cal() == 1 {}
    println!("Calibration Complete");

    adc.set_sequence_channel(1, 0);
    adc.set_sequence_length(1);
    
    loop {        
        adc.start();
        while !adc.complete() {}
        println!("{}", adc.data());
        board::delay(1_000);
    }
}
