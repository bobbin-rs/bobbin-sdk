#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;
extern crate examples;

use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::dac::*;
// DAC_CH2 = DAC_OUT2 = PA5 = D13

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("DAC Test");

    let dac = DAC0;

    dac.gate_enable();
    dac.with_c0(|r| r.set_dacen(1).set_dacrfs(1));

    let mut v: u8 = 16;
    let s: u8 = 4;
    let mut d: bool = true;
    loop {
        dac.analog_write(U8::from(v));
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
