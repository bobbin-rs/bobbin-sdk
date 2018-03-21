pub use ::bobbin_common::mcu::*;

pub mod rcc;
pub mod dac;
pub mod dma;
pub mod crc;
pub mod lcd;
pub mod tsc;
pub mod iwdg;
pub mod wwdg;
pub mod comp;
pub mod firewall;
pub mod i2c;
pub mod flash;
pub mod pwr;
pub mod syscfg;
pub mod rng;
pub mod aes;
pub mod adc;
pub mod gpio;
pub mod sai;
pub mod tim_gen;
pub mod tim_adv;
pub mod tim_bas;
pub mod lptim;
pub mod usart;
pub mod lpuart;
pub mod spi;
pub mod sdio;
pub mod exti;
pub mod vref;
pub mod can;
pub mod rtc;
pub mod swpmi;
pub mod opamp;
pub mod crs;
pub mod usb;
pub mod dfsdm;
pub mod quadspi;
pub mod dbgmcu;
pub mod sig;
pub mod pin;
pub mod irq;

pub struct Mcu {}
pub const MCU: Mcu = Mcu {};

impl Mcu {
    pub fn rcc(&self) -> rcc::Rcc { rcc::RCC }
    pub fn dac1(&self) -> dac::Dac1 { dac::DAC1 }
    pub fn dma1(&self) -> dma::Dma1 { dma::DMA1 }
    pub fn dma2(&self) -> dma::Dma2 { dma::DMA2 }
    pub fn crc(&self) -> crc::Crc { crc::CRC }
    pub fn lcd(&self) -> lcd::Lcd { lcd::LCD }
    pub fn tsc(&self) -> tsc::Tsc { tsc::TSC }
    pub fn iwdg(&self) -> iwdg::Iwdg { iwdg::IWDG }
    pub fn wwdg(&self) -> wwdg::Wwdg { wwdg::WWDG }
    pub fn comp(&self) -> comp::Comp { comp::COMP }
    pub fn firewall(&self) -> firewall::Firewall { firewall::FIREWALL }
    pub fn i2c1(&self) -> i2c::I2c1 { i2c::I2C1 }
    pub fn i2c2(&self) -> i2c::I2c2 { i2c::I2C2 }
    pub fn i2c3(&self) -> i2c::I2c3 { i2c::I2C3 }
    pub fn i2c4(&self) -> i2c::I2c4 { i2c::I2C4 }
    pub fn flash(&self) -> flash::Flash { flash::FLASH }
    pub fn pwr(&self) -> pwr::Pwr { pwr::PWR }
    pub fn syscfg(&self) -> syscfg::Syscfg { syscfg::SYSCFG }
    pub fn rng(&self) -> rng::Rng { rng::RNG }
    pub fn aes(&self) -> aes::Aes { aes::AES }
    pub fn adc(&self) -> adc::Adc { adc::ADC }
    pub fn gpioa(&self) -> gpio::Gpioa { gpio::GPIOA }
    pub fn gpiob(&self) -> gpio::Gpiob { gpio::GPIOB }
    pub fn gpioc(&self) -> gpio::Gpioc { gpio::GPIOC }
    pub fn gpiod(&self) -> gpio::Gpiod { gpio::GPIOD }
    pub fn gpioe(&self) -> gpio::Gpioe { gpio::GPIOE }
    pub fn gpiof(&self) -> gpio::Gpiof { gpio::GPIOF }
    pub fn gpiog(&self) -> gpio::Gpiog { gpio::GPIOG }
    pub fn gpioh(&self) -> gpio::Gpioh { gpio::GPIOH }
    pub fn sai1(&self) -> sai::Sai1 { sai::SAI1 }
    pub fn tim2(&self) -> tim_gen::Tim2 { tim_gen::TIM2 }
    pub fn tim3(&self) -> tim_gen::Tim3 { tim_gen::TIM3 }
    pub fn tim15(&self) -> tim_gen::Tim15 { tim_gen::TIM15 }
    pub fn tim16(&self) -> tim_gen::Tim16 { tim_gen::TIM16 }
    pub fn tim1(&self) -> tim_adv::Tim1 { tim_adv::TIM1 }
    pub fn tim6(&self) -> tim_bas::Tim6 { tim_bas::TIM6 }
    pub fn tim7(&self) -> tim_bas::Tim7 { tim_bas::TIM7 }
    pub fn lptim1(&self) -> lptim::Lptim1 { lptim::LPTIM1 }
    pub fn lptim2(&self) -> lptim::Lptim2 { lptim::LPTIM2 }
    pub fn usart1(&self) -> usart::Usart1 { usart::USART1 }
    pub fn usart2(&self) -> usart::Usart2 { usart::USART2 }
    pub fn usart3(&self) -> usart::Usart3 { usart::USART3 }
    pub fn uart4(&self) -> usart::Uart4 { usart::UART4 }
    pub fn lpuart1(&self) -> lpuart::Lpuart1 { lpuart::LPUART1 }
    pub fn spi1(&self) -> spi::Spi1 { spi::SPI1 }
    pub fn spi2(&self) -> spi::Spi2 { spi::SPI2 }
    pub fn spi3(&self) -> spi::Spi3 { spi::SPI3 }
    pub fn sdmmc(&self) -> sdio::Sdmmc { sdio::SDMMC }
    pub fn exti(&self) -> exti::Exti { exti::EXTI }
    pub fn vrefbuf(&self) -> vref::Vrefbuf { vref::VREFBUF }
    pub fn can1(&self) -> can::Can1 { can::CAN1 }
    pub fn rtc(&self) -> rtc::Rtc { rtc::RTC }
    pub fn swpmi1(&self) -> swpmi::Swpmi1 { swpmi::SWPMI1 }
    pub fn opamp(&self) -> opamp::Opamp { opamp::OPAMP }
    pub fn crs(&self) -> crs::Crs { crs::CRS }
    pub fn usb_sram(&self) -> usb::UsbSram { usb::USB_SRAM }
    pub fn usb_fs(&self) -> usb::UsbFs { usb::USB_FS }
    pub fn dfsdm(&self) -> dfsdm::Dfsdm { dfsdm::DFSDM }
    pub fn quadspi(&self) -> quadspi::Quadspi { quadspi::QUADSPI }
    pub fn dbgmcu(&self) -> dbgmcu::Dbgmcu { dbgmcu::DBGMCU }
}

