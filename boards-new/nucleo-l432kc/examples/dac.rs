#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;
extern crate examples;

use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::dac::*;

// DAC1_CH1 = DAC1_OUT1 = PA4 = A3

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("DAC Test");

    let dac = DAC1;
    let dac_ch = DAC1_CH1;
    let dac_pin = PA4;

    dac.gate_enable();
    dac_pin.port().gate_enable();
    dac_pin.mode_analog();
    dac_pin.connect_to(dac_ch);
    
    dac_ch.enable();    

    let mut v: u8 = 16;
    let s: u8 = 4;
    let mut d: bool = true;
    loop {
        dac_ch.analog_write(U8::from(v));
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
