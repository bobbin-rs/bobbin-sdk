#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics)]

pub extern crate log;
#[cfg(target_os="none")]
pub extern crate cortex_m_rt;
extern crate stm32l432x as mcu;

pub use mcu::bobbin_common::{print, println};
pub use mcu::bobbin_common as common;

#[macro_use] pub mod logger;

#[cfg(target_os="none")]
pub use cortex_m_rt::default_handler;

#[cfg(target_os="none")]
mod lang_items;
// pub mod cache;
pub mod clock;
pub mod console;
pub mod led;
pub mod btn;
pub mod tim;

// pub fn delay(n: u32) {
//     for _ in 0..(n * 10_000) { unsafe { asm!("nop") } }
// }

pub use tim::delay;

pub fn init() {    
    // cache::init();
    clock::init();
    console::init();
    led::init();
    btn::init();
    tim::init();
}

#[cfg(target_os="none")]
default_handler!(handle_exception);

pub fn handle_exception() {
    console::write_str("EXCEPTION\n");
    unsafe { asm!("bkpt") }
    loop {}
}

