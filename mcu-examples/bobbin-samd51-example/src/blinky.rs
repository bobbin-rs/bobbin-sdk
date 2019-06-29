#![no_std]
#![no_main]

extern crate panic_abort;
use cortex_m_rt::entry;
use cortex_m::asm;

use samd51::port::PORT;

// LED Pin D13 = a23

#[entry]
fn main() -> ! {
    PORT.set_dirset(0, |r| r.set_dirset(23, 1));

    loop {
        for _ in 0..1_000_000 {
            asm::nop();
        }
        PORT.set_outtgl(0, |r| r.set_outtgl(23, 1));
    }
}