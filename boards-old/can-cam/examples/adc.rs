#![no_std]
#![no_main]

#[macro_use]
extern crate can_cam as board;

use board::common::bits::*;
use board::pin::*;
use board::hal::gpio::ModeAdc;
use board::hal::adc::*;
use board::chip::c_adc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");
   
    let a0 = A0; // PA3 // ADC123_IN3
    let a1 = A1; // PC0 // ADC123_IN10

    let ch1 = ADC1_CH3;
    let ch2 = ADC1_CH10;

    a0.port().rcc_set_enabled(true);
    a1.port().rcc_set_enabled(true);

    a0.mode_adc(&ch1);
    a1.mode_adc(&ch2);

    C_ADC.with_ccr(|r| r.set_adcpre(0b10));

    let adc = ch1.periph();
    adc
        .rcc_set_enabled(true)
        .set_enabled(true)
        .calibrate();

    loop {    
        let v0: U12 = ch1.analog_read();
        let v1: U12 = ch2.analog_read();
        
        println!("{} {}", v0, v1);
        board::delay(1_000);
    }
}
