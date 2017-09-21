#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f103rb as board;

use board::pin::*;
use board::hal::gpio::*;
use board::hal::adc::*;
use board::hal::rcc::RCC;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");
    
    let a0 = A0; // PA0 / ADC12_IN0   
    let a1 = A1; // PA1 / ADC12_IN1

    let ch1 = ADC1_CH0;
    let ch2 = ADC1_CH1;
    let adc = ch1.periph();

    a0.port().rcc_set_enabled(true);
    a1.port().rcc_set_enabled(true);

    a0.mode_analog();
    a1.mode_analog();

    RCC.with_cfgr(|r| r.set_adcpre(0b10));

    adc
        .rcc_set_enabled(true)
        .set_enabled(true)
        .calibrate();
    
    println!("Starting loop");
    loop {    
        let v0 = ch1.analog_read();
        let v1 = ch2.analog_read();
        println!("{} {}", v0, v1);
        board::delay(1_000);
    }
}
