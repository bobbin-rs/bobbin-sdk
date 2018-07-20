#![no_std]
#![feature(use_extern_macros)]

extern crate panic_abort;
extern crate cortex_m_rt;
pub extern crate samd21 as mcu;


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

pub type Mcu = mcu::Samd21;
pub type Board = FeatherM0;

pub struct FeatherM0 {}

impl bobbin_sys::board::Board for FeatherM0 {
    fn id(&self) -> &'static str { "feather-m0" }    
}

cortex_m_rt::default_handler!(bobbin_sys::irq_dispatch::IrqDispatcher::<Mcu>::handle_exception);
cortex_m_rt::exception!(SYS_TICK, bobbin_sys::tick::Tick::tick);
cortex_m_rt::exception!(PENDSV, bobbin_sys::pend::Pend::pend);