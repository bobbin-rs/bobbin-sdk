pub use bobbin_cortexm::hal::*;

pub mod rcc;
pub mod clock;
pub mod power;
pub mod gpio;

pub mod dma {
    pub use chip::dma::*;
    pub use stm32_common::hal::dma_f3::*;
    pub use super::rcc::RccEnabled;

}

pub mod adc {
    pub use chip::adc::*;
    pub use stm32_common::hal::adc_f124::*;
    pub use super::rcc::RccEnabled;
}

pub mod usart {
    pub use chip::usart::*;
    pub use stm32_common::hal::usart_f24::*;
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
    pub use stm32_common::hal::crc_24::*;
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