#![no_std]
#![feature(lang_items)]

#![feature(asm)]

extern crate r0;


extern crate kl26;
pub use kl26::{chip, hal, cortexm, common};

#[macro_use]
pub mod console;

// #[macro_use]
// pub mod logger;

pub mod exceptions;
#[cfg(target_os="none")]
pub mod lang_items;

pub mod clock;
// pub mod led;
// pub mod btn;
//pub mod pin;
pub mod tim;

pub use tim::delay;

// pub fn delay(n: u32) {
//     for _ in 0..25_000*n {
//         unsafe { asm!("nop") }
//     }
// }

pub fn init() {
    clock::init();
    // led::init();
    // btn::init();
    tim::init();
    console::init();
}