#![no_std]
#![feature(use_extern_macros)]

extern crate panic_abort;
extern crate cortex_m_rt;
%imports%
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

pub type Mcu = mcu::%mcu_type%;
pub type Board = %board_type%;

pub struct %board_type% {}

impl bobbin_sys::board::Board for %board_type% {
    fn id(&self) -> &'static str { "%board%" }    
}

#[exception]
fn DefaultHandler(_irqn: i16) {
    bobbin_sys::irq_dispatch::IrqDispatcher::<Mcu>::handle_exception();
}

#[exception]
fn SysTick() {
    bobbin_sys::tick::Tick::tick();
}

#[exception]
fn PendSV() {
    bobbin_sys::pend::Pend::pend();
}