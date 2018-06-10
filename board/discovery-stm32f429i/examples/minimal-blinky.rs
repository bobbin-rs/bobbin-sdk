#![no_std]
#![no_main]
#![feature(asm)]

extern crate discovery_stm32f429i as board;

// LED0 = PG13;

use board::mcu::rcc::*;
use board::mcu::gpio::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    RCC.with_ahb1enr(|r| r.set_gpiogen(1));
    GPIOG.with_moder(|r| r.set_moder(13, 0b01));

    loop {
        GPIOG.set_bsrr(|r| r.set_bs(13, 1));

        // Delay approx 1/2 second
        for _ in 0..4_000_000 { unsafe { asm!("nop") } }

        // Reset Set PB0
        GPIOG.set_bsrr(|r| r.set_br(13, 1));

        // Delay approx 1/2 second
        for _ in 0..4_000_000 { unsafe { asm!("nop") } }
    }
}