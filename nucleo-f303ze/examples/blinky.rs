#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f303ze as board;

use board::hal::gpio::{self, GPIOA};

// LED @ D13 = PA5

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
            
    gpio::configure(GPIOA, 1 << 5, &gpio::Config::output());    

    loop {
        gpio::set_odr(GPIOA, 1 << 5, true);
        delay(1_000_000);
        gpio::set_odr(GPIOA, 1 << 5, false);
        delay(1_000_000);
    }
}

fn delay(n: usize) {
    for _ in 0..n {
        unsafe { asm!("nop") }
    }
}
