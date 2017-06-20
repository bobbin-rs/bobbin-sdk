#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(asm)]

extern crate r0;
extern crate compiler_builtins;

extern crate log;

extern crate k64;
pub use k64::{chip, hal};

#[macro_use]
pub mod console;

#[macro_use]
pub mod logger;

pub mod exceptions;
pub mod lang_items;

pub mod clock;
pub mod led;
pub mod sw;
pub mod pin;
pub mod tim;
pub mod uart;

pub use tim::delay;

// pub fn delay(n: u32) {
//     for _ in 0..25_000*n {
//         unsafe { asm!("nop") }
//     }
// }

pub fn init() {
    clock::init();
    //console::CONSOLE.init(115_200);
}