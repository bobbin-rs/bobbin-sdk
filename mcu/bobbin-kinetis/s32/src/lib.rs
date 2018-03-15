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
pub mod hal;
pub mod mcu;

pub use mcu::sim;
pub use mcu::scg;
pub use mcu::pcc;
pub use mcu::rcm;
pub use mcu::wdog;
pub use mcu::rtc;
pub use mcu::smc;
pub use mcu::crc;
pub use mcu::dmamux;
pub use mcu::edma;
pub use mcu::ftm;
pub use mcu::lpit;
pub use mcu::lptmr;
pub use mcu::flexcan;
pub use mcu::port;
pub use mcu::gpio;
pub use mcu::lpuart;
pub use mcu::lpi2c;
pub use mcu::lpspi;
pub use mcu::adc;
pub use mcu::pin;
pub use mcu::sig;
pub use mcu::irq;
pub use mcu::*;

pub mod clock;
pub use clock::*;

