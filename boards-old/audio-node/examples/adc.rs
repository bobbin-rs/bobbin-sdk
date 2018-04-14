#![no_std]
#![no_main]

#[macro_use]
extern crate audio_node as board;

use board::common::bits::*;
use board::pin::*;
use board::hal::gpio::ModeAdc;
use board::hal::adc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");
    
    let a0 = A0;    
    let a1 = A1;

    let ch1 = ADC1_CH0;
    let ch2 = ADC1_CH1;

    a0.port().rcc_set_enabled(true);
    a1.port().rcc_set_enabled(true);

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