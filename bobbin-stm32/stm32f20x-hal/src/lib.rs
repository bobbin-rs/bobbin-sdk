#![no_std]

extern crate bobbin_cortexm;
extern crate stm32_common;
extern crate stm32f20x_chip as chip;

pub use bobbin_cortexm::hal::{nvic, scb};

pub mod rcc;
pub mod clock;
//pub mod usart;

pub mod gpio {
    pub use chip::gpio::*;
    pub use stm32_common::hal::gpio::*;
}

pub mod usart {
    pub use chip::usart_f24::*;
    pub use stm32_common::hal::usart_f24::*;
}

pub mod tim {
    pub use chip::tim_gen::*;
    pub use stm32_common::hal::tim_gen::*;
}
