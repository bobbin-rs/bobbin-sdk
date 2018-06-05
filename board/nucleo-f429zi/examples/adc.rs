#![no_std]
#![no_main]

extern crate nucleo_f429zi as board;
extern crate examples;

use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::adc::*;
use board::mcu::c_adc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    let brd = board::board();
    
    let a0 = PA3;  // A0

    let adc = ADC1;
    let ch1 = ADC1_CH3;

    adc.gate_enable();
    a0.port().gate_enable();
    a0.connect_to(ch1);
    a0.mode_analog();

    adc.set_enabled(true).calibrate();
    C_ADC.with_ccr(|r| r.set_tsvrefe(1));    
    
    let ch1: AdcCh = ch1.into();

    let mut app = examples::adc::AdcExample::new(brd.console(), ch1, brd, 500, U12::from(0));
    app.run()

}
