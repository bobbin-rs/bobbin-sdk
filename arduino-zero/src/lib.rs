#![no_std]
#![feature(lang_items)]

#![feature(asm)]

extern crate r0;

#[macro_use]
pub mod console;

extern crate samd21;
pub use samd21::{chip, hal};

pub mod exceptions;
pub mod lang_items;

pub mod clock;
pub mod led;
// // pub mod btn;
// pub mod pin;
pub mod tim;
// pub mod usart;

pub use tim::delay;

// pub fn delay(ms: u32) {
//     for _ in 0..ms * 10000 {
//         unsafe { asm!("nop") }
//     }
// }

pub fn init() {
    clock::init();
    tim::init();
    led::init();
    console::init();
}