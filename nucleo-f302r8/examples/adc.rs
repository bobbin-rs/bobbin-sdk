#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f302r8 as board;

use board::common::bits::*;
use board::pin::*;
use board::hal::gpio::ModeAdc;
use board::hal::adc::*;
use board::hal::rcc::RCC;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");
    RCC.with_cfgr2(|r| r.set_adc12pres(0b10001));
    
    let a0 = A0;    
    let a1 = A1;

    let ch1 = ADC1_CH1;
    let ch2 = ADC1_CH2;

    a0.mode_adc(&ch1);
    a1.mode_adc(&ch2);

    let adc = ch1.periph();
    adc
        .rcc_set_enabled(true)
        .init();
    
    loop {        
        let v0: U12 = ch1.analog_read();
        let v1: U10 = ch2.analog_read();
        println!("{} {}", v0, v1);
        board::delay(1_000);
    }
}
