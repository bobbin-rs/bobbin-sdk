#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(asm, naked_functions)]

extern crate r0;
extern crate compiler_builtins;

extern crate log;

extern crate s32k144;
pub use s32k144::{chip, hal};

#[macro_use]
pub mod console;

#[macro_use]
pub mod logger;

pub mod exceptions;
pub mod lang_items;

pub mod clock;
pub mod pin;
pub mod led;
pub mod sw;
pub mod timer;
pub mod serial;

pub use timer::delay;

// pub fn delay(n: u32) {
//     for _ in 0..10_000*n {
//         unsafe { asm!("nop") }
//     }
// }

pub fn init() {
    clock::init();
    console::init();
}