#![no_std]
#![no_main]

#[macro_use]
extern crate feather_m0 as board;
extern crate examples;

// use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::dac::*;
use board::mcu::gclk;

// DAC = PA0 = PA02
#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    board::delay(500);
    println!("DAC Test");

    PA02.port().gate_enable();
    PA02.set_dir_input();
    PA02.set_pull_enabled(false);
    PA02.set_pmux_enabled(false);
    PA02.set_pmux_enabled(true).set_pmux(1);

    let dac = DAC;
    dac.gate_enable();
    {
        DAC.init();
        DAC.enable();
    }

    println!("Running Loop");

    let mut v: u8 = 16;
    let s: u8 = 4;
    let mut d: bool = true;
    loop {
        DAC.set_data(|r| r.set_data((v as u16) << 2));
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
