#![no_std]

extern crate bobbin_cortexm;
extern crate kinetis_common;
extern crate kl26_chip as chip;

pub use bobbin_cortexm::hal::*;

pub mod sim;
pub mod clock;

pub mod port {
    pub use chip::port::*;    
    pub use kinetis_common::hal::port::*;
    pub use sim::SimEnabled;
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
    pub use sim::SimEnabled;
}


pub mod uart {
    pub use chip::uart::*;
    pub use kinetis_common::hal::uart::*;
    pub use sim::SimEnabled;
}

pub mod uart0 {
    pub use chip::uart0::*;
    pub use kinetis_common::hal::uart0::*;
    pub use sim::{SimEnabled, SimSrc};
}


pub mod pit {
    pub use chip::pit::*;
    pub use kinetis_common::hal::pit::*;
    pub use sim::SimEnabled;
}