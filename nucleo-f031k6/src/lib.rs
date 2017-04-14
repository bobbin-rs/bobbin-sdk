#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(asm)]

extern crate r0;
extern crate compiler_builtins;

extern crate stm32l0;
pub use stm32l0::{chip, hal, driver};

pub mod exceptions;
pub mod lang_items;

pub mod led;
pub mod btn;
pub mod pin;
pub mod timer;
pub mod usart;

pub use timer::delay;

pub fn init() {
    hal::clock::init_pll();
    timer::init();
}