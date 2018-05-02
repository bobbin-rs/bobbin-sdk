#![no_std]
#![feature(asm, use_extern_macros)]

extern crate panic_abort;
extern crate cortex_m_rt;
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

cortex_m_rt::default_handler!(bobbin_sys::irq_dispatch::IrqDispatcher::<Mcu>::handle_exception);
cortex_m_rt::exception!(SYS_TICK, bobbin_sys::tick::Tick::tick);