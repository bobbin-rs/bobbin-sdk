#![no_std]
#![allow(unused_unsafe)]

extern crate bobbin_cortexm;
extern crate stm32_common;
extern crate stm32l031x_chip as chip;

pub use bobbin_cortexm::hal::{nvic, scb};

pub use stm32_common::hal::usart;
pub use stm32_common::hal::tim_gen;
pub use stm32_common::hal::gpio;

pub mod pwr;
pub mod rcc;
pub mod clock;
//pub mod exti;
//pub mod gpio;
//pub mod usart;
// pub mod lptim;
// pub mod rtc;
// pub mod dma;
// pub mod crc;
// pub mod iwdg;
// pub mod wwdg;
// pub mod spi;
//pub mod pin;

// use chip::sig::Signal;
// use chip::gpio::{Pin, AltFn, GpioImpl, PinImpl};
// use gpio::PinExt;
// //use stm32_common::hal::gpio::PinExt;
// use core::ops::Deref;

// pub fn connect<T, S: Signal<T>, P: Deref<Target=PinImpl> + Pin<GpioImpl> + AltFn<T>>(_: T, _: S, pin: &P) {
//     pin.mode_altfn(AltFn::<T>::alt_fn(pin));
// }