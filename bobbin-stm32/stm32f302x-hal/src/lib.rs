#![no_std]

extern crate bobbin_cortexm;
extern crate stm32_common;
extern crate stm32f302x_chip as chip;

pub use bobbin_cortexm::hal::{nvic, scb};
pub use stm32_common::hal::usart;
pub use stm32_common::hal::tim_bas;
pub use stm32_common::hal::tim_gen;
pub use stm32_common::hal::tim_adv;

pub mod rcc;
pub mod clock;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod dma;
pub mod rtc;
pub mod pwr;
//pub mod pin;
pub mod gpio;