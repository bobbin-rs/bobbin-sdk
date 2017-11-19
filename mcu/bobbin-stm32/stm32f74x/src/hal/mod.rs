pub use bobbin_cortexm::hal::*;

pub mod rcc;
pub mod clock;
pub mod dcmi;

pub mod dma {
    pub use chip::dma::*;
    pub use stm32_common::hal::dma::*;
    pub use super::rcc::RccEnabled;
}

pub mod adc {
    pub use chip::adc::*;
    pub use stm32_common::hal::adc_f24::*;
    pub use super::rcc::RccEnabled;
}

pub mod gpio {
    pub use bobbin_common::{AltFn, Pin};
    pub use chip::gpio::*;
    pub use stm32_common::hal::gpio::*;
    pub use super::rcc::RccEnabled;
    
    use core::ops::Deref;

    macro_rules! impl_mode {
        ($mode:ident, $meth:ident, $sig:ident) => (
            use chip::sig::$sig;

            pub trait $mode<SIG, PERIPH> {
                fn $meth(&self, _: &PERIPH) -> &Self;
            }

            impl<PERIPH, PIN, SIG> $mode<SIG, PERIPH> for PIN where PERIPH: $sig<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=GpioPin> {
                #[inline]
                fn $meth(&self, _: &PERIPH) -> &Self {
                    self.mode_alt_fn(self.alt_fn());
                    self
                }
            }            
        )
    }

    impl_mode!(ModeRx, mode_rx, SignalRx);
    impl_mode!(ModeTx, mode_tx, SignalTx);
    impl_mode!(ModeTim, mode_tim, SignalTim);
    impl_mode!(ModeAdc, mode_adc, SignalAdc);
    impl_mode!(ModeI2cScl, mode_i2c_scl, SignalI2cScl);
    impl_mode!(ModeI2cSda, mode_i2c_sda, SignalI2cSda);
    impl_mode!(ModeSpiSck, mode_spi_sck, SignalSpiSck);
    impl_mode!(ModeSpiNss, mode_spi_nss, SignalSpiNss);
    impl_mode!(ModeSpiMosi, mode_spi_mosi, SignalSpiMosi);
    impl_mode!(ModeSpiMiso, mode_spi_miso, SignalSpiMiso);
    impl_mode!(ModeDcmiHsync, mode_dcmi_hsync, SignalSigDcmiHsync);
    impl_mode!(ModeDcmiVsync, mode_dcmi_vsync, SignalSigDcmiVsync);
    impl_mode!(ModeDcmiPxclk, mode_dcmi_pxclk, SignalSigDcmiPxclk);
    impl_mode!(ModeDcmiD0, mode_dcmi_d0, SignalSigDcmiD0);
    impl_mode!(ModeDcmiD1, mode_dcmi_d1, SignalSigDcmiD1);
    impl_mode!(ModeDcmiD2, mode_dcmi_d2, SignalSigDcmiD2);
    impl_mode!(ModeDcmiD3, mode_dcmi_d3, SignalSigDcmiD3);
    impl_mode!(ModeDcmiD4, mode_dcmi_d4, SignalSigDcmiD4);
    impl_mode!(ModeDcmiD5, mode_dcmi_d5, SignalSigDcmiD5);
    impl_mode!(ModeDcmiD6, mode_dcmi_d6, SignalSigDcmiD6);
    impl_mode!(ModeDcmiD7, mode_dcmi_d7, SignalSigDcmiD7);
    impl_mode!(ModeDcmiD8, mode_dcmi_d8, SignalSigDcmiD8);
    impl_mode!(ModeDcmiD9, mode_dcmi_d9, SignalSigDcmiD9);
    impl_mode!(ModeDcmiD10, mode_dcmi_d10, SignalSigDcmiD10);
    impl_mode!(ModeDcmiD11, mode_dcmi_d11, SignalSigDcmiD11);
    impl_mode!(ModeDcmiD12, mode_dcmi_d12, SignalSigDcmiD12);
    impl_mode!(ModeDcmiD13, mode_dcmi_d13, SignalSigDcmiD13);

    // impl_mode!(ModeUsbId, mode_usb_id, SignalUsbId);
    // impl_mode!(ModeUsbDm, mode_usb_dm, SignalUsbDm);
    // impl_mode!(ModeUsbDp, mode_usb_dp, SignalUsbDp);

}

pub mod usart {
    pub use chip::usart::*;
    pub use stm32_common::hal::usart::*;
    pub use super::rcc::RccEnabled;
}

pub mod i2c {
    pub use chip::i2c::*;
    pub use stm32_common::hal::i2c_v2::*;
    pub use super::rcc::RccEnabled;
}

pub mod spi {
    pub use chip::spi::*;
    pub use stm32_common::hal::spi_v2::*;
    pub use super::rcc::RccEnabled;
}

pub mod tim {
    pub use chip::tim_gen::*;
    pub use stm32_common::hal::tim_gen::*;
    pub use super::rcc::RccEnabled;    
}

pub mod tim_adv {
    pub use chip::tim_adv::*;
    pub use stm32_common::hal::tim_adv::*;
    pub use super::rcc::RccEnabled;    
}

pub mod crc {
    pub use chip::crc::*;
    pub use stm32_common::hal::crc::*;
    pub use super::rcc::RccEnabled;
}

// pub mod rng {
//     pub use chip::rng::*;
//     pub use stm32_common::hal::rng::*;
//     pub use super::rcc::RccEnabled;
// }

pub mod iwdg {
    pub use chip::iwdg::*;
    pub use stm32_common::hal::iwdg::*;
    pub use super::rcc::RccEnabled;
}

pub mod wwdg {
    pub use chip::wwdg::*;
    pub use stm32_common::hal::wwdg::*;
    pub use super::rcc::RccEnabled;
}

pub mod exti {
    pub use chip::exti::*;
    pub use stm32_common::hal::exti::*;
    pub use super::rcc::RccEnabled;

}

pub mod syscfg {
    pub use chip::syscfg::*;

    pub enum Source {
        GpioA = 0,
        GpioB = 1,
        GpioC = 2,
        GpioD = 3,
        GpioE = 4,
        GpioF = 5,
        GpioG = 6,
        GpioH = 7,
        GpioI = 8,
        GpioJ = 9,
        GpioK = 10,
    }


    pub trait SyscfgExt {
        fn set_exti(&self, index: usize, source: Source) -> &Self;
    }

    impl SyscfgExt for Syscfg {
        fn set_exti(&self, index: usize, source: Source) -> &Self {
            let source: u32 = source as u32;
            match index {
                0 => self.with_exticr1(|r| r.set_exti0(source)),
                1 => self.with_exticr1(|r| r.set_exti1(source)),
                2 => self.with_exticr1(|r| r.set_exti2(source)),
                3 => self.with_exticr1(|r| r.set_exti3(source)),
                4 => self.with_exticr2(|r| r.set_exti4(source)),
                5 => self.with_exticr2(|r| r.set_exti5(source)),
                6 => self.with_exticr2(|r| r.set_exti6(source)),
                7 => self.with_exticr2(|r| r.set_exti7(source)),
                8 => self.with_exticr3(|r| r.set_exti8(source)),
                9 => self.with_exticr3(|r| r.set_exti9(source)),
                10 => self.with_exticr3(|r| r.set_exti10(source)),
                11 => self.with_exticr3(|r| r.set_exti11(source)),
                12 => self.with_exticr4(|r| r.set_exti12(source)),
                13 => self.with_exticr4(|r| r.set_exti13(source)),
                14 => self.with_exticr4(|r| r.set_exti14(source)),
                15 => self.with_exticr4(|r| r.set_exti15(source)),
                _ => unimplemented!(),
            };
            self
        }
    }
}

pub mod usb;