#![no_std]
#![feature(lang_items)]
#![feature(alloc, core_intrinsics, offset_to, allocator_api, const_fn, global_allocator)]
#![feature(asm)]

extern crate r0;
extern crate log;
extern crate alloc;

#[macro_use] pub mod itm;
#[macro_use] pub mod console;
#[macro_use] pub mod logger;

extern crate stm32f74x;
pub use stm32f74x::{chip, hal, cortexm, common};

pub mod exceptions;
#[cfg(target_os="none")]
pub mod lang_items;

pub mod pin;
pub mod clock;
pub mod led;
pub mod btn;
pub mod tim;

pub mod heap;

#[global_allocator]
static ALLOCATOR: heap::Heap = heap::Heap::empty();

// pub fn delay(ms: u32) {
//     for _ in 0..1_000_000 {
//         unsafe { asm!("nop") }
//     }
// }

pub use tim::delay;

pub fn init() {
    clock::init();
    led::init();
    btn::init();
    tim::init();
    console::init();
}