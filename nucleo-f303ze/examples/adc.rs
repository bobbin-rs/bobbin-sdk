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
        .set_sequence(&[ch1.index(), ch2.index()]);        
    
    loop {        
        let mut data = [0u16; 2];
        adc.read_sequence(&mut data);
        println!("{:?}", data);
        board::delay(1_000);
    }
}
