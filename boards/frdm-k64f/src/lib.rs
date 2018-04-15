#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics, const_fn)]

#[cfg(target_os="none")]
pub extern crate cortex_m_rt;
pub extern crate k64 as mcu;
pub extern crate bobbin_sys;

pub use bobbin_sys::{system, memory, heap, print, println};
#[cfg(feature="logger")]
pub use bobbin_sys::logger;

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
// pub mod sys;

pub use delay::delay;

pub fn init() -> System {    
    system::System::init(|| {
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
pub type System = system::System<
        Mcu,
        Clock,
        Dispatcher,
>;

pub type Mcu = mcu::K64;
pub type Clock = clock::SystemClock;
pub type Memory = memory::Memory;
pub type Heap = heap::Heap;
#[cfg(feature="logger")]
pub type Logger = logger::Logger;
pub type Dispatcher = mcu::dispatch::Dispatcher<mcu::dispatch::ExcHandlers8>;

#[cfg(target_os="none")]
default_handler!(Dispatcher::handle_exception);

#[derive(Debug, Default)]
pub struct FrdmK64f {}

impl common::board::Board for FrdmK64f {
   type Mcu = mcu::K64;
   fn id(&self) -> &'static str { "frdm-k64f" }
   fn mcu(&self) -> Self::Mcu { Self::Mcu::default() }
}

pub const fn board() -> FrdmK64f { FrdmK64f{} }

pub type Board = FrdmK64f;