#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics, const_fn)]

#[cfg(target_os="none")]
pub extern crate cortex_m_rt;
pub extern crate stm32f3x as mcu;


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
pub mod sys;

pub use delay::delay;

pub fn init() -> System {    
    System::init()
}

pub type System = sys::System<
        Mcu,
        Clock,
>;

pub type Mcu = mcu::Stm32f3x;
pub type Clock = clock::SystemClock;
pub type Memory = mcu::bobbin_common::memory::Memory;
pub type Heap = mcu::bobbin_common::heap::Heap;
#[cfg(feature="logger")]
pub type Logger = mcu::bobbin_common::logger::Logger;
pub type Dispatcher = mcu::dispatch::Dispatcher<mcu::dispatch::ExcHandlers8>;

#[cfg(target_os="none")]
default_handler!(Dispatcher::handle_exception);

#[derive(Debug, Default)]
pub struct DiscoveryStm32f3 {}

impl common::board::Board for DiscoveryStm32f3 {
   type Mcu = mcu::Stm32f3x;
   fn id(&self) -> &'static str { "discovery-stm32f3" }
   fn mcu(&self) -> Self::Mcu { Self::Mcu::default() }
}

pub const fn board() -> DiscoveryStm32f3 { DiscoveryStm32f3{} }

