#![no_std]
#![feature(use_extern_macros)]

extern crate panic_abort;
extern crate cortex_m_rt;
use cortex_m_rt::exception;

pub extern crate k64 as mcu;

pub mod prelude;
pub mod led;
pub mod btn;
pub mod sys;

pub use mcu::bobbin_bits;
pub use mcu::bobbin_mcu;
pub use mcu::bobbin_hal;
pub use mcu::bobbin_sys;

pub use bobbin_sys::{print, println, abort};
pub use sys::init;

pub type Mcu = mcu::K64;
pub type Board = FrdmK64f;

pub struct FrdmK64f {}

impl bobbin_sys::board::Board for FrdmK64f {
    fn id(&self) -> &'static str { "frdm-k64f" }    
}

#[exception]
fn DefaultHandler(irqn: i16) {
    bobbin_sys::irq_dispatch::IrqDispatcher::<Mcu>::handle_irq(irqn);
}

#[exception]
fn SysTick() {
    bobbin_sys::tick::Tick::tick();
}

#[exception]
fn PendSV() {
    bobbin_sys::pend::Pend::pend();
}
