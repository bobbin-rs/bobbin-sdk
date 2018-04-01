#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics, const_fn)]

#[cfg(target_os="none")]
pub extern crate cortex_m_rt;
pub extern crate samd21 as mcu;


pub use mcu::bobbin_common::{print, println};
pub use mcu::bobbin_common as common;

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


#[derive(Debug, Default)]
pub struct FeatherM0 {}

impl common::board::Board for FeatherM0 {
   type Mcu = mcu::Samd21;
   fn id(&self) -> &'static str { "arduino-zero" }
   fn mcu(&self) -> Self::Mcu { Self::Mcu::default() }
}

pub const fn board() -> FeatherM0 { FeatherM0{} }

