#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate bobbin_bits;
extern crate bobbin_mcu;
extern crate bobbin_hal;
extern crate sam_common;

pub extern crate bobbin_cortexm;
pub use bobbin_cortexm::nvic;
pub use bobbin_cortexm::scb;
pub use bobbin_cortexm::systick;

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


