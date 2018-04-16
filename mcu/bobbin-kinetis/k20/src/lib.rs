#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate kinetis_common;
pub use kinetis_common::*;

pub use exc;
pub use nvic;
pub use scb;
pub use systick;
pub use mpu;
pub use fpu;
pub use dcb;
pub use itm;

pub mod periph;
pub mod ext;
pub mod mcu;

pub use mcu::sim;
pub use mcu::mcg;
pub use mcu::osc;
pub use mcu::crc;
pub use mcu::dmamux;
pub use mcu::edma;
pub use mcu::ftm;
pub use mcu::pit;
pub use mcu::spi;
pub use mcu::i2c;
pub use mcu::uart;
pub use mcu::gpio;
pub use mcu::port;
pub use mcu::pin;
pub use mcu::sig;
pub use mcu::irq;
pub use mcu::K20;

pub mod clock;
pub use clock::*;

