#![no_std]

extern crate bobbin_common;
extern crate bobbin_cortexm;
extern crate samd21_chip as chip;

pub use bobbin_cortexm::hal::{nvic, scb};

pub mod clock;
pub mod port;
pub mod gclk;
pub mod pm;
pub mod tc;
pub mod sercom;

// pub mod port;
// pub mod usart;
// // pub mod spi;
// // pub mod i2c;
// pub mod tc;
// pub mod tcc;
// pub mod adc;