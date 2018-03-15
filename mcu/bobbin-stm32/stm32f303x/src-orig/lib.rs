#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate stm32_common;
pub use stm32_common::*;

pub extern crate bobbin_cortexm;
pub use bobbin_cortexm::chip::exc;
pub use bobbin_cortexm::chip::nvic;
pub use bobbin_cortexm::chip::scb;
pub use bobbin_cortexm::chip::systick;
pub use bobbin_cortexm::chip::mpu;
pub use bobbin_cortexm::chip::fpu;
pub use bobbin_cortexm::chip::dcb;
pub use bobbin_cortexm::chip::itm;
pub extern crate stm32_common;

pub mod periph;
pub mod hal;
pub mod mcu;

pub use mcu::rcc;
pub use mcu::syscfg;
pub use mcu::flash;
pub use mcu::pwr;
pub use mcu::rtc;
pub use mcu::iwdg;
pub use mcu::wwdg;
pub use mcu::crc;
pub use mcu::exti;
pub use mcu::gpio;
pub use mcu::usart;
pub use mcu::i2c;
pub use mcu::spi;
pub use mcu::adc;
pub use mcu::c_adc;
pub use mcu::dac;
pub use mcu::tim_bas;
pub use mcu::tim_gen;
pub use mcu::tim_adv;
pub use mcu::dma;
pub use mcu::pin;
pub use mcu::sig;
pub use mcu::irq;
pub use mcu::*;

pub mod clock;
pub use clock::*;

