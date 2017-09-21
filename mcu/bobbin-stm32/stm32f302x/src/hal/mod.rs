pub use bobbin_cortexm::hal::*;

pub mod rcc;
pub mod clock;
// pub mod iwdg;
// pub mod wwdg;
// pub mod crc;
// pub mod dma;
// pub mod rtc;
// pub mod pwr;

pub mod dma {
    pub use chip::dma::*;
    pub use stm32_common::hal::dma_f3::*;
    pub use super::rcc::RccEnabled;
}

pub mod adc {
    pub use chip::adc::*;
    pub use stm32_common::hal::adc_f3::*;
    pub use super::rcc::RccEnabled;
}

pub mod gpio {
    pub use bobbin_common::{AltFn, Pin};
    pub use chip::gpio::*;
    pub use stm32_common::hal::gpio::*;
    pub use super::rcc::RccEnabled;
    use chip::sig::{SignalTx, SignalRx, SignalTim, SignalAdc};

    use core::ops::Deref;

    pub trait ModeTx<SIG, PERIPH> {
        fn mode_tx(&self, _: &PERIPH) -> &Self;
    }

    pub trait ModeRx<SIG, PERIPH> {
        fn mode_rx(&self, _: &PERIPH) -> &Self;
    }

    pub trait ModeTim<SIG, PERIPH> {
        fn mode_tim(&self, _: &PERIPH) -> &Self;
    }

    pub trait ModeAdc<SIG, PERIPH> {
        fn mode_adc(&self, _: &PERIPH) -> &Self;
    }

    impl<PERIPH, PIN, SIG> ModeTx<SIG, PERIPH> for PIN where PERIPH: SignalTx<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=GpioPin> {
        fn mode_tx(&self, _: &PERIPH) -> &Self {
            self.mode_alt_fn(self.alt_fn());
            self
        }
    }

    impl<PERIPH, PIN, SIG> ModeRx<SIG, PERIPH> for PIN where PERIPH: SignalRx<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=GpioPin> {
        fn mode_rx(&self, _: &PERIPH) -> &Self {
            self.mode_alt_fn(self.alt_fn());
            self
        }
    }
    impl<PERIPH, PIN, SIG> ModeTim<SIG, PERIPH> for PIN where PERIPH: SignalTim<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=GpioPin> {
        fn mode_tim(&self, _: &PERIPH) -> &Self {
            self.mode_alt_fn(self.alt_fn());
            self
        }
    }    
    impl<PERIPH, PIN, SIG> ModeAdc<SIG, PERIPH> for PIN where PERIPH: SignalAdc<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=GpioPin> {
        fn mode_adc(&self, _: &PERIPH) -> &Self {
            self.mode_alt_fn(self.alt_fn()).mode_analog().pull_none();
            self
        }
    }
}

pub mod usart {
    pub use chip::usart::*;
    pub use stm32_common::hal::usart::*;
    pub use super::rcc::RccEnabled;
}

pub mod tim {
    pub use chip::tim_gen::*;
    pub use stm32_common::hal::tim_gen::*;
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