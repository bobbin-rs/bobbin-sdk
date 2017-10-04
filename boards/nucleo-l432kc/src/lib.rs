#![no_std]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(asm)]

extern crate r0;

#[macro_use]
pub mod console;

extern crate stm32l432x;
pub use stm32l432x::{chip, hal, common, cortexm};

pub mod exceptions;
#[cfg(target_os="none")]
pub mod lang_items;

// pub mod pin;
pub mod clock;
pub mod led;
pub mod btn;
pub mod tim;

pub use tim::delay;

// pub fn delay(n: u32) {
//     for _ in 0..(n * 100_000) {
//         unsafe { asm!("nop") }
//     }
// }

pub fn init() {
    clock::init();
    led::init();
    btn::init();
    tim::init();
    console::init();
}