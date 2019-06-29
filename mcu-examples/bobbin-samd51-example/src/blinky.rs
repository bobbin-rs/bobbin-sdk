#![no_std]
#![no_main]

extern crate panic_abort;
use cortex_m_rt::entry;
use cortex_m::asm;
use core::ptr;

use samd21::bobbin_hal::prelude::*;
use samd21::pin::PA23;
use samd21::port::PORTA;
const PORTA_OUTTGL: u32 = 0x4100_801C;

// LED Pin D13 = a23

#[entry]
fn main() -> ! {
    PA23.set_mode_output();

    // Set a23 high / set a23 low

    loop {
        for _ in 0..1_000_000 {
            asm::nop();
        }
        // PA23.toggle_output();
        // PORTA.set_outtgl(|r| r.set_outtgl(23, 1));
        unsafe {
            ptr::write_volatile(PORTA_OUTTGL as *mut u32, 1 << 23);
        }        
    }
}