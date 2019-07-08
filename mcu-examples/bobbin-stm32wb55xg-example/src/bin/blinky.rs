#![no_std]
#![no_main]

extern crate bobbin_stm32wb55xg_example;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32wb55xx as mcu;

use cortex_m::asm;
use cortex_m_rt::entry;

use bobbin_stm32wb55xg_example::init;
use mcu::bobbin_hal::prelude::*;
use mcu::bobbin_mcu::prelude::*;
use mcu::pin::*;

// Green LED on Nucleo board
pub const LED: Pb0 = PB0;

#[entry]
unsafe fn main() -> ! {
    init();

    LED.port_gate_enable() // Enable the port associated with this pin.
        .mode_output(); // Set this port to digital output.

    loop {
        LED.toggle_output(); // Toggle the digital output.

        for _ in 0..5_000_000 {
            asm::nop();
        } // Delay
    }
}
