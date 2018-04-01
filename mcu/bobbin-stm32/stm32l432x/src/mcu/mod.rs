pub use ::bobbin_common::mcu::*;

pub mod flash;
pub mod pwr;
pub mod rcc;
pub mod syscfg;
pub mod c_adc;
pub mod dac;
pub mod rng;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod rtc;
pub mod lptim;
pub mod exti;
pub mod dma;
pub mod i2c;
pub mod tim_adv;
pub mod tim_bas;
pub mod tim_gen;
pub mod gpio;
pub mod usart;
pub mod lpuart;
pub mod spi;
pub mod adc;
pub mod sig;
pub mod pin;
pub mod irq;

#[derive(Debug, Default)]
pub struct Stm32l432x {}

impl Mcu for Stm32l432x {
    fn id(&self) -> &'static str { "STM32L432x" }
}

impl Stm32l432x {
    pub fn flash(&self) -> flash::Flash { flash::FLASH }
    pub fn pwr(&self) -> pwr::Pwr { pwr::PWR }
    pub fn rcc(&self) -> rcc::Rcc { rcc::RCC }
    pub fn syscfg(&self) -> syscfg::Syscfg { syscfg::SYSCFG }
    pub fn c_adc(&self) -> c_adc::CAdc { c_adc::C_ADC }
    pub fn dac1(&self) -> dac::Dac1 { dac::DAC1 }
    pub fn rng(&self) -> rng::Rng { rng::RNG }
    pub fn iwdg(&self) -> iwdg::Iwdg { iwdg::IWDG }
    pub fn wwdg(&self) -> wwdg::Wwdg { wwdg::WWDG }
    pub fn crc(&self) -> crc::Crc { crc::CRC }
    pub fn rtc(&self) -> rtc::Rtc { rtc::RTC }
    pub fn lptim1(&self) -> lptim::Lptim1 { lptim::LPTIM1 }
    pub fn lptim2(&self) -> lptim::Lptim2 { lptim::LPTIM2 }
    pub fn exti(&self) -> exti::Exti { exti::EXTI }
    pub fn dma1(&self) -> dma::Dma1 { dma::DMA1 }
    pub fn dma2(&self) -> dma::Dma2 { dma::DMA2 }
    pub fn i2c1(&self) -> i2c::I2c1 { i2c::I2C1 }
    pub fn i2c2(&self) -> i2c::I2c2 { i2c::I2C2 }
    pub fn i2c3(&self) -> i2c::I2c3 { i2c::I2C3 }
    pub fn tim1(&self) -> tim_adv::Tim1 { tim_adv::TIM1 }
    pub fn tim8(&self) -> tim_adv::Tim8 { tim_adv::TIM8 }
    pub fn tim6(&self) -> tim_bas::Tim6 { tim_bas::TIM6 }
    pub fn tim7(&self) -> tim_bas::Tim7 { tim_bas::TIM7 }
    pub fn tim2(&self) -> tim_gen::Tim2 { tim_gen::TIM2 }
    pub fn tim3(&self) -> tim_gen::Tim3 { tim_gen::TIM3 }
    pub fn tim4(&self) -> tim_gen::Tim4 { tim_gen::TIM4 }
    pub fn tim5(&self) -> tim_gen::Tim5 { tim_gen::TIM5 }
    pub fn tim15(&self) -> tim_gen::Tim15 { tim_gen::TIM15 }
    pub fn tim16(&self) -> tim_gen::Tim16 { tim_gen::TIM16 }
    pub fn gpioa(&self) -> gpio::Gpioa { gpio::GPIOA }
    pub fn gpiob(&self) -> gpio::Gpiob { gpio::GPIOB }
    pub fn gpioc(&self) -> gpio::Gpioc { gpio::GPIOC }
    pub fn gpiod(&self) -> gpio::Gpiod { gpio::GPIOD }
    pub fn gpioe(&self) -> gpio::Gpioe { gpio::GPIOE }
    pub fn gpiof(&self) -> gpio::Gpiof { gpio::GPIOF }
    pub fn gpiog(&self) -> gpio::Gpiog { gpio::GPIOG }
    pub fn gpioh(&self) -> gpio::Gpioh { gpio::GPIOH }
    pub fn usart1(&self) -> usart::Usart1 { usart::USART1 }
    pub fn usart2(&self) -> usart::Usart2 { usart::USART2 }
    pub fn usart3(&self) -> usart::Usart3 { usart::USART3 }
    pub fn uart4(&self) -> usart::Uart4 { usart::UART4 }
    pub fn uart5(&self) -> usart::Uart5 { usart::UART5 }
    pub fn lpuart1(&self) -> lpuart::Lpuart1 { lpuart::LPUART1 }
    pub fn spi1(&self) -> spi::Spi1 { spi::SPI1 }
    pub fn spi2(&self) -> spi::Spi2 { spi::SPI2 }
    pub fn spi3(&self) -> spi::Spi3 { spi::SPI3 }
    pub fn adc1(&self) -> adc::Adc1 { adc::ADC1 }
    pub fn adc2(&self) -> adc::Adc2 { adc::ADC2 }
    pub fn adc3(&self) -> adc::Adc3 { adc::ADC3 }
}