impl Get<rcc::Rcc> for Mcu {
    fn get(&self) -> rcc::Rcc { rcc::RCC }
}

impl GetPeriph<rcc::RccPeriph> for Mcu {
    fn get_periph(&self) -> rcc::RccPeriph { rcc::RCC_PERIPH }
}

impl GetPeriphInstance<rcc::RccPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<rcc::RccPeriph> {
        match index { 
            0 => Some(rcc::RCC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dac::Dac1> for Mcu {
    fn get(&self) -> dac::Dac1 { dac::DAC1 }
}

impl GetPeriph<dac::DacPeriph> for Mcu {
    fn get_periph(&self) -> dac::DacPeriph { dac::DAC1_PERIPH }
}

impl GetPeriphInstance<dac::DacPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<dac::DacPeriph> {
        match index {
            0 => Some(dac::DAC1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dma::Dma1> for Mcu {
    fn get(&self) -> dma::Dma1 { dma::DMA1 }
}

impl Get<dma::Dma2> for Mcu {
    fn get(&self) -> dma::Dma2 { dma::DMA2 }
}

impl GetPeriphInstance<dma::DmaPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<dma::DmaPeriph> {
        match index {
            0 => Some(dma::DMA1_PERIPH),
            1 => Some(dma::DMA2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<crc::Crc> for Mcu {
    fn get(&self) -> crc::Crc { crc::CRC }
}

impl GetPeriph<crc::CrcPeriph> for Mcu {
    fn get_periph(&self) -> crc::CrcPeriph { crc::CRC_PERIPH }
}

impl GetPeriphInstance<crc::CrcPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<crc::CrcPeriph> {
        match index {
            0 => Some(crc::CRC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<lcd::Lcd> for Mcu {
    fn get(&self) -> lcd::Lcd { lcd::LCD }
}

impl GetPeriph<lcd::LcdPeriph> for Mcu {
    fn get_periph(&self) -> lcd::LcdPeriph { lcd::LCD_PERIPH }
}

impl GetPeriphInstance<lcd::LcdPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<lcd::LcdPeriph> {
        match index {
            0 => Some(lcd::LCD_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<tsc::Tsc> for Mcu {
    fn get(&self) -> tsc::Tsc { tsc::TSC }
}

impl GetPeriph<tsc::TscPeriph> for Mcu {
    fn get_periph(&self) -> tsc::TscPeriph { tsc::TSC_PERIPH }
}

impl GetPeriphInstance<tsc::TscPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<tsc::TscPeriph> {
        match index {
            0 => Some(tsc::TSC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<iwdg::Iwdg> for Mcu {
    fn get(&self) -> iwdg::Iwdg { iwdg::IWDG }
}

impl GetPeriph<iwdg::IwdgPeriph> for Mcu {
    fn get_periph(&self) -> iwdg::IwdgPeriph { iwdg::IWDG_PERIPH }
}

impl GetPeriphInstance<iwdg::IwdgPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<iwdg::IwdgPeriph> {
        match index {
            0 => Some(iwdg::IWDG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<wwdg::Wwdg> for Mcu {
    fn get(&self) -> wwdg::Wwdg { wwdg::WWDG }
}

impl GetPeriph<wwdg::WwdgPeriph> for Mcu {
    fn get_periph(&self) -> wwdg::WwdgPeriph { wwdg::WWDG_PERIPH }
}

impl GetPeriphInstance<wwdg::WwdgPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<wwdg::WwdgPeriph> {
        match index {
            0 => Some(wwdg::WWDG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<comp::Comp> for Mcu {
    fn get(&self) -> comp::Comp { comp::COMP }
}

impl GetPeriph<comp::CompPeriph> for Mcu {
    fn get_periph(&self) -> comp::CompPeriph { comp::COMP_PERIPH }
}

impl GetPeriphInstance<comp::CompPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<comp::CompPeriph> {
        match index {
            0 => Some(comp::COMP_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<firewall::Firewall> for Mcu {
    fn get(&self) -> firewall::Firewall { firewall::FIREWALL }
}

impl GetPeriph<firewall::FirewallPeriph> for Mcu {
    fn get_periph(&self) -> firewall::FirewallPeriph { firewall::FIREWALL_PERIPH }
}

impl GetPeriphInstance<firewall::FirewallPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<firewall::FirewallPeriph> {
        match index {
            0 => Some(firewall::FIREWALL_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<i2c::I2c1> for Mcu {
    fn get(&self) -> i2c::I2c1 { i2c::I2C1 }
}

impl Get<i2c::I2c2> for Mcu {
    fn get(&self) -> i2c::I2c2 { i2c::I2C2 }
}

impl Get<i2c::I2c3> for Mcu {
    fn get(&self) -> i2c::I2c3 { i2c::I2C3 }
}

impl Get<i2c::I2c4> for Mcu {
    fn get(&self) -> i2c::I2c4 { i2c::I2C4 }
}

impl GetPeriphInstance<i2c::I2cPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<i2c::I2cPeriph> {
        match index {
            0 => Some(i2c::I2C1_PERIPH),
            1 => Some(i2c::I2C2_PERIPH),
            2 => Some(i2c::I2C3_PERIPH),
            3 => Some(i2c::I2C4_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 4 }
}

impl Get<flash::Flash> for Mcu {
    fn get(&self) -> flash::Flash { flash::FLASH }
}

impl GetPeriph<flash::FlashPeriph> for Mcu {
    fn get_periph(&self) -> flash::FlashPeriph { flash::FLASH_PERIPH }
}

impl GetPeriphInstance<flash::FlashPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<flash::FlashPeriph> {
        match index {
            0 => Some(flash::FLASH_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<pwr::Pwr> for Mcu {
    fn get(&self) -> pwr::Pwr { pwr::PWR }
}

impl GetPeriph<pwr::PwrPeriph> for Mcu {
    fn get_periph(&self) -> pwr::PwrPeriph { pwr::PWR_PERIPH }
}

impl GetPeriphInstance<pwr::PwrPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<pwr::PwrPeriph> {
        match index {
            0 => Some(pwr::PWR_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<syscfg::Syscfg> for Mcu {
    fn get(&self) -> syscfg::Syscfg { syscfg::SYSCFG }
}

impl GetPeriph<syscfg::SyscfgPeriph> for Mcu {
    fn get_periph(&self) -> syscfg::SyscfgPeriph { syscfg::SYSCFG_PERIPH }
}

impl GetPeriphInstance<syscfg::SyscfgPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<syscfg::SyscfgPeriph> {
        match index {
            0 => Some(syscfg::SYSCFG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rng::Rng> for Mcu {
    fn get(&self) -> rng::Rng { rng::RNG }
}

impl GetPeriph<rng::RngPeriph> for Mcu {
    fn get_periph(&self) -> rng::RngPeriph { rng::RNG_PERIPH }
}

impl GetPeriphInstance<rng::RngPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<rng::RngPeriph> {
        match index {
            0 => Some(rng::RNG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<aes::Aes> for Mcu {
    fn get(&self) -> aes::Aes { aes::AES }
}

impl GetPeriph<aes::AesPeriph> for Mcu {
    fn get_periph(&self) -> aes::AesPeriph { aes::AES_PERIPH }
}

impl GetPeriphInstance<aes::AesPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<aes::AesPeriph> {
        match index {
            0 => Some(aes::AES_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<adc::Adc> for Mcu {
    fn get(&self) -> adc::Adc { adc::ADC }
}

impl GetPeriph<adc::AdcPeriph> for Mcu {
    fn get_periph(&self) -> adc::AdcPeriph { adc::ADC_PERIPH }
}

impl GetPeriphInstance<adc::AdcPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<adc::AdcPeriph> {
        match index {
            0 => Some(adc::ADC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<gpio::Gpioa> for Mcu {
    fn get(&self) -> gpio::Gpioa { gpio::GPIOA }
}

impl Get<gpio::Gpiob> for Mcu {
    fn get(&self) -> gpio::Gpiob { gpio::GPIOB }
}

impl Get<gpio::Gpioc> for Mcu {
    fn get(&self) -> gpio::Gpioc { gpio::GPIOC }
}

impl Get<gpio::Gpiod> for Mcu {
    fn get(&self) -> gpio::Gpiod { gpio::GPIOD }
}

impl Get<gpio::Gpioe> for Mcu {
    fn get(&self) -> gpio::Gpioe { gpio::GPIOE }
}

impl Get<gpio::Gpiof> for Mcu {
    fn get(&self) -> gpio::Gpiof { gpio::GPIOF }
}

impl Get<gpio::Gpiog> for Mcu {
    fn get(&self) -> gpio::Gpiog { gpio::GPIOG }
}

impl Get<gpio::Gpioh> for Mcu {
    fn get(&self) -> gpio::Gpioh { gpio::GPIOH }
}

impl GetPeriphInstance<gpio::GpioPeriph> for Mcu {
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

impl Get<sai::Sai1> for Mcu {
    fn get(&self) -> sai::Sai1 { sai::SAI1 }
}

impl GetPeriph<sai::SaiPeriph> for Mcu {
    fn get_periph(&self) -> sai::SaiPeriph { sai::SAI1_PERIPH }
}

impl GetPeriphInstance<sai::SaiPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<sai::SaiPeriph> {
        match index {
            0 => Some(sai::SAI1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<tim_gen::Tim2> for Mcu {
    fn get(&self) -> tim_gen::Tim2 { tim_gen::TIM2 }
}

impl Get<tim_gen::Tim3> for Mcu {
    fn get(&self) -> tim_gen::Tim3 { tim_gen::TIM3 }
}

impl Get<tim_gen::Tim15> for Mcu {
    fn get(&self) -> tim_gen::Tim15 { tim_gen::TIM15 }
}

impl Get<tim_gen::Tim16> for Mcu {
    fn get(&self) -> tim_gen::Tim16 { tim_gen::TIM16 }
}

impl GetPeriphInstance<tim_gen::TimGenPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<tim_gen::TimGenPeriph> {
        match index {
            0 => Some(tim_gen::TIM2_PERIPH),
            1 => Some(tim_gen::TIM3_PERIPH),
            2 => Some(tim_gen::TIM15_PERIPH),
            3 => Some(tim_gen::TIM16_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 4 }
}

impl Get<tim_adv::Tim1> for Mcu {
    fn get(&self) -> tim_adv::Tim1 { tim_adv::TIM1 }
}

impl GetPeriph<tim_adv::TimAdvPeriph> for Mcu {
    fn get_periph(&self) -> tim_adv::TimAdvPeriph { tim_adv::TIM1_PERIPH }
}

impl GetPeriphInstance<tim_adv::TimAdvPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<tim_adv::TimAdvPeriph> {
        match index {
            0 => Some(tim_adv::TIM1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<tim_bas::Tim6> for Mcu {
    fn get(&self) -> tim_bas::Tim6 { tim_bas::TIM6 }
}

impl Get<tim_bas::Tim7> for Mcu {
    fn get(&self) -> tim_bas::Tim7 { tim_bas::TIM7 }
}

impl GetPeriphInstance<tim_bas::TimBasPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<tim_bas::TimBasPeriph> {
        match index {
            0 => Some(tim_bas::TIM6_PERIPH),
            1 => Some(tim_bas::TIM7_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<lptim::Lptim1> for Mcu {
    fn get(&self) -> lptim::Lptim1 { lptim::LPTIM1 }
}

impl Get<lptim::Lptim2> for Mcu {
    fn get(&self) -> lptim::Lptim2 { lptim::LPTIM2 }
}

impl GetPeriphInstance<lptim::LptimPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<lptim::LptimPeriph> {
        match index {
            0 => Some(lptim::LPTIM1_PERIPH),
            1 => Some(lptim::LPTIM2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<usart::Usart1> for Mcu {
    fn get(&self) -> usart::Usart1 { usart::USART1 }
}

impl Get<usart::Usart2> for Mcu {
    fn get(&self) -> usart::Usart2 { usart::USART2 }
}

impl Get<usart::Usart3> for Mcu {
    fn get(&self) -> usart::Usart3 { usart::USART3 }
}

impl Get<usart::Uart4> for Mcu {
    fn get(&self) -> usart::Uart4 { usart::UART4 }
}

impl GetPeriphInstance<usart::UsartPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<usart::UsartPeriph> {
        match index {
            0 => Some(usart::USART1_PERIPH),
            1 => Some(usart::USART2_PERIPH),
            2 => Some(usart::USART3_PERIPH),
            3 => Some(usart::UART4_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 4 }
}

impl Get<lpuart::Lpuart1> for Mcu {
    fn get(&self) -> lpuart::Lpuart1 { lpuart::LPUART1 }
}

impl GetPeriph<lpuart::LpuartPeriph> for Mcu {
    fn get_periph(&self) -> lpuart::LpuartPeriph { lpuart::LPUART1_PERIPH }
}

impl GetPeriphInstance<lpuart::LpuartPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<lpuart::LpuartPeriph> {
        match index {
            0 => Some(lpuart::LPUART1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<spi::Spi1> for Mcu {
    fn get(&self) -> spi::Spi1 { spi::SPI1 }
}

impl Get<spi::Spi2> for Mcu {
    fn get(&self) -> spi::Spi2 { spi::SPI2 }
}

impl Get<spi::Spi3> for Mcu {
    fn get(&self) -> spi::Spi3 { spi::SPI3 }
}

impl GetPeriphInstance<spi::SpiPeriph> for Mcu {
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

impl Get<sdio::Sdmmc> for Mcu {
    fn get(&self) -> sdio::Sdmmc { sdio::SDMMC }
}

impl GetPeriph<sdio::SdioPeriph> for Mcu {
    fn get_periph(&self) -> sdio::SdioPeriph { sdio::SDMMC_PERIPH }
}

impl GetPeriphInstance<sdio::SdioPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<sdio::SdioPeriph> {
        match index {
            0 => Some(sdio::SDMMC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<exti::Exti> for Mcu {
    fn get(&self) -> exti::Exti { exti::EXTI }
}

impl GetPeriph<exti::ExtiPeriph> for Mcu {
    fn get_periph(&self) -> exti::ExtiPeriph { exti::EXTI_PERIPH }
}

impl GetPeriphInstance<exti::ExtiPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<exti::ExtiPeriph> {
        match index {
            0 => Some(exti::EXTI_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<vref::Vrefbuf> for Mcu {
    fn get(&self) -> vref::Vrefbuf { vref::VREFBUF }
}

impl GetPeriph<vref::VrefPeriph> for Mcu {
    fn get_periph(&self) -> vref::VrefPeriph { vref::VREFBUF_PERIPH }
}

impl GetPeriphInstance<vref::VrefPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<vref::VrefPeriph> {
        match index {
            0 => Some(vref::VREFBUF_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<can::Can1> for Mcu {
    fn get(&self) -> can::Can1 { can::CAN1 }
}

impl GetPeriph<can::CanPeriph> for Mcu {
    fn get_periph(&self) -> can::CanPeriph { can::CAN1_PERIPH }
}

impl GetPeriphInstance<can::CanPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<can::CanPeriph> {
        match index {
            0 => Some(can::CAN1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rtc::Rtc> for Mcu {
    fn get(&self) -> rtc::Rtc { rtc::RTC }
}

impl GetPeriph<rtc::RtcPeriph> for Mcu {
    fn get_periph(&self) -> rtc::RtcPeriph { rtc::RTC_PERIPH }
}

impl GetPeriphInstance<rtc::RtcPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<rtc::RtcPeriph> {
        match index {
            0 => Some(rtc::RTC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<swpmi::Swpmi1> for Mcu {
    fn get(&self) -> swpmi::Swpmi1 { swpmi::SWPMI1 }
}

impl GetPeriph<swpmi::SwpmiPeriph> for Mcu {
    fn get_periph(&self) -> swpmi::SwpmiPeriph { swpmi::SWPMI1_PERIPH }
}

impl GetPeriphInstance<swpmi::SwpmiPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<swpmi::SwpmiPeriph> {
        match index {
            0 => Some(swpmi::SWPMI1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<opamp::Opamp> for Mcu {
    fn get(&self) -> opamp::Opamp { opamp::OPAMP }
}

impl GetPeriph<opamp::OpampPeriph> for Mcu {
    fn get_periph(&self) -> opamp::OpampPeriph { opamp::OPAMP_PERIPH }
}

impl GetPeriphInstance<opamp::OpampPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<opamp::OpampPeriph> {
        match index {
            0 => Some(opamp::OPAMP_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<crs::Crs> for Mcu {
    fn get(&self) -> crs::Crs { crs::CRS }
}

impl GetPeriph<crs::CrsPeriph> for Mcu {
    fn get_periph(&self) -> crs::CrsPeriph { crs::CRS_PERIPH }
}

impl GetPeriphInstance<crs::CrsPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<crs::CrsPeriph> {
        match index {
            0 => Some(crs::CRS_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<usb::UsbSram> for Mcu {
    fn get(&self) -> usb::UsbSram { usb::USB_SRAM }
}

impl Get<usb::UsbFs> for Mcu {
    fn get(&self) -> usb::UsbFs { usb::USB_FS }
}

impl GetPeriphInstance<usb::UsbPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<usb::UsbPeriph> {
        match index {
            0 => Some(usb::USB_SRAM_PERIPH),
            1 => Some(usb::USB_FS_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<dfsdm::Dfsdm> for Mcu {
    fn get(&self) -> dfsdm::Dfsdm { dfsdm::DFSDM }
}

impl GetPeriph<dfsdm::DfsdmPeriph> for Mcu {
    fn get_periph(&self) -> dfsdm::DfsdmPeriph { dfsdm::DFSDM_PERIPH }
}

impl GetPeriphInstance<dfsdm::DfsdmPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<dfsdm::DfsdmPeriph> {
        match index {
            0 => Some(dfsdm::DFSDM_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<quadspi::Quadspi> for Mcu {
    fn get(&self) -> quadspi::Quadspi { quadspi::QUADSPI }
}

impl GetPeriph<quadspi::QuadspiPeriph> for Mcu {
    fn get_periph(&self) -> quadspi::QuadspiPeriph { quadspi::QUADSPI_PERIPH }
}

impl GetPeriphInstance<quadspi::QuadspiPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<quadspi::QuadspiPeriph> {
        match index {
            0 => Some(quadspi::QUADSPI_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dbgmcu::Dbgmcu> for Mcu {
    fn get(&self) -> dbgmcu::Dbgmcu { dbgmcu::DBGMCU }
}

impl GetPeriph<dbgmcu::DbgmcuPeriph> for Mcu {
    fn get_periph(&self) -> dbgmcu::DbgmcuPeriph { dbgmcu::DBGMCU_PERIPH }
}

impl GetPeriphInstance<dbgmcu::DbgmcuPeriph> for Mcu {
    fn get_periph_instance(&self, index: usize) -> Option<dbgmcu::DbgmcuPeriph> {
        match index {
            0 => Some(dbgmcu::DBGMCU_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

