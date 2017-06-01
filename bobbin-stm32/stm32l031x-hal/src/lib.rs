#![no_std]
#![allow(unused_unsafe)]

extern crate bobbin_cortexm;
extern crate stm32_common;
extern crate stm32l031x_chip as chip;

pub use bobbin_cortexm::hal::{nvic, scb};

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

pub mod gpio {
    pub use chip::gpio::*;
    pub use stm32_common::hal::gpio::*;
    use chip::sig::{SignalTx, SignalRx};
    use core::ops::Deref;

    pub trait ModeTx<T, S> {
        fn mode_tx(&self, _: &S) -> &Self;
    }

    pub trait ModeRx<T, S> {
        fn mode_rx(&self, _: &S) -> &Self;
    }

    impl<P, S, T> ModeTx<T, S> for P where S: SignalTx<T>, P: Deref<Target=PinImpl> + AltFn<T> {
        fn mode_tx(&self, _: &S) -> &Self {
            self.mode_altfn(self.alt_fn());
            self
        }
    }

    impl<P, S, T> ModeRx<T, S> for P where S: SignalRx<T>, P: Deref<Target=PinImpl> + AltFn<T> {
        fn mode_rx(&self, _: &S) -> &Self {
            self.mode_altfn(self.alt_fn());
            self
        }
    }

}

pub mod usart {
    pub use chip::usart::*;
    pub use stm32_common::hal::usart::*;
}

pub mod tim {
    pub use chip::tim_gen::*;
    pub use stm32_common::hal::tim_gen::*;
}