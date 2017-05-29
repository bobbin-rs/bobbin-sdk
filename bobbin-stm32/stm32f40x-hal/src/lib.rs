#![no_std]

extern crate bobbin_cortexm;
extern crate stm32_common;
extern crate stm32f40x_chip as chip;

pub use bobbin_cortexm::hal::{nvic, scb};
//pub use stm32_common::hal::gpio;
//pub use stm32_common::hal::usart;
pub use stm32_common::hal::tim_bas;
pub use stm32_common::hal::tim_gen;
pub use stm32_common::hal::tim_adv;

pub mod rcc;
pub mod clock;
pub mod usart;
pub mod dma;
pub mod gpio;
//pub mod iwdg;
//pub mod wwdg;
//pub mod crc;
//pub mod dma;
//pub mod rtc;
//pub mod pwr;


// pub mod rcc;
// pub mod gpio;
// pub mod clock;
// // pub mod nvic;
// pub mod fpu;
// // pub mod itm;
// pub mod ethernet;
// pub mod i2c;
// pub mod spi;
// pub mod usart;
// pub mod adc;
// pub mod dac;
// pub mod tim;
// //pub mod systick;
