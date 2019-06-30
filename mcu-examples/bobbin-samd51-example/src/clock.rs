#![no_std]
#![no_main]

extern crate panic_abort;
use cortex_m_rt::entry;
use cortex_m::asm;

use samd51::bobbin_hal::prelude::*;
use samd51::pin::PA23;

// LED Pin D13 = a23

#[entry]
fn main() -> ! {
    PA23.set_output(true);

    loop {
        for _ in 0..1_000_000 {
            asm::nop();
        }
        PA23.toggle_output();
    }
}