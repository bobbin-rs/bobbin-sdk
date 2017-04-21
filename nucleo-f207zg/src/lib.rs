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

extern crate stm32f2;
pub use stm32f2::{chip, hal};

pub mod exceptions;
pub mod lang_items;

pub mod clock;
pub mod led;
pub mod btn;
pub mod pin;
pub mod tim;
pub mod usart;

pub use tim::delay;
// pub fn delay(n: u32) {
//     for _ in 0..100_000 * n {
//         unsafe { asm!("nop") }
//     }
// }

pub fn init() {
    clock::enable_pll_external_mode();
    console::CONSOLE.init(115_200);
}