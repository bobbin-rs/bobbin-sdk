#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate stm32_common;
pub use stm32_common::*;

pub use nvic;
pub use scb;
pub use systick;
pub use mpu;
pub use fpu;
pub use dcb;
pub use itm;
pub use dwt;

pub mod periph;
pub mod ext;
pub mod mcu;

pub use mcu::flash;
pub use mcu::pwr;
pub use mcu::rcc;
pub use mcu::syscfg;
pub use mcu::c_adc;
pub use mcu::dac;
pub use mcu::rng;
pub use mcu::iwdg;
pub use mcu::wwdg;
pub use mcu::crc;
pub use mcu::rtc;
pub use mcu::lptim;
pub use mcu::exti;
pub use mcu::dma;
pub use mcu::i2c;
pub use mcu::tim_adv;
pub use mcu::tim_bas;
pub use mcu::tim_gen;
pub use mcu::gpio;
pub use mcu::usart;
pub use mcu::lpuart;
pub use mcu::spi;
pub use mcu::adc;
pub use mcu::pin;
pub use mcu::sig;
pub use mcu::irq;
pub use mcu::Stm32l432x;

pub mod clock;
pub use clock::*;

