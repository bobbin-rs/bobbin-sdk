#![no_std]

extern crate bobbin_cortexm;
extern crate stm32_common;
extern crate stm32f302x_chip as chip;

pub use bobbin_cortexm::hal::{nvic, scb};

pub mod rcc;
pub mod clock;
// pub mod iwdg;
// pub mod wwdg;
// pub mod crc;
// pub mod dma;
// pub mod rtc;
// pub mod pwr;

pub mod gpio {
    pub use chip::gpio::*;
    pub use stm32_common::hal::gpio::*;
}

pub mod usart {
    pub use chip::usart::*;
    pub use stm32_common::hal::usart::*;
}

pub mod tim {
    pub use chip::tim_gen::*;
    pub use stm32_common::hal::tim_gen::*;
}