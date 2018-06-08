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
use cortex_m_rt::ExceptionFrame;

pub type Mcu = mcu::Samd21;
pub type Board = ArduinoZero;

pub struct ArduinoZero {}

impl bobbin_sys::board::Board for ArduinoZero {
    fn id(&self) -> &'static str { "arduino-zero" }    
}

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

cortex_m_rt::exception!(*, bobbin_sys::irq_dispatch::IrqDispatcher::<Mcu>::handle_exception);
cortex_m_rt::exception!(HardFault, hard_fault);
cortex_m_rt::exception!(SysTick, bobbin_sys::tick::Tick::tick);
cortex_m_rt::exception!(PendSV, bobbin_sys::pend::Pend::pend);