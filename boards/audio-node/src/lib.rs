#![no_std]
#![feature(asm, lang_items, global_allocator)]

extern crate r0;

#[macro_use]
pub mod console;

extern crate stm32l4x2;
pub use stm32l4x2::{chip, hal, common, cortexm};

pub mod exceptions;
#[cfg(target_os="none")]
pub mod lang_items;

// pub mod pin;
pub mod clock;
pub mod led;
pub mod tim;

pub use common::heap::Heap;

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

pub unsafe fn init_allocator(buf: &'static mut [u8]) {
    ALLOCATOR.init(buf);
}

pub use tim::delay;

// pub fn delay(n: u32) {
//     for _ in 0..(n * 100_000) {
//         unsafe { asm!("nop") }
//     }
// }

pub fn init() {
    clock::init();
    led::init();
    tim::init();
    console::init();
}