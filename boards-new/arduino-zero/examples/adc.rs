#![no_std]
#![no_main]
#![feature(asm)]

extern crate arduino_zero as board;
extern crate examples;

use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::adc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    let brd = board::board();

    ADC.gate_enable();
    ADC.init();

    let a0 = PA02;
    let ch0 = ADC_CH0;

    a0.connect_to(ch0);
    

    let ch0: AdcCh = ch0.into();
    let mut app = examples::adc::AdcExample::new(brd.console(), ch0, brd, 500, U12::from(0));
    app.run()
}
