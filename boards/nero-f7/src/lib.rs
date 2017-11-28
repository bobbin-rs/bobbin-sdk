#![no_std]
#![feature(lang_items)]

#![feature(asm)]

extern crate r0;

extern crate log;

#[macro_use] pub mod itm;
#[macro_use] pub mod console;
#[macro_use] pub mod logger;

extern crate stm32f74x;
pub use stm32f74x::{chip, hal, cortexm, common};
pub mod icm20602;

pub mod exceptions;
#[cfg(target_os="none")]
pub mod lang_items;

pub mod pin;
pub mod clock;
pub mod led;
pub mod buz;
pub mod tim;

// pub fn delay(ms: u32) {
//     for _ in 0..10_000_000 {
//         unsafe { asm!("nop") }
//     }
// }

pub use tim::delay;

pub fn init() {
    clock::init();
    led::init();
    buz::init();
    tim::init();
    console::init();
}