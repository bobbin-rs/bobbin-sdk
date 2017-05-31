#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(asm)]

extern crate r0;
extern crate compiler_builtins;

#[macro_use]
pub mod console;

extern crate stm32l031x;
pub use stm32l031x::{chip, hal};

pub mod exceptions;
pub mod lang_items;

pub mod led;
pub mod btn;
//pub mod pin;
pub mod tim;

pub use tim::delay;

pub fn init() {
    hal::clock::init_pll();    
    led::init();
    btn::init();
    console::CONSOLE.init();
}