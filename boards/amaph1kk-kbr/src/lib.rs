#![no_std]
#![feature(lang_items)]
#![feature(asm)]

extern crate r0;

#[macro_use]
pub mod console;

extern crate apollo2;
pub use apollo2::{chip, hal, cortexm, common};

pub mod exceptions;
#[cfg(target_os="none")]
pub mod lang_items;

// pub mod clock;
pub mod led;
pub mod btn;
pub mod tim;

pub use tim::delay;

// pub fn delay(n: u32) {
//     for _ in 0..(n * 5_000) {
//         unsafe { asm!("nop") }
//     }
// }

pub fn init() {
    // clock::init();
    led::init();
    btn::init();
    tim::init();
    console::init();
}