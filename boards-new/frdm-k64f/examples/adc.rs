#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;
extern crate examples;

use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::adc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    let brd = board::board();
    println!("ADC Test");

    let a0 = PTB2; // A0 // PTB2 / ADC0_SE12
    
    let adc0 = ADC0;
    let ch12 = ADC0_CH12;

    a0.port().gate_enable();
    a0.connect_to(ch12);

    adc0        
        .gate_enable()
        .init();    

    let ch12: AdcCh = ch12.into();

    let mut app = examples::adc::AdcExample::new(brd.console(), ch12, brd, 500, U16::from(0));
    app.run()

}
