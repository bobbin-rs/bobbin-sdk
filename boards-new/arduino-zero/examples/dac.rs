#![no_std]
#![no_main]

#[macro_use]
extern crate arduino_zero as board;
extern crate examples;

// use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::dac::*;
use board::mcu::gclk;
// DAC = PA0 = PA02

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("DAC Test");

    let dac = DAC;
    // PA02.connect_to(dac); // NOTE - this may burn out chip!!!!
    dac.gate_enable();
    {
        while gclk::GCLK.status().syncbusy() != 0 {}

        // Set GCLK_GEN0 as source for ADC
        gclk::GCLK.set_clkctrl(|r| r
            .set_id(0x1e) // GCLK_ADC
            .set_gen(0x0)
            .set_clken(true)
        );    
        DAC.set_ctrlb(|r| r.set_refsel(0x01).set_eoen(1));
        DAC.with_ctrla(|r| r.set_enable(1));
    }

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
