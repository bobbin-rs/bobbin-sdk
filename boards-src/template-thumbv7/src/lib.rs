#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics, const_fn)]

#[cfg(target_os="none")]
pub extern crate cortex_m_rt;
pub extern crate bobbin_sys;
%imports%
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

pub type Mcu = mcu::%mcu_type%;
pub type Clock = clock::SystemClock;
pub type Memory = memory::Memory;
pub type Heap = heap::Heap;
#[cfg(feature="logger")]
pub type Logger = logger::Logger;
pub type Dispatcher = mcu::dispatch::Dispatcher<mcu::dispatch::ExcHandlers8>;

#[cfg(target_os="none")]
default_handler!(Dispatcher::handle_exception);
