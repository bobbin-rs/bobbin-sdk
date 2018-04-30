#![no_std]
#![feature(asm)]

extern crate cortex_m_rt;
extern crate panic_abort;
extern crate stm32f74x as mcu;

// LED0 = PB0;

use mcu::bobbin_mcu::prelude::*;
use mcu::bobbin_hal::prelude::*;
use mcu::pin::*;

pub const LED: Pb0 = PB0;

fn main() {
    mcu::ext::init();
    
    LED
        .port_gate_enable()
        .mode_output();

    loop {
        LED.toggle_output();
        for _ in 0..5_000_000 { unsafe { asm!("nop") }}
    }    
}
