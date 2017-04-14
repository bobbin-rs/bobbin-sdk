#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(asm)]

extern crate r0;
extern crate compiler_builtins;

extern crate stm32f3;
pub use stm32f3::{chip, hal, driver};

pub mod exceptions;
pub mod lang_items;

pub mod led;
pub mod btn;
pub mod pin;
pub mod timer;
pub mod usart;

pub use timer::delay;

pub fn init() {
    hal::clock::enable_pll_external_mode();
    timer::init();
}