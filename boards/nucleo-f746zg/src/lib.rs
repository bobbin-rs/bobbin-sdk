#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics, const_fn, align_offset, offset_to)]

#[cfg(target_os="none")]
pub extern crate cortex_m_rt;
pub extern crate stm32f74x as mcu;
pub extern crate bobbin_sys;

pub use bobbin_sys::{print, println};

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
    System::init(|| {
        ::cache::init();
        ::clock::init();
        ::console::init();
        ::led::init();
        ::btn::init();
        ::delay::init();
        #[cfg(feature="logger")]
        Logger::init();          
    })
}

pub type System = sys::System<
        Mcu,
        Clock,
>;

pub type Mcu = mcu::Stm32f74x;
pub type Clock = clock::SystemClock;
pub type Memory = bobbin_sys::memory::Memory;
pub type Heap = bobbin_sys::heap::Heap;
#[cfg(feature="logger")]
pub type Logger = mcu::bobbin_common::logger::Logger;
pub type Dispatcher = mcu::dispatch::Dispatcher<mcu::dispatch::ExcHandlers8>;

#[cfg(target_os="none")]
default_handler!(Dispatcher::handle_exception);

#[derive(Debug, Default)]
pub struct NucleoF746zg {}

impl common::board::Board for NucleoF746zg {
   type Mcu = mcu::Stm32f74x;
   fn id(&self) -> &'static str { "nucleo-f746zg" }
   fn mcu(&self) -> Self::Mcu { Self::Mcu::default() }
}

pub const fn board() -> NucleoF746zg { NucleoF746zg{} }

pub type Board = NucleoF746zg;