#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics, const_fn)]

#[cfg(target_os="none")]
pub extern crate cortex_m_rt;
pub extern crate bobbin_sys;
pub extern crate samd21 as mcu;

pub use bobbin_sys::{system, memory, heap, print, println};
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
    })
}

pub type System = system::System<
        Mcu,
        Clock,
        Dispatcher,
>;

pub type Mcu = mcu::Samd21;
pub type Clock = clock::SystemClock;
pub type Memory = memory::Memory;
pub type Heap = heap::Heap;
pub type Dispatcher = mcu::dispatch::Dispatcher<mcu::dispatch::ExcHandlers8>;

#[cfg(target_os="none")]
default_handler!(Dispatcher::handle_exception);

#[derive(Debug, Default)]
pub struct ArduinoZero {}

impl common::board::Board for ArduinoZero {
   type Mcu = mcu::Samd21;
   fn id(&self) -> &'static str { "arduino-zero" }
   fn mcu(&self) -> Self::Mcu { Self::Mcu::default() }
}

pub const fn board() -> ArduinoZero { ArduinoZero{} }

pub type Board = ArduinoZero;
