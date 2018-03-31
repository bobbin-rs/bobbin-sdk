#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate examples;

use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::dac::*;

// DAC_CH2 = DAC_OUT2 = PA5 = D13

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("DAC Test");

    let dac = DAC;
    let dac_ch = DAC_CH2;
    let dac_pin = PA5;

    dac.gate_enable();
    dac_pin.port().gate_enable();
    dac_pin.mode_analog();
    dac_pin.connect_to(dac_ch);
    
    dac.with_cr(|r| r.set_en2(1));
    

    let mut v: u8 = 16;
    let s: u8 = 4;
    let mut d: bool = true;
    loop {
        dac.with_dhr8r2(|r| r.set_dacc2dhr(v));
        if d {
            v += s;
            if v == 240 {
                d = !d;
            }
        } else {
            v -= s;
            if v == 16 {
                d = !d;
            }
        }
        board::delay(5);
    }
}
