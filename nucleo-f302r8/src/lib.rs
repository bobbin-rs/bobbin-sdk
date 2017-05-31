#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(asm)]

extern crate r0;
extern crate compiler_builtins;

extern crate log;

#[macro_use]
pub mod console;

#[macro_use]
pub mod logger;

extern crate stm32f302x;
pub use stm32f302x::{chip, hal};

pub mod exceptions;
pub mod lang_items;

pub mod led;
pub mod btn;
pub mod pin;
pub mod tim;

pub use tim::delay;

pub fn init() {
    hal::clock::enable_pll_external_mode();
    led::init();
    btn::init();
    console::CONSOLE.init();
}