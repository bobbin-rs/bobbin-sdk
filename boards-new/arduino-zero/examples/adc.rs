#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;
extern crate examples;

use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::adc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let brd = board::board();
    println!("Analog Test");

    ADC.gate_enable();
    ADC.init();

    let a0 = PA02;
    let ch0 = ADC_CH0;

    // a0.mode_ain(&ch0);
    a0.connect_to(ch0);

    let ch0: AdcCh = ch0.into();
    let mut app = examples::adc::AdcExample::new(brd.console(), ch0, brd, 500, U12::from(0));
    app.run()
    // loop {
    //     let v0: U12 = ch0.analog_read();
    //     let v1: U8 = ch2.analog_read();
    //     println!("{}, {}", v0, v1);
    //     board::delay(500);
    // }

}
