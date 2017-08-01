#![allow(non_camel_case_types)]

pub use bobbin_cortexm::hal::*;

pub mod clock;
pub mod pcc;
pub mod wdog;

pub mod port {
    pub use chip::port::*;    
    pub use kinetis_common::hal::port::*;
    pub use super::pcc::PccEnabled;
    use chip::gpio;
    use chip::sig::{SignalTx, SignalRx, SignalFtm};

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

    pub trait ModeFtm<T, S> {
        fn mode_ftm(&self, _: &S) -> &Self;
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

    impl<P, O, S, T> ModeFtm<T, S> for Pin<P, O> where S: SignalFtm<T>, P: AltFn<T> {
        fn mode_ftm(&self, _: &S) -> &Self {
            self.set_mux(self.id.alt_fn());
            self
        }
    }            
}

pub mod gpio {
    pub use chip::gpio::*;
    pub use kinetis_common::hal::gpio::*;
    pub use super::pcc::PccEnabled;
}


pub mod lpuart {
    pub use chip::lpuart::*;
    pub use kinetis_common::hal::lpuart::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod lpit {
    pub use chip::lpit::*;
    pub use kinetis_common::hal::lpit::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod ftm {
    pub use chip::ftm::*;
    pub use kinetis_common::hal::ftm::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod flexcan {
    pub use chip::flexcan::*;
    pub use kinetis_common::hal::flexcan::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod dma {
    pub use chip::dma::*;
    pub use kinetis_common::hal::dma::*;
}

