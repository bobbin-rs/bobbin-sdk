#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate kinetis_common;
pub use kinetis_common::*;

pub extern crate bobbin_cortexm;
pub use bobbin_cortexm::chip::exc;
pub use bobbin_cortexm::chip::nvic;
pub use bobbin_cortexm::chip::scb;
pub use bobbin_cortexm::chip::systick;
pub use bobbin_cortexm::chip::fpu;
pub use bobbin_cortexm::chip::dcb;
pub use bobbin_cortexm::chip::itm;
pub extern crate kinetis_common;

pub mod periph;
pub mod hal;
pub mod mcu;

pub use mcu::sim;
pub use mcu::mcg;
pub use mcu::mpu;
pub use mcu::osc;
pub use mcu::rcm;
pub use mcu::enet;
pub use mcu::crc;
pub use mcu::wdog;
pub use mcu::dmamux;
pub use mcu::edma;
pub use mcu::ftm;
pub use mcu::pit;
pub use mcu::lptmr;
pub use mcu::spi;
pub use mcu::i2c;
pub use mcu::uart;
pub use mcu::usb;
pub use mcu::flexcan;
pub use mcu::gpio;
pub use mcu::port;
pub use mcu::adc;
pub use mcu::pin;
pub use mcu::sig;
pub use mcu::irq;
pub use mcu::*;

pub mod clock;
pub use clock::*;

