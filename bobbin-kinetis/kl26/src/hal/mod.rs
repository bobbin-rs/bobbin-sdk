#![allow(non_camel_case_types)]

pub use bobbin_cortexm::hal::*;

pub mod sim;
pub mod clock;

pub mod port {
    pub use chip::port::*;    
    pub use kinetis_common::hal::port::*;
    pub use super::sim::SimEnabled;
    use chip::gpio;
    use chip::sig::{SignalTx, SignalRx, SignalTpm};
    //use core::ops::Deref;

    pub trait GpioPin<PIN_ID, GPIO_ID> {
        fn gpio_pin(&self) -> gpio::Pin<PIN_ID, GPIO_ID>;
    }

    impl<PIN_ID, PORT_ID, GPIO_ID> GpioPin<PIN_ID, GPIO_ID> for Pin<PIN_ID, PORT_ID>
     where PIN_ID: Copy, Periph<PORT_ID>: LinkGpio<gpio::Periph<GPIO_ID>> {
        fn gpio_pin(&self) -> gpio::Pin<PIN_ID, GPIO_ID> {
            gpio::Pin {
                port: self.port.gpio(),
                index: self.index,
                id: self.id,
            }
        }
    }

    pub trait ModeTx<T, S> {
        fn mode_tx(&self, _: &S) -> &Self;
    }

    pub trait ModeRx<T, S> {
        fn mode_rx(&self, _: &S) -> &Self;
    }

    pub trait ModeTpm<T, S> {
        fn mode_tpm(&self, _: &S) -> &Self;
    }


    impl<P, O, S, T> ModeTx<T, S> for Pin<P, O> where S: SignalTx<T>, P: AltFn<T> {
        fn mode_tx(&self, _: &S) -> &Self {
            self.set_mux(self.id.alt_fn());
            self
        }
    }

    impl<P, O, S, T> ModeRx<T, S> for Pin<P, O> where S: SignalRx<T>, P: AltFn<T> {
        fn mode_rx(&self, _: &S) -> &Self {
            self.set_mux(self.id.alt_fn());
            self
        }
    }    

    impl<P, O, S, T> ModeTpm<T, S> for Pin<P, O> where S: SignalTpm<T>, P: AltFn<T> {
        fn mode_tpm(&self, _: &S) -> &Self {
            self.set_mux(self.id.alt_fn());
            self
        }
    }        

}

pub mod gpio {
    pub use chip::gpio::*;
    pub use kinetis_common::hal::gpio::*;
    pub use super::sim::SimEnabled;
}


pub mod uart {
    pub use chip::uart::*;
    pub use kinetis_common::hal::uart::*;
    pub use super::sim::SimEnabled;
}

pub mod uart0 {
    pub use chip::uart0::*;
    pub use kinetis_common::hal::uart0::*;
    pub use super::sim::{SimEnabled, SimSrc};
}

pub mod tpm {
    pub use chip::tpm::*;
    pub use kinetis_common::hal::tpm::*;
    pub use super::sim::{SimEnabled, SimSrc};
}

pub mod pit {
    pub use chip::pit::*;
    pub use kinetis_common::hal::pit::*;
    pub use super::sim::SimEnabled;
}

pub mod dma {
    pub use chip::dma::*;
    pub use kinetis_common::hal::dma::*;
    pub use super::sim::SimEnabled;
}