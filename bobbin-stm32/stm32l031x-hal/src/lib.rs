#![no_std]
#![allow(unused_unsafe)]

extern crate bobbin_common;
extern crate bobbin_cortexm;
extern crate stm32_common;
extern crate stm32l031x_chip as chip;

pub use bobbin_cortexm::hal::{nvic, scb};

pub mod pwr;
pub mod rcc;
pub mod clock;

pub mod dma {
    pub use chip::dma::*;
    pub use stm32_common::hal::dma_f3::*;
    pub use rcc::RccEnabled;
}

pub mod gpio {
    pub use chip::gpio::*;
    pub use stm32_common::hal::gpio::*;
    pub use rcc::RccEnabled;
    use chip::sig::{SignalTx, SignalRx, SignalTim};

    pub trait ModeTx<T, S> {
        fn mode_tx(&self, _: &S) -> &Self;
    }

    pub trait ModeRx<T, S> {
        fn mode_rx(&self, _: &S) -> &Self;
    }

    pub trait ModeTim<T, S> {
        fn mode_tim(&self, _: &S) -> &Self;
    }    

    impl<P, O, S, T> ModeTx<T, S> for Pin<P, O> where S: SignalTx<T>, P: AltFn<T> {
        fn mode_tx(&self, _: &S) -> &Self {
            self.mode_altfn(self.id.alt_fn());
            self
        }
    }

    impl<P, O, S, T> ModeRx<T, S> for Pin<P, O> where S: SignalRx<T>, P: AltFn<T> {
        fn mode_rx(&self, _: &S) -> &Self {
            self.mode_altfn(self.id.alt_fn());
            self
        }
    }
    
    impl<P, O, S, T> ModeTim<T, S> for Pin<P, O> where S: SignalTim<T>, P: AltFn<T> {
        fn mode_tim(&self, _: &S) -> &Self {
            self.mode_altfn(self.id.alt_fn());
            self
        }
    }    
}

pub mod usart {
    pub use chip::usart::*;
    pub use stm32_common::hal::usart::*;
    pub use rcc::RccEnabled;
}

pub mod tim {
    pub use chip::tim_gen::*;
    pub use stm32_common::hal::tim_gen::*;
    pub use rcc::RccEnabled;
}

pub mod tim_bas {
    pub use chip::tim_bas::*;
    pub use stm32_common::hal::tim_bas::*;
    pub use rcc::RccEnabled;
}

pub mod crc {
    pub use chip::crc::*;
    pub use stm32_common::hal::crc::*;
    pub use rcc::RccEnabled;
}

// pub mod rng {
//     pub use chip::rng::*;
//     pub use stm32_common::hal::rng::*;
//     pub use rcc::RccEnabled;
// }

pub mod iwdg {
    pub use chip::iwdg::*;
    pub use stm32_common::hal::iwdg::*;
    pub use rcc::RccEnabled;
}

pub mod wwdg {
    pub use chip::wwdg::*;
    pub use stm32_common::hal::wwdg::*;
    pub use rcc::RccEnabled;
}

pub mod exti {
    pub use chip::exti::*;
    pub use stm32_common::hal::exti::*;
    pub use rcc::RccEnabled;

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