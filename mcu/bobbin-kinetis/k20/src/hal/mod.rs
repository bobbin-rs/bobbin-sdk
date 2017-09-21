#![allow(non_camel_case_types)]

pub use bobbin_cortexm::hal::*;

pub mod sim;
pub mod clock;
// pub mod port;
// pub mod uart;
// pub mod pit;
// pub mod i2c;
// pub mod spi;
// pub mod enet;

pub mod port {
    pub use bobbin_common::{AltFn, Pin};
    pub use chip::port::*;    
    pub use kinetis_common::hal::port::*;
    pub use super::sim::SimEnabled;
    use chip::sig::{SignalTx, SignalRx, SignalFtm};

    use core::ops::Deref;

    pub trait ModeTx<SIG, PERIPH> {
        fn mode_tx(&self, _: &PERIPH) -> &Self;
    }

    pub trait ModeRx<SIG, PERIPH> {
        fn mode_rx(&self, _: &PERIPH) -> &Self;
    }

    pub trait ModeFtm<SIG, PERIPH> {
        fn mode_ftm(&self, _: &PERIPH) -> &Self;
    }

    impl<PERIPH, PIN, SIG> ModeTx<SIG, PERIPH> for PIN where PERIPH: SignalTx<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
        fn mode_tx(&self, _: &PERIPH) -> &Self {
            self.set_mux(self.alt_fn());
            self
        }
    }

    impl<PERIPH, PIN, SIG> ModeRx<SIG, PERIPH> for PIN where PERIPH: SignalRx<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
        fn mode_rx(&self, _: &PERIPH) -> &Self {
            self.set_mux(self.alt_fn());
            self
        }
    }   

    impl<PERIPH, PIN, SIG> ModeFtm<SIG, PERIPH> for PIN where PERIPH: SignalFtm<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
        fn mode_ftm(&self, _: &PERIPH) -> &Self {
            self.set_mux(self.alt_fn());
            self
        }
    }        
}

pub mod gpio {
    pub use chip::gpio::*;
    pub use kinetis_common::hal::gpio::*;
    pub use super::sim::SimEnabled;
    pub use bobbin_common::Pin;
}


pub mod uart {
    pub use chip::uart::*;
    pub use kinetis_common::hal::uart::*;
    pub use super::sim::SimEnabled;
}

pub mod ftm {
    pub use chip::ftm::*;
    pub use kinetis_common::hal::ftm::*;
    pub use super::sim::SimEnabled;
}

pub mod pit {
    pub use chip::pit::*;
    pub use kinetis_common::hal::pit::*;
    pub use super::sim::SimEnabled;
}

pub mod edma {
    pub use chip::edma::*;
    pub use kinetis_common::hal::edma::*;
    pub use super::sim::SimEnabled;
}