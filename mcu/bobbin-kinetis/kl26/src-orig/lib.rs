#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate kinetis_common;
pub use kinetis_common::*;

pub extern crate bobbin_cortexm;
pub use bobbin_cortexm::chip::exc;
pub use bobbin_cortexm::chip::nvic;
pub use bobbin_cortexm::chip::scb;
pub use bobbin_cortexm::chip::systick;
pub extern crate kinetis_common;

pub mod periph;
pub mod hal;
pub mod mcu;

pub use mcu::ftfa;
pub use mcu::osc;
pub use mcu::sim;
pub use mcu::mcg;
pub use mcu::rcm;
pub use mcu::dmamux;
pub use mcu::dma;
pub use mcu::tpm;
pub use mcu::pit;
pub use mcu::lptmr;
pub use mcu::spi;
pub use mcu::i2c;
pub use mcu::port;
pub use mcu::gpio;
pub use mcu::uart0;
pub use mcu::uart;
pub use mcu::adc;
pub use mcu::pin;
pub use mcu::sig;
pub use mcu::irq;
pub use mcu::*;

pub mod clock;
pub use clock::*;