impl Get<flash::Flash> for Stm32l432x {
    fn get(&self) -> flash::Flash { flash::FLASH }
}

impl GetPeriph<flash::FlashPeriph> for Stm32l432x {
    fn get_periph(&self) -> flash::FlashPeriph { flash::FLASH_PERIPH }
}

impl GetPeriphInstance<flash::FlashPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<flash::FlashPeriph> {
        match index { 
            0 => Some(flash::FLASH_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<pwr::Pwr> for Stm32l432x {
    fn get(&self) -> pwr::Pwr { pwr::PWR }
}

impl GetPeriph<pwr::PwrPeriph> for Stm32l432x {
    fn get_periph(&self) -> pwr::PwrPeriph { pwr::PWR_PERIPH }
}

impl GetPeriphInstance<pwr::PwrPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<pwr::PwrPeriph> {
        match index { 
            0 => Some(pwr::PWR_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rcc::Rcc> for Stm32l432x {
    fn get(&self) -> rcc::Rcc { rcc::RCC }
}

impl GetPeriph<rcc::RccPeriph> for Stm32l432x {
    fn get_periph(&self) -> rcc::RccPeriph { rcc::RCC_PERIPH }
}

impl GetPeriphInstance<rcc::RccPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<rcc::RccPeriph> {
        match index { 
            0 => Some(rcc::RCC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<syscfg::Syscfg> for Stm32l432x {
    fn get(&self) -> syscfg::Syscfg { syscfg::SYSCFG }
}

impl GetPeriph<syscfg::SyscfgPeriph> for Stm32l432x {
    fn get_periph(&self) -> syscfg::SyscfgPeriph { syscfg::SYSCFG_PERIPH }
}

impl GetPeriphInstance<syscfg::SyscfgPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<syscfg::SyscfgPeriph> {
        match index { 
            0 => Some(syscfg::SYSCFG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<c_adc::CAdc> for Stm32l432x {
    fn get(&self) -> c_adc::CAdc { c_adc::C_ADC }
}

impl GetPeriph<c_adc::CAdcPeriph> for Stm32l432x {
    fn get_periph(&self) -> c_adc::CAdcPeriph { c_adc::C_ADC_PERIPH }
}

impl GetPeriphInstance<c_adc::CAdcPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<c_adc::CAdcPeriph> {
        match index { 
            0 => Some(c_adc::C_ADC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dac::Dac1> for Stm32l432x {
    fn get(&self) -> dac::Dac1 { dac::DAC1 }
}

impl GetPeriph<dac::DacPeriph> for Stm32l432x {
    fn get_periph(&self) -> dac::DacPeriph { dac::DAC1_PERIPH }
}

impl GetPeriphInstance<dac::DacPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<dac::DacPeriph> {
        match index {
            0 => Some(dac::DAC1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rng::Rng> for Stm32l432x {
    fn get(&self) -> rng::Rng { rng::RNG }
}

impl GetPeriph<rng::RngPeriph> for Stm32l432x {
    fn get_periph(&self) -> rng::RngPeriph { rng::RNG_PERIPH }
}

impl GetPeriphInstance<rng::RngPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<rng::RngPeriph> {
        match index {
            0 => Some(rng::RNG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<iwdg::Iwdg> for Stm32l432x {
    fn get(&self) -> iwdg::Iwdg { iwdg::IWDG }
}

impl GetPeriph<iwdg::IwdgPeriph> for Stm32l432x {
    fn get_periph(&self) -> iwdg::IwdgPeriph { iwdg::IWDG_PERIPH }
}

impl GetPeriphInstance<iwdg::IwdgPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<iwdg::IwdgPeriph> {
        match index {
            0 => Some(iwdg::IWDG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<wwdg::Wwdg> for Stm32l432x {
    fn get(&self) -> wwdg::Wwdg { wwdg::WWDG }
}

impl GetPeriph<wwdg::WwdgPeriph> for Stm32l432x {
    fn get_periph(&self) -> wwdg::WwdgPeriph { wwdg::WWDG_PERIPH }
}

impl GetPeriphInstance<wwdg::WwdgPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<wwdg::WwdgPeriph> {
        match index {
            0 => Some(wwdg::WWDG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<crc::Crc> for Stm32l432x {
    fn get(&self) -> crc::Crc { crc::CRC }
}

impl GetPeriph<crc::CrcPeriph> for Stm32l432x {
    fn get_periph(&self) -> crc::CrcPeriph { crc::CRC_PERIPH }
}

impl GetPeriphInstance<crc::CrcPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<crc::CrcPeriph> {
        match index {
            0 => Some(crc::CRC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rtc::Rtc> for Stm32l432x {
    fn get(&self) -> rtc::Rtc { rtc::RTC }
}

impl GetPeriph<rtc::RtcPeriph> for Stm32l432x {
    fn get_periph(&self) -> rtc::RtcPeriph { rtc::RTC_PERIPH }
}

impl GetPeriphInstance<rtc::RtcPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<rtc::RtcPeriph> {
        match index {
            0 => Some(rtc::RTC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<lptim::Lptim1> for Stm32l432x {
    fn get(&self) -> lptim::Lptim1 { lptim::LPTIM1 }
}

impl Get<lptim::Lptim2> for Stm32l432x {
    fn get(&self) -> lptim::Lptim2 { lptim::LPTIM2 }
}

impl GetPeriphInstance<lptim::LptimPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<lptim::LptimPeriph> {
        match index {
            0 => Some(lptim::LPTIM1_PERIPH),
            1 => Some(lptim::LPTIM2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<exti::Exti> for Stm32l432x {
    fn get(&self) -> exti::Exti { exti::EXTI }
}

impl GetPeriph<exti::ExtiPeriph> for Stm32l432x {
    fn get_periph(&self) -> exti::ExtiPeriph { exti::EXTI_PERIPH }
}

impl GetPeriphInstance<exti::ExtiPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<exti::ExtiPeriph> {
        match index {
            0 => Some(exti::EXTI_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dma::Dma1> for Stm32l432x {
    fn get(&self) -> dma::Dma1 { dma::DMA1 }
}

impl Get<dma::Dma2> for Stm32l432x {
    fn get(&self) -> dma::Dma2 { dma::DMA2 }
}

impl GetPeriphInstance<dma::DmaPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<dma::DmaPeriph> {
        match index {
            0 => Some(dma::DMA1_PERIPH),
            1 => Some(dma::DMA2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<i2c::I2c1> for Stm32l432x {
    fn get(&self) -> i2c::I2c1 { i2c::I2C1 }
}

impl Get<i2c::I2c2> for Stm32l432x {
    fn get(&self) -> i2c::I2c2 { i2c::I2C2 }
}

impl Get<i2c::I2c3> for Stm32l432x {
    fn get(&self) -> i2c::I2c3 { i2c::I2C3 }
}

impl GetPeriphInstance<i2c::I2cPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<i2c::I2cPeriph> {
        match index {
            0 => Some(i2c::I2C1_PERIPH),
            1 => Some(i2c::I2C2_PERIPH),
            2 => Some(i2c::I2C3_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<tim_adv::Tim1> for Stm32l432x {
    fn get(&self) -> tim_adv::Tim1 { tim_adv::TIM1 }
}

impl Get<tim_adv::Tim8> for Stm32l432x {
    fn get(&self) -> tim_adv::Tim8 { tim_adv::TIM8 }
}

impl GetPeriphInstance<tim_adv::TimAdvPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<tim_adv::TimAdvPeriph> {
        match index {
            0 => Some(tim_adv::TIM1_PERIPH),
            1 => Some(tim_adv::TIM8_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<tim_bas::Tim6> for Stm32l432x {
    fn get(&self) -> tim_bas::Tim6 { tim_bas::TIM6 }
}

impl Get<tim_bas::Tim7> for Stm32l432x {
    fn get(&self) -> tim_bas::Tim7 { tim_bas::TIM7 }
}

impl GetPeriphInstance<tim_bas::TimBasPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<tim_bas::TimBasPeriph> {
        match index {
            0 => Some(tim_bas::TIM6_PERIPH),
            1 => Some(tim_bas::TIM7_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<tim_gen::Tim2> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim2 { tim_gen::TIM2 }
}

impl Get<tim_gen::Tim3> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim3 { tim_gen::TIM3 }
}

impl Get<tim_gen::Tim4> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim4 { tim_gen::TIM4 }
}

impl Get<tim_gen::Tim5> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim5 { tim_gen::TIM5 }
}

impl Get<tim_gen::Tim15> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim15 { tim_gen::TIM15 }
}

impl Get<tim_gen::Tim16> for Stm32l432x {
    fn get(&self) -> tim_gen::Tim16 { tim_gen::TIM16 }
}

impl GetPeriphInstance<tim_gen::TimGenPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<tim_gen::TimGenPeriph> {
        match index {
            0 => Some(tim_gen::TIM2_PERIPH),
            1 => Some(tim_gen::TIM3_PERIPH),
            2 => Some(tim_gen::TIM4_PERIPH),
            3 => Some(tim_gen::TIM5_PERIPH),
            4 => Some(tim_gen::TIM15_PERIPH),
            5 => Some(tim_gen::TIM16_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 6 }
}

impl Get<gpio::Gpioa> for Stm32l432x {
    fn get(&self) -> gpio::Gpioa { gpio::GPIOA }
}

impl Get<gpio::Gpiob> for Stm32l432x {
    fn get(&self) -> gpio::Gpiob { gpio::GPIOB }
}

impl Get<gpio::Gpioc> for Stm32l432x {
    fn get(&self) -> gpio::Gpioc { gpio::GPIOC }
}

impl Get<gpio::Gpiod> for Stm32l432x {
    fn get(&self) -> gpio::Gpiod { gpio::GPIOD }
}

impl Get<gpio::Gpioe> for Stm32l432x {
    fn get(&self) -> gpio::Gpioe { gpio::GPIOE }
}

impl Get<gpio::Gpiof> for Stm32l432x {
    fn get(&self) -> gpio::Gpiof { gpio::GPIOF }
}

impl Get<gpio::Gpiog> for Stm32l432x {
    fn get(&self) -> gpio::Gpiog { gpio::GPIOG }
}

impl Get<gpio::Gpioh> for Stm32l432x {
    fn get(&self) -> gpio::Gpioh { gpio::GPIOH }
}

impl GetPeriphInstance<gpio::GpioPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<gpio::GpioPeriph> {
        match index {
            0 => Some(gpio::GPIOA_PERIPH),
            1 => Some(gpio::GPIOB_PERIPH),
            2 => Some(gpio::GPIOC_PERIPH),
            3 => Some(gpio::GPIOD_PERIPH),
            4 => Some(gpio::GPIOE_PERIPH),
            5 => Some(gpio::GPIOF_PERIPH),
            6 => Some(gpio::GPIOG_PERIPH),
            7 => Some(gpio::GPIOH_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 8 }
}

impl Get<usart::Usart1> for Stm32l432x {
    fn get(&self) -> usart::Usart1 { usart::USART1 }
}

impl Get<usart::Usart2> for Stm32l432x {
    fn get(&self) -> usart::Usart2 { usart::USART2 }
}

impl Get<usart::Usart3> for Stm32l432x {
    fn get(&self) -> usart::Usart3 { usart::USART3 }
}

impl Get<usart::Uart4> for Stm32l432x {
    fn get(&self) -> usart::Uart4 { usart::UART4 }
}

impl Get<usart::Uart5> for Stm32l432x {
    fn get(&self) -> usart::Uart5 { usart::UART5 }
}

impl GetPeriphInstance<usart::UsartPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<usart::UsartPeriph> {
        match index {
            0 => Some(usart::USART1_PERIPH),
            1 => Some(usart::USART2_PERIPH),
            2 => Some(usart::USART3_PERIPH),
            3 => Some(usart::UART4_PERIPH),
            4 => Some(usart::UART5_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 5 }
}

impl Get<lpuart::Lpuart1> for Stm32l432x {
    fn get(&self) -> lpuart::Lpuart1 { lpuart::LPUART1 }
}

impl GetPeriph<lpuart::LpuartPeriph> for Stm32l432x {
    fn get_periph(&self) -> lpuart::LpuartPeriph { lpuart::LPUART1_PERIPH }
}

impl GetPeriphInstance<lpuart::LpuartPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<lpuart::LpuartPeriph> {
        match index {
            0 => Some(lpuart::LPUART1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<spi::Spi1> for Stm32l432x {
    fn get(&self) -> spi::Spi1 { spi::SPI1 }
}

impl Get<spi::Spi2> for Stm32l432x {
    fn get(&self) -> spi::Spi2 { spi::SPI2 }
}

impl Get<spi::Spi3> for Stm32l432x {
    fn get(&self) -> spi::Spi3 { spi::SPI3 }
}

impl GetPeriphInstance<spi::SpiPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<spi::SpiPeriph> {
        match index {
            0 => Some(spi::SPI1_PERIPH),
            1 => Some(spi::SPI2_PERIPH),
            2 => Some(spi::SPI3_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<adc::Adc1> for Stm32l432x {
    fn get(&self) -> adc::Adc1 { adc::ADC1 }
}

impl Get<adc::Adc2> for Stm32l432x {
    fn get(&self) -> adc::Adc2 { adc::ADC2 }
}

impl Get<adc::Adc3> for Stm32l432x {
    fn get(&self) -> adc::Adc3 { adc::ADC3 }
}

impl GetPeriphInstance<adc::AdcPeriph> for Stm32l432x {
    fn get_periph_instance(&self, index: usize) -> Option<adc::AdcPeriph> {
        match index {
            0 => Some(adc::ADC1_PERIPH),
            1 => Some(adc::ADC2_PERIPH),
            2 => Some(adc::ADC3_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

