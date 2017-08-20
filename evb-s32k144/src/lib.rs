#![no_std]
#![feature(lang_items)]

#![feature(asm, naked_functions)]

extern crate r0;

extern crate log;

extern crate s32;
pub use s32::{common, chip, hal};

#[macro_use] pub mod itm;
#[macro_use] pub mod console;
#[macro_use] pub mod logger;

pub mod exceptions;
pub mod lang_items;

pub mod pin;
pub mod clock;
pub mod led;
pub mod btn;
pub mod pot;
pub mod tim;
//pub mod serial;
//pub mod can;
//pub mod spi;
pub mod uja1169;

pub use tim::delay;

// pub fn delay(n: u32) {
//     for _ in 0..10_000*n {
//         unsafe { asm!("nop") }
//     }
// }

pub fn init() {
    clock::init();
    led::init();
    btn::init();
    pot::init();
    tim::init();
    console::init();
}