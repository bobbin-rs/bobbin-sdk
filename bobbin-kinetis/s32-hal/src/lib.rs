#![no_std]

extern crate bobbin_cortexm;
extern crate kinetis_common;
extern crate s32_chip as chip;

pub use bobbin_cortexm::hal::*;

pub mod clock;
pub mod pcc;

pub mod port {
    pub use chip::port::*;    
    pub use kinetis_common::hal::port::*;
    pub use pcc::PccEnabled;
    use chip::gpio;
    use chip::sig::{SignalTx, SignalRx};
    use core::ops::Deref;

    pub trait GpioPin<T> {
        fn gpio_pin(&self) -> gpio::PinImpl;
    }

    impl<P, T> GpioPin<T> for P where P: Pin<T>, T: GpioLink {
        fn gpio_pin(&self) -> gpio::PinImpl {
            gpio::PinImpl {
                port: *self.port().gpio().deref(),
                index: self.index()
            }
        }        
    }    

    pub trait ModeTx<T, S> {
        fn mode_tx(&self, _: &S) -> &Self;
    }

    pub trait ModeRx<T, S> {
        fn mode_rx(&self, _: &S) -> &Self;
    }

    impl<P, S, T> ModeTx<T, S> for P where S: SignalTx<T>, P: Deref<Target=PinImpl> + AltFn<T> {
        fn mode_tx(&self, _: &S) -> &Self {
            self.set_mux(self.alt_fn());
            self
        }
    }

    impl<P, S, T> ModeRx<T, S> for P where S: SignalRx<T>, P: Deref<Target=PinImpl> + AltFn<T> {
        fn mode_rx(&self, _: &S) -> &Self {
            self.set_mux(self.alt_fn());
            self
        }
    }
}

pub mod gpio {
    pub use chip::gpio::*;
    pub use kinetis_common::hal::gpio::*;
    pub use pcc::PccEnabled;
}


pub mod lpuart {
    pub use chip::lpuart::*;
    pub use kinetis_common::hal::lpuart::*;
    pub use pcc::{PccEnabled, PccClockSource};
}

pub mod lpit {
    pub use chip::lpit::*;
    pub use kinetis_common::hal::lpit::*;
    pub use pcc::{PccEnabled, PccClockSource};
}

pub mod flexcan {
    pub use chip::flexcan::*;
    pub use kinetis_common::hal::flexcan::*;
    pub use pcc::{PccEnabled, PccClockSource};
}