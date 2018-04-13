#![no_std]
#![feature(asm, lang_items, global_allocator)]

extern crate r0;

#[macro_use]
pub mod console;

extern crate samd21;
pub use samd21::{chip, hal, cortexm, common};

pub mod exceptions;
#[cfg(target_os="none")]
pub mod lang_items;

pub mod clock;
pub mod led;
// // pub mod btn;
pub mod pin;
pub mod tim;
// pub mod adc;
// pub mod usart;

pub use common::heap::Heap;

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

pub unsafe fn init_allocator(buf: &'static mut [u8]) {
    ALLOCATOR.init(buf);
}

pub use tim::delay;

// pub fn delay(ms: u32) {
//     for _ in 0..ms * 10000 {
//         unsafe { asm!("nop") }
//     }
// }

pub fn init() {
    clock::init();
    tim::init();
    led::init();
    console::init();
}