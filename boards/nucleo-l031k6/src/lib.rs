#![no_std]
#![feature(lang_items)]
#![feature(asm)]

extern crate r0;

#[macro_use]
pub mod console;

extern crate stm32l031x;
pub use stm32l031x::{chip, hal, common};

pub mod exceptions;
#[cfg(target_os="none")]
pub mod lang_items;

pub mod pin;
pub mod clock;
pub mod led;
pub mod btn;
pub mod tim;

pub use tim::delay;

pub fn init() {
    clock::init();
    led::init();
    btn::init();
    tim::init();
    console::init();
}