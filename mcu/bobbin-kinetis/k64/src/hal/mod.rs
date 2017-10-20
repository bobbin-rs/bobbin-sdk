#![allow(non_camel_case_types)]

pub use bobbin_cortexm::hal::*;

pub mod sim;
pub mod fpu;
pub mod clock;

pub mod port {
    pub use bobbin_common::{AltFn, Pin};
    pub use chip::port::*;    
    pub use kinetis_common::hal::port::*;
    pub use super::sim::SimEnabled;
    // use chip::sig::{SignalTx, SignalRx, SignalFtm, SignalAdc};

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
    impl_mode!(ModeFtm, mode_ftm, SignalFtm);
    impl_mode!(ModeAdc, mode_adc, SignalAdc);
    impl_mode!(ModeI2cScl, mode_i2c_scl, SignalI2cScl);
    impl_mode!(ModeI2cSda, mode_i2c_sda, SignalI2cSda);
    impl_mode!(ModeSpiSck, mode_spi_sck, SignalSpiSck);
    impl_mode!(ModeSpiSout, mode_spi_sout, SignalSpiSout);
    impl_mode!(ModeSpiSin, mode_spi_sin, SignalSpiSin);
    impl_mode!(ModeSpiPcs0, mode_spi_pcs0, SignalSpiPcs0);
    impl_mode!(ModeSpiPcs1, mode_spi_pcs1, SignalSpiPcs1);
    impl_mode!(ModeSpiPcs2, mode_spi_pcs2, SignalSpiPcs2);
    impl_mode!(ModeSpiPcs3, mode_spi_pcs3, SignalSpiPcs3);
    impl_mode!(ModeSpiPcs4, mode_spi_pcs4, SignalSpiPcs4);
    impl_mode!(ModeSpiPcs5, mode_spi_pcs5, SignalSpiPcs5);


    // pub trait ModeTx<SIG, PERIPH> {
    //     fn mode_tx(&self, _: &PERIPH) -> &Self;
    // }

    // pub trait ModeRx<SIG, PERIPH> {
    //     fn mode_rx(&self, _: &PERIPH) -> &Self;
    // }

    // pub trait ModeFtm<SIG, PERIPH> {
    //     fn mode_ftm(&self, _: &PERIPH) -> &Self;
    // }

    // pub trait ModeAdc<SIG, PERIPH> {
    //     fn mode_adc(&self, _: &PERIPH) -> &Self;
    // }

    // impl<PERIPH, PIN, SIG> ModeTx<SIG, PERIPH> for PIN where PERIPH: SignalTx<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    //     fn mode_tx(&self, _: &PERIPH) -> &Self {
    //         self.set_mux(self.alt_fn());
    //         self
    //     }
    // }

    // impl<PERIPH, PIN, SIG> ModeRx<SIG, PERIPH> for PIN where PERIPH: SignalRx<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    //     fn mode_rx(&self, _: &PERIPH) -> &Self {
    //         self.set_mux(self.alt_fn());
    //         self
    //     }
    // }   

    // impl<PERIPH, PIN, SIG> ModeFtm<SIG, PERIPH> for PIN where PERIPH: SignalFtm<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    //     fn mode_ftm(&self, _: &PERIPH) -> &Self {
    //         self.set_mux(self.alt_fn());
    //         self
    //     }
    // }        

    // impl<PERIPH, PIN, SIG> ModeAdc<SIG, PERIPH> for PIN where PERIPH: SignalAdc<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    //     fn mode_adc(&self, _: &PERIPH) -> &Self {
    //         self.set_mux(self.alt_fn());
    //         self
    //     }
    // }
}

pub mod gpio {
    pub use chip::gpio::*;
    pub use kinetis_common::hal::gpio::*;
    pub use super::sim::SimEnabled;
}

pub mod i2c {
    pub use chip::i2c::*;
    pub use kinetis_common::hal::i2c::*;
    pub use super::sim::SimEnabled;
}

pub mod spi {
    pub use chip::spi::*;
    pub use kinetis_common::hal::spi::*;
    pub use super::sim::SimEnabled;
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

pub mod lptmr {
    pub use chip::lptmr::*;
    pub use kinetis_common::hal::lptmr::*;
    pub use super::sim::SimEnabled;
}

pub mod edma {
    pub use chip::edma::*;
    pub use kinetis_common::hal::edma::*;
    pub use super::sim::SimEnabled;
}

pub mod dmamux {
    pub use chip::dmamux::*;
    pub use kinetis_common::hal::dmamux::*;
    pub use super::sim::SimEnabled;
}

pub mod adc {
    pub use chip::adc::*;
    pub use kinetis_common::hal::adc::*;
    pub use super::sim::SimEnabled;
}

pub mod wdog {
    pub use chip::wdog::*;
    pub use kinetis_common::hal::wdog::*;
}

pub mod crc {
    pub use chip::crc::*;
    pub use kinetis_common::hal::crc::*;
    pub use super::sim::SimEnabled;
}

pub mod flexcan {
    pub use chip::flexcan::*;
    pub use kinetis_common::hal::flexcan::*;
    pub use super::sim::SimEnabled;
}

pub mod usb {
    pub use chip::usb::*;
    pub use kinetis_common::hal::usb::*;
    pub use super::sim::SimEnabled;

    pub trait InitUsbIrc {
        fn init_usbsrc_irc(&self);
    }

    impl InitUsbIrc for UsbPeriph {
        fn init_usbsrc_irc(&self) {    
            // DISABLE MPU    
            // NOTE: MPU must be disabled to allow DMA from USBFS to memory.
            ::chip::mpu::MPU.with_cesr(|r| r.set_vld(0));

            
            // Reset USB
            
            self.with_usbtrc0(|r| r.set_usbreset(1));
            while self.usbtrc0().test_usbreset() {}

            // Set clock source

            ::chip::sim::SIM.with_sopt2(|r| r
                .set_pllfllsel(0b11)
                .set_usbsrc(1)
            );        

            // Enable IRC
            self.with_clk_recover_irc_en(|r| r.set_irc_en(1));

            // Enable Clock Recovery
            self.with_clk_recover_ctrl(|r| r.set_clock_recover_en(1));

        }
    }
}