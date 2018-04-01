#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate sam_common;
pub use sam_common::*;

pub use nvic;
pub use scb;
pub use systick;

pub mod periph;
pub mod ext;
pub mod mcu;

pub use mcu::gclk;
pub use mcu::nvmctrl;
pub use mcu::pm;
pub use mcu::sysctrl;
pub use mcu::wdt;
pub use mcu::rtc;
pub use mcu::dmac;
pub use mcu::adc;
pub use mcu::dac;
pub use mcu::tcc;
pub use mcu::tc;
pub use mcu::port;
pub use mcu::sercom;
pub use mcu::pin;
pub use mcu::sig;
pub use mcu::irq;
pub use mcu::Samd21;

pub mod clock;
pub use clock::*;

