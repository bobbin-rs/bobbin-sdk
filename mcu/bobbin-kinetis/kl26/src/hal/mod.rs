#![allow(non_camel_case_types)]

pub use bobbin_cortexm::hal::*;

pub mod sim;
pub mod clock;

pub mod port {
    pub use bobbin_common::{AltFn, Pin};
    pub use chip::port::*;    
    pub use kinetis_common::hal::port::*;
    pub use super::sim::SimEnabled;
    use core::ops::Deref;

    macro_rules! impl_mode {
        ($mode:ident, $meth:ident, $sig:ident) => (
            use chip::sig::$sig;

            pub trait $mode<SIG, PERIPH> {
                fn $meth(&self, _: &PERIPH) -> &Self;
            }

            impl<PERIPH, PIN, SIG> $mode<SIG, PERIPH> for PIN where PERIPH: $sig<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
                #[inline]
                fn $meth(&self, _: &PERIPH) -> &Self {
                    self.set_mux(self.alt_fn());
                    self
                }
            }            
        )
    }


    impl_mode!(ModeTx, mode_tx, SignalTx);
    impl_mode!(ModeRx, mode_rx, SignalRx);
    impl_mode!(ModeAdc, mode_adc, SignalAdc);
    impl_mode!(ModeI2cScl, mode_i2c_scl, SignalI2cScl);
    impl_mode!(ModeI2cSda, mode_i2c_sda, SignalI2cSda);
    impl_mode!(ModeSpiSck, mode_spi_sck, SignalSpiSck);
    impl_mode!(ModeSpiMiso, mode_spi_miso, SignalSpiMiso);
    impl_mode!(ModeSpiMosi, mode_spi_mosi, SignalSpiMosi);
    impl_mode!(ModeSpiPcs0, mode_spi_pcs0, SignalSpiPcs0);
}

pub mod gpio {
    pub use chip::gpio::*;
    pub use kinetis_common::hal::gpio::*;
    pub use super::sim::SimEnabled;
    pub use bobbin_common::Pin;
}


pub mod i2c {
    pub use chip::i2c::*;
    pub use kinetis_common::hal::i2c::*;
    pub use super::sim::SimEnabled;
}

// pub mod spi {
//     pub use chip::spi::*;
//     pub use kinetis_common::hal::spi::*;
//     pub use super::sim::SimEnabled;
// }

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

pub mod lptmr {
    pub use chip::lptmr::*;
    pub use kinetis_common::hal::lptmr::*;
    pub use super::sim::SimEnabled;
}

pub mod dma {
    pub use chip::dma::*;
    pub use kinetis_common::hal::dma::*;
    pub use super::sim::SimEnabled;
    pub use bobbin_common::Channel;
}

pub mod adc {
    pub use chip::adc::*;
    pub use kinetis_common::hal::adc::*;
    pub use super::sim::SimEnabled;
    pub use bobbin_common::Channel;
}