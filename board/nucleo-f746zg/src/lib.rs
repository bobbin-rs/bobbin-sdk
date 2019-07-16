#![no_std]

extern crate panic_abort;
extern crate cortex_m_rt;
use cortex_m_rt::exception;

pub extern crate stm32f74x as mcu;

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

pub type Mcu = mcu::Stm32f74x;
pub type Board = NucleoF746zg;

pub struct NucleoF746zg {}

impl bobbin_sys::board::Board for NucleoF746zg {
    fn id(&self) -> &'static str { "nucleo-f746zg" }    
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
