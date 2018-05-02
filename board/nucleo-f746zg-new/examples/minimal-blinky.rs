#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;

// LED0 = PB0;

use board::mcu::rcc::*;
use board::mcu::gpio::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    RCC.with_ahb1enr(|r| r.set_gpioben(1));

    // Set PB3 Mode = Output
    GPIOB.with_moder(|r| r.set_moder(0, 0b01));    

    loop {
        // Set PB3
        GPIOB.set_bsrr(|r| r.set_bs(0, 1));
        // Delay approx 1/2 second
        for _ in 0..4_000_000 { unsafe { asm!("nop") } } 
        // Reset Set PB0
        GPIOB.set_bsrr(|r| r.set_br(0, 1));
        // Delay approx 1/2 second
        for _ in 0..4_000_000 { unsafe { asm!("nop") } }
    }
}