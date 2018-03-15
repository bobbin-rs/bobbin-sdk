#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate stm32_common;
pub use stm32_common::*;

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

pub use mcu::rcc;
pub use mcu::flash;
pub use mcu::pwr;
pub use mcu::syscfg;
pub use mcu::dbg;
pub use mcu::ethernet_mac;
pub use mcu::ethernet_mmc;
pub use mcu::ethernet_ptp;
pub use mcu::ethernet_dma;
pub use mcu::dac;
pub use mcu::sdmmc;
pub use mcu::quadspi;
pub use mcu::cec;
pub use mcu::spdif_rx;
pub use mcu::ltdc;
pub use mcu::dma2d;
pub use mcu::hash;
pub use mcu::cryp;
pub use mcu::c_adc;
pub use mcu::dcmi;
pub use mcu::usb_fs_global;
pub use mcu::usb_fs_host;
pub use mcu::usb_fs_device;
pub use mcu::usb_fs_pwrclk;
pub use mcu::iwdg;
pub use mcu::wwdg;
pub use mcu::crc;
pub use mcu::exti;
pub use mcu::tim_bas;
pub use mcu::tim_gen;
pub use mcu::tim_adv;
pub use mcu::lptim;
pub use mcu::adc;
pub use mcu::spi;
pub use mcu::i2c;
pub use mcu::can;
pub use mcu::gpio;
pub use mcu::usart;
pub use mcu::dma;
pub use mcu::pin;
pub use mcu::sig;
pub use mcu::irq;
pub use mcu::*;

pub mod clock;
pub use clock::*;

