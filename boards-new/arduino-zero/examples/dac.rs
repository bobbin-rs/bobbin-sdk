#![no_std]
#![no_main]

#[macro_use]
extern crate arduino_zero as board;
extern crate examples;

// use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::dac::*;
use board::mcu::gclk;
use board::mcu::pm::*;

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
        while gclk::GCLK.status().syncbusy() != 0 {}

        // Set GCLK_GEN0 as source for ADC
        gclk::GCLK.set_clkctrl(|r| r

            .set_id(0x21) // GCLK_DAC
            .set_gen(0x0)
            .set_clken(true)
        );    
        while gclk::GCLK.status().syncbusy() != 0 {}
        while DAC.status().syncbusy() != 0 {}
        DAC.set_ctrlb(|r| r.set_refsel(0x01).set_eoen(1));
        while DAC.status().syncbusy() != 0 {}
        DAC.set_ctrla(|r| r.set_enable(1));
        while DAC.status().syncbusy() != 0 {}
    }

    println!("Running Loop");

    let mut v: u8 = 16;
    let s: u8 = 4;
    let mut d: bool = true;
    loop {
        DAC.set_data(|r| r.set_data(v << 1));
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
