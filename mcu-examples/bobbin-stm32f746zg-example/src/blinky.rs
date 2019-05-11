#![no_std]
#![no_main]
#![feature(asm)]

extern crate cortex_m_rt;
extern crate panic_abort;
extern crate stm32f74x as mcu;

use cortex_m_rt::entry;
use mcu::bobbin_mcu::prelude::*;
use mcu::bobbin_hal::prelude::*;
use mcu::pin::*;

// The LED for this board is pin PB0. Notice that PB0 is a constant of type Pb0.
pub const LED: Pb0 = PB0;

#[entry]
unsafe fn main() -> ! {
    LED
        .port_gate_enable() // Enable the port associated with this pin.
        .mode_output(); // Set this port to digital output.

    loop {
        LED.toggle_output(); // Toggle the digital output.        
        for _ in 0..5_000_000 { asm!("nop") } // Delay
    }    
}
