#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm)]

pub extern crate sam_common;
pub use sam_common::bobbin_bits;
pub use sam_common::bobbin_mcu;
pub use sam_common::bobbin_hal;
pub use sam_common::bobbin_sys;
pub use sam_common::nvic;
pub use sam_common::scb;
pub use sam_common::systick;

pub mod ext;
pub mod gclk;
pub mod nvmctrl;
pub mod pm;
pub mod sysctrl;
pub mod wdt;
pub mod rtc;
pub mod dmac;
pub mod adc;
pub mod dac;
pub mod tcc;
pub mod tc;
pub mod port;
pub mod sercom;
pub mod sig;
pub mod pin;
pub mod irq;
pub mod clock;


#[derive(Debug, Default)]
pub struct Samd21 {}

impl ::bobbin_mcu::mcu::Mcu for Samd21 {
    fn id(&self) -> &'static str { "SAMD21" }
}

pub type Mcu = Samd21;

