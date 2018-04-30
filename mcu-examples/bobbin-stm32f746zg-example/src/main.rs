#![no_std]
#![feature(asm, lang_items)]

extern crate cortex_m_rt;
extern crate stm32f74x as mcu;

mod lang_items;

// LED0 = PB0;

use mcu::bobbin_mcu::prelude::{Pin, GateEn};
use mcu::bobbin_hal::prelude::DigitalOutput;
use mcu::pin::PB0;

fn main() {
    let pin = PB0;

    pin.port().gate_enable();
    pin.mode_output();

    loop {
        pin.toggle_output();
        for _ in 0..5_000_000 { unsafe { asm!("nop") }}
    }    
}
