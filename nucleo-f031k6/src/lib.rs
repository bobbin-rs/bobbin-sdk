#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]

extern crate r0;
extern crate compiler_builtins;

extern crate stm32l0;
pub use stm32l0::{chip, hal, driver};

pub mod lang_items;
pub mod exceptions;

pub mod led;
pub mod btn;
pub mod pin;

pub fn init() {}