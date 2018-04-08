#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics, const_fn)]

#[cfg(target_os="none")]
pub extern crate cortex_m_rt;
pub extern crate log;
pub extern crate stm32l432x as mcu;


pub use mcu::bobbin_common::{print, println};
pub use mcu::bobbin_common as common;

#[macro_use] pub mod logger;

#[cfg(target_os="none")]
pub use cortex_m_rt::default_handler;

#[cfg(target_os="none")]
mod lang_items;
pub mod cache;
pub mod clock;
pub mod console;
pub mod led;
pub mod btn;
pub mod delay;

pub use delay::delay;

pub fn init() {    
    cache::init();
    clock::init();
    console::init();
    led::init();
    btn::init();
    delay::init();
}

#[cfg(target_os="none")]
default_handler!(handle_exception);

pub fn handle_exception() {
    console::write_str("EXCEPTION\n");
    unsafe { asm!("bkpt") }
    loop {}
}

pub use mcu::bobbin_common::memory::Memory;
pub use mcu::bobbin_common::heap::Heap;

#[derive(Debug, Default)]
pub struct NucleoL432kc {}

impl common::board::Board for NucleoL432kc {
   type Mcu = mcu::Stm32l432x;
   fn id(&self) -> &'static str { "nucleo-l432kc" }
   fn mcu(&self) -> Self::Mcu { Self::Mcu::default() }
}

pub const fn board() -> NucleoL432kc { NucleoL432kc{} }

pub type Board = NucleoL432kc;