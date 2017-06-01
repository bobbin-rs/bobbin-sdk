#![no_std]

extern crate bobbin_cortexm;
extern crate stm32_common;
extern crate stm32f42x_chip as chip;

pub use bobbin_cortexm::hal::{nvic, scb};

pub mod rcc;
pub mod clock;

pub mod gpio {
    pub use chip::gpio::*;
    pub use stm32_common::hal::gpio::*;
    pub use rcc::RccEnabled;
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
    pub use chip::usart_f24::*;
    pub use stm32_common::hal::usart_f24::*;
    pub use rcc::RccEnabled;
}

pub mod tim {
    pub use chip::tim_gen::*;
    pub use stm32_common::hal::tim_gen::*;
    pub use rcc::RccEnabled;
}