#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f031k6 as board;

use board::driver::gpio::{self, GPIOB};

// LED = PB3

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
            
    let led = gpio::pin(GPIOB, 3).into_digital_output();

    loop {
        led.set(true);
        delay(100_000);
        led.set(false);
        delay(100_000);
    }
}

fn delay(n: usize) {
    for _ in 0..n {
        unsafe { asm!("nop") }
    }
}
