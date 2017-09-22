#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

extern crate silabs_slstk3401a as board;

use board::hal::cmu::*;
use board::hal::gpio::*;

// LED0 = PF4
// LED1 = PF5
// BTN0 = PF6
// BTN1 = PF7

pub const PIN: usize = 4;

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Enable GPIO
    CMU.with_hfbusclken0(|r| r.set_gpio(true));

    // Set PF4 Mode = Output
    GPIOF.with_model(|r| r.set_mode(PIN, 0b0100));

    loop {
        // Toggle PF4
        GPIOF.set_douttgl(|r| r.set_douttgl(PIN, 1));
        // Delay approx 1/2 second
        for _ in 0..5_000_000 { unsafe { asm!("nop") } }
    }
}
