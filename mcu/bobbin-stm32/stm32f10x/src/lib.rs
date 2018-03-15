#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate stm32_common;
pub use stm32_common::*;

pub use exc;
pub use nvic;
pub use scb;
pub use systick;
pub use mpu;
pub use dcb;
pub use itm;

pub mod periph;
pub mod hal;
pub mod mcu;

pub use mcu::rcc;
pub use mcu::flash;
pub use mcu::pwr;
pub use mcu::syscfg;
pub use mcu::dbg;
pub use mcu::bkp;
pub use mcu::fsmc;
pub use mcu::afio;
pub use mcu::c_adc;
pub use mcu::iwdg;
pub use mcu::wwdg;
pub use mcu::crc;
pub use mcu::exti;
pub use mcu::tim_gen;
pub use mcu::tim_adv;
pub use mcu::adc;
pub use mcu::spi;
pub use mcu::i2c;
pub use mcu::gpio;
pub use mcu::usart;
pub use mcu::dma;
pub use mcu::pin;
pub use mcu::sig;
pub use mcu::irq;
pub use mcu::*;

pub mod clock;
pub use clock::*;

