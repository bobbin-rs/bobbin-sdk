#![no_std]
#![feature(use_extern_macros)]

extern crate panic_abort;
extern crate cortex_m_rt;
pub extern crate stm32f42x as mcu;

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

pub type Mcu = mcu::Stm32f42x;
pub type Board = NucleoF429zi;

pub struct NucleoF429zi {}

impl bobbin_sys::board::Board for NucleoF429zi {
    fn id(&self) -> &'static str { "nucleo-f429zi" }    
}

cortex_m_rt::default_handler!(bobbin_sys::irq_dispatch::IrqDispatcher::<Mcu>::handle_exception);
cortex_m_rt::exception!(SYS_TICK, bobbin_sys::tick::Tick::tick);
cortex_m_rt::exception!(PENDSV, bobbin_sys::pend::Pend::pend);