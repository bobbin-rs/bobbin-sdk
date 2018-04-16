#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate bobbin_bits;
extern crate bobbin_mcu;
extern crate bobbin_hal;
extern crate stm32_common;


pub mod ext;
pub mod rcc;
pub mod syscfg;
pub mod flash;
pub mod pwr;
pub mod rtc;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod exti;
pub mod gpio;
pub mod usart;
pub mod i2c;
pub mod spi;
pub mod adc;
pub mod c_adc;
pub mod dac;
pub mod tim_bas;
pub mod tim_gen;
pub mod tim_adv;
pub mod dma;
pub mod sig;
pub mod pin;
pub mod irq;

pub mod clock;

