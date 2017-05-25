#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(asm)]

extern crate r0;
extern crate compiler_builtins;

#[macro_use]
pub mod console;

extern crate samd21;
pub use samd21::{chip, hal};

pub mod exceptions;
pub mod lang_items;

pub mod led;
// pub mod btn;
pub mod pin;
pub mod tim;
pub mod usart;

pub use tim::delay;

pub fn init() {
    hal::clock::run_48mhz();
    usart::init(115_200);
    console::init();
}