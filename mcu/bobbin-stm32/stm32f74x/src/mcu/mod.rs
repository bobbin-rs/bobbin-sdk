pub use ::bobbin_common::mcu::*;

pub mod rcc;
pub mod flash;
pub mod pwr;
pub mod syscfg;
pub mod dbg;
pub mod ethernet_mac;
pub mod ethernet_mmc;
pub mod ethernet_ptp;
pub mod ethernet_dma;
pub mod dac;
pub mod sdmmc;
pub mod quadspi;
pub mod cec;
pub mod spdif_rx;
pub mod ltdc;
pub mod dma2d;
pub mod hash;
pub mod cryp;
pub mod c_adc;
pub mod dcmi;
pub mod usb_fs_global;
pub mod usb_fs_host;
pub mod usb_fs_device;
pub mod usb_fs_pwrclk;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod exti;
pub mod tim_bas;
pub mod tim_gen;
pub mod tim_adv;
pub mod lptim;
pub mod adc;
pub mod spi;
pub mod i2c;
pub mod can;
pub mod gpio;
pub mod usart;
pub mod dma;
pub mod sig;
pub mod pin;
pub mod irq;

#[derive(Debug, Default)]
pub struct Stm32f74x {}

impl Mcu for Stm32f74x {
    fn id(&self) -> &'static str { "STM32F74x" }
}

impl Stm32f74x {
    pub fn rcc(&self) -> rcc::Rcc { rcc::RCC }
    pub fn flash(&self) -> flash::Flash { flash::FLASH }
    pub fn pwr(&self) -> pwr::Pwr { pwr::PWR }
    pub fn syscfg(&self) -> syscfg::Syscfg { syscfg::SYSCFG }
    pub fn dbg(&self) -> dbg::Dbg { dbg::DBG }
    pub fn ethernet_mac(&self) -> ethernet_mac::EthernetMac { ethernet_mac::ETHERNET_MAC }
    pub fn ethernet_mmc(&self) -> ethernet_mmc::EthernetMmc { ethernet_mmc::ETHERNET_MMC }
    pub fn ethernet_ptp(&self) -> ethernet_ptp::EthernetPtp { ethernet_ptp::ETHERNET_PTP }
    pub fn ethernet_dma(&self) -> ethernet_dma::EthernetDma { ethernet_dma::ETHERNET_DMA }
    pub fn dac(&self) -> dac::Dac { dac::DAC }
    pub fn sdmmc1(&self) -> sdmmc::Sdmmc { sdmmc::SDMMC1 }
    pub fn quadspi(&self) -> quadspi::Quadspi { quadspi::QUADSPI }
    pub fn cec(&self) -> cec::Cec { cec::CEC }
    pub fn spdif_rx(&self) -> spdif_rx::SpdifRx { spdif_rx::SPDIF_RX }
    pub fn ltdc(&self) -> ltdc::Ltdc { ltdc::LTDC }
    pub fn dma2d(&self) -> dma2d::Dma2d { dma2d::DMA2D }
    pub fn hash(&self) -> hash::Hash { hash::HASH }
    pub fn cryp(&self) -> cryp::Cryp { cryp::CRYP }
    pub fn c_adc(&self) -> c_adc::CAdc { c_adc::C_ADC }
    pub fn dcmi(&self) -> dcmi::Dcmi { dcmi::DCMI }
    pub fn usb_fs_global(&self) -> usb_fs_global::UsbFsGlobal { usb_fs_global::USB_FS_GLOBAL }
    pub fn usb_fs_host(&self) -> usb_fs_host::UsbFsHost { usb_fs_host::USB_FS_HOST }
    pub fn usb_fs_device(&self) -> usb_fs_device::UsbFsDevice { usb_fs_device::USB_FS_DEVICE }
    pub fn usb_fs_pwrclk(&self) -> usb_fs_pwrclk::UsbFsPwrclk { usb_fs_pwrclk::USB_FS_PWRCLK }
    pub fn iwdg(&self) -> iwdg::Iwdg { iwdg::IWDG }
    pub fn wwdg(&self) -> wwdg::Wwdg { wwdg::WWDG }
    pub fn crc(&self) -> crc::Crc { crc::CRC }
    pub fn exti(&self) -> exti::Exti { exti::EXTI }
    pub fn tim6(&self) -> tim_bas::Tim6 { tim_bas::TIM6 }
    pub fn tim7(&self) -> tim_bas::Tim7 { tim_bas::TIM7 }
    pub fn tim2(&self) -> tim_gen::Tim2 { tim_gen::TIM2 }
    pub fn tim3(&self) -> tim_gen::Tim3 { tim_gen::TIM3 }
    pub fn tim4(&self) -> tim_gen::Tim4 { tim_gen::TIM4 }
    pub fn tim5(&self) -> tim_gen::Tim5 { tim_gen::TIM5 }
    pub fn tim9(&self) -> tim_gen::Tim9 { tim_gen::TIM9 }
    pub fn tim10(&self) -> tim_gen::Tim10 { tim_gen::TIM10 }
    pub fn tim11(&self) -> tim_gen::Tim11 { tim_gen::TIM11 }
    pub fn tim12(&self) -> tim_gen::Tim12 { tim_gen::TIM12 }
    pub fn tim13(&self) -> tim_gen::Tim13 { tim_gen::TIM13 }
    pub fn tim14(&self) -> tim_gen::Tim14 { tim_gen::TIM14 }
    pub fn tim1(&self) -> tim_adv::Tim1 { tim_adv::TIM1 }
    pub fn tim8(&self) -> tim_adv::Tim8 { tim_adv::TIM8 }
    pub fn lptim1(&self) -> lptim::Lptim1 { lptim::LPTIM1 }
    pub fn adc1(&self) -> adc::Adc1 { adc::ADC1 }
    pub fn adc2(&self) -> adc::Adc2 { adc::ADC2 }
    pub fn adc3(&self) -> adc::Adc3 { adc::ADC3 }
    pub fn spi1(&self) -> spi::Spi1 { spi::SPI1 }
    pub fn spi2(&self) -> spi::Spi2 { spi::SPI2 }
    pub fn spi3(&self) -> spi::Spi3 { spi::SPI3 }
    pub fn i2s2ext(&self) -> spi::I2s2ext { spi::I2S2EXT }
    pub fn i2s3ext(&self) -> spi::I2s3ext { spi::I2S3EXT }
    pub fn spi4(&self) -> spi::Spi4 { spi::SPI4 }
    pub fn spi5(&self) -> spi::Spi5 { spi::SPI5 }
    pub fn spi6(&self) -> spi::Spi6 { spi::SPI6 }
    pub fn i2c1(&self) -> i2c::I2c1 { i2c::I2C1 }
    pub fn i2c2(&self) -> i2c::I2c2 { i2c::I2C2 }
    pub fn i2c3(&self) -> i2c::I2c3 { i2c::I2C3 }
    pub fn i2c4(&self) -> i2c::I2c4 { i2c::I2C4 }
    pub fn can1(&self) -> can::Can1 { can::CAN1 }
    pub fn can2(&self) -> can::Can2 { can::CAN2 }
    pub fn gpioa(&self) -> gpio::Gpioa { gpio::GPIOA }
    pub fn gpiob(&self) -> gpio::Gpiob { gpio::GPIOB }
    pub fn gpioc(&self) -> gpio::Gpioc { gpio::GPIOC }
    pub fn gpiod(&self) -> gpio::Gpiod { gpio::GPIOD }
    pub fn gpioe(&self) -> gpio::Gpioe { gpio::GPIOE }
    pub fn gpiof(&self) -> gpio::Gpiof { gpio::GPIOF }
    pub fn gpiog(&self) -> gpio::Gpiog { gpio::GPIOG }
    pub fn gpioh(&self) -> gpio::Gpioh { gpio::GPIOH }
    pub fn gpioi(&self) -> gpio::Gpioi { gpio::GPIOI }
    pub fn gpioj(&self) -> gpio::Gpioj { gpio::GPIOJ }
    pub fn gpiok(&self) -> gpio::Gpiok { gpio::GPIOK }
    pub fn usart1(&self) -> usart::Usart1 { usart::USART1 }
    pub fn usart2(&self) -> usart::Usart2 { usart::USART2 }
    pub fn usart3(&self) -> usart::Usart3 { usart::USART3 }
    pub fn uart4(&self) -> usart::Uart4 { usart::UART4 }
    pub fn uart5(&self) -> usart::Uart5 { usart::UART5 }
    pub fn usart6(&self) -> usart::Usart6 { usart::USART6 }
    pub fn uart7(&self) -> usart::Uart7 { usart::UART7 }
    pub fn uart8(&self) -> usart::Uart8 { usart::UART8 }
    pub fn dma1(&self) -> dma::Dma1 { dma::DMA1 }
    pub fn dma2(&self) -> dma::Dma2 { dma::DMA2 }
}

impl Get<rcc::Rcc> for Stm32f74x {
    fn get(&self) -> rcc::Rcc { rcc::RCC }
}

impl GetPeriph<rcc::RccPeriph> for Stm32f74x {
    fn get_periph(&self) -> rcc::RccPeriph { rcc::RCC_PERIPH }
}

impl GetPeriphInstance<rcc::RccPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<rcc::RccPeriph> {
        match index { 
            0 => Some(rcc::RCC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<flash::Flash> for Stm32f74x {
    fn get(&self) -> flash::Flash { flash::FLASH }
}

impl GetPeriph<flash::FlashPeriph> for Stm32f74x {
    fn get_periph(&self) -> flash::FlashPeriph { flash::FLASH_PERIPH }
}

impl GetPeriphInstance<flash::FlashPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<flash::FlashPeriph> {
        match index { 
            0 => Some(flash::FLASH_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<pwr::Pwr> for Stm32f74x {
    fn get(&self) -> pwr::Pwr { pwr::PWR }
}

impl GetPeriph<pwr::PwrPeriph> for Stm32f74x {
    fn get_periph(&self) -> pwr::PwrPeriph { pwr::PWR_PERIPH }
}

impl GetPeriphInstance<pwr::PwrPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<pwr::PwrPeriph> {
        match index { 
            0 => Some(pwr::PWR_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<syscfg::Syscfg> for Stm32f74x {
    fn get(&self) -> syscfg::Syscfg { syscfg::SYSCFG }
}

impl GetPeriph<syscfg::SyscfgPeriph> for Stm32f74x {
    fn get_periph(&self) -> syscfg::SyscfgPeriph { syscfg::SYSCFG_PERIPH }
}

impl GetPeriphInstance<syscfg::SyscfgPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<syscfg::SyscfgPeriph> {
        match index { 
            0 => Some(syscfg::SYSCFG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dbg::Dbg> for Stm32f74x {
    fn get(&self) -> dbg::Dbg { dbg::DBG }
}

impl GetPeriph<dbg::DbgPeriph> for Stm32f74x {
    fn get_periph(&self) -> dbg::DbgPeriph { dbg::DBG_PERIPH }
}

impl GetPeriphInstance<dbg::DbgPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<dbg::DbgPeriph> {
        match index { 
            0 => Some(dbg::DBG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<ethernet_mac::EthernetMac> for Stm32f74x {
    fn get(&self) -> ethernet_mac::EthernetMac { ethernet_mac::ETHERNET_MAC }
}

impl GetPeriph<ethernet_mac::EthernetMacPeriph> for Stm32f74x {
    fn get_periph(&self) -> ethernet_mac::EthernetMacPeriph { ethernet_mac::ETHERNET_MAC_PERIPH }
}

impl GetPeriphInstance<ethernet_mac::EthernetMacPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<ethernet_mac::EthernetMacPeriph> {
        match index { 
            0 => Some(ethernet_mac::ETHERNET_MAC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<ethernet_mmc::EthernetMmc> for Stm32f74x {
    fn get(&self) -> ethernet_mmc::EthernetMmc { ethernet_mmc::ETHERNET_MMC }
}

impl GetPeriph<ethernet_mmc::EthernetMmcPeriph> for Stm32f74x {
    fn get_periph(&self) -> ethernet_mmc::EthernetMmcPeriph { ethernet_mmc::ETHERNET_MMC_PERIPH }
}

impl GetPeriphInstance<ethernet_mmc::EthernetMmcPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<ethernet_mmc::EthernetMmcPeriph> {
        match index { 
            0 => Some(ethernet_mmc::ETHERNET_MMC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<ethernet_ptp::EthernetPtp> for Stm32f74x {
    fn get(&self) -> ethernet_ptp::EthernetPtp { ethernet_ptp::ETHERNET_PTP }
}

impl GetPeriph<ethernet_ptp::EthernetPtpPeriph> for Stm32f74x {
    fn get_periph(&self) -> ethernet_ptp::EthernetPtpPeriph { ethernet_ptp::ETHERNET_PTP_PERIPH }
}

impl GetPeriphInstance<ethernet_ptp::EthernetPtpPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<ethernet_ptp::EthernetPtpPeriph> {
        match index { 
            0 => Some(ethernet_ptp::ETHERNET_PTP_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<ethernet_dma::EthernetDma> for Stm32f74x {
    fn get(&self) -> ethernet_dma::EthernetDma { ethernet_dma::ETHERNET_DMA }
}

impl GetPeriph<ethernet_dma::EthernetDmaPeriph> for Stm32f74x {
    fn get_periph(&self) -> ethernet_dma::EthernetDmaPeriph { ethernet_dma::ETHERNET_DMA_PERIPH }
}

impl GetPeriphInstance<ethernet_dma::EthernetDmaPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<ethernet_dma::EthernetDmaPeriph> {
        match index { 
            0 => Some(ethernet_dma::ETHERNET_DMA_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dac::Dac> for Stm32f74x {
    fn get(&self) -> dac::Dac { dac::DAC }
}

impl GetPeriph<dac::DacPeriph> for Stm32f74x {
    fn get_periph(&self) -> dac::DacPeriph { dac::DAC_PERIPH }
}

impl GetPeriphInstance<dac::DacPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<dac::DacPeriph> {
        match index { 
            0 => Some(dac::DAC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<sdmmc::Sdmmc> for Stm32f74x {
    fn get(&self) -> sdmmc::Sdmmc { sdmmc::SDMMC1 }
}

impl GetPeriph<sdmmc::SdmmcPeriph> for Stm32f74x {
    fn get_periph(&self) -> sdmmc::SdmmcPeriph { sdmmc::SDMMC1_PERIPH }
}

impl GetPeriphInstance<sdmmc::SdmmcPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<sdmmc::SdmmcPeriph> {
        match index { 
            0 => Some(sdmmc::SDMMC1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<quadspi::Quadspi> for Stm32f74x {
    fn get(&self) -> quadspi::Quadspi { quadspi::QUADSPI }
}

impl GetPeriph<quadspi::QuadspiPeriph> for Stm32f74x {
    fn get_periph(&self) -> quadspi::QuadspiPeriph { quadspi::QUADSPI_PERIPH }
}

impl GetPeriphInstance<quadspi::QuadspiPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<quadspi::QuadspiPeriph> {
        match index { 
            0 => Some(quadspi::QUADSPI_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<cec::Cec> for Stm32f74x {
    fn get(&self) -> cec::Cec { cec::CEC }
}

impl GetPeriph<cec::CecPeriph> for Stm32f74x {
    fn get_periph(&self) -> cec::CecPeriph { cec::CEC_PERIPH }
}

impl GetPeriphInstance<cec::CecPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<cec::CecPeriph> {
        match index { 
            0 => Some(cec::CEC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<spdif_rx::SpdifRx> for Stm32f74x {
    fn get(&self) -> spdif_rx::SpdifRx { spdif_rx::SPDIF_RX }
}

impl GetPeriph<spdif_rx::SpdifRxPeriph> for Stm32f74x {
    fn get_periph(&self) -> spdif_rx::SpdifRxPeriph { spdif_rx::SPDIF_RX_PERIPH }
}

impl GetPeriphInstance<spdif_rx::SpdifRxPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<spdif_rx::SpdifRxPeriph> {
        match index { 
            0 => Some(spdif_rx::SPDIF_RX_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<ltdc::Ltdc> for Stm32f74x {
    fn get(&self) -> ltdc::Ltdc { ltdc::LTDC }
}

impl GetPeriph<ltdc::LtdcPeriph> for Stm32f74x {
    fn get_periph(&self) -> ltdc::LtdcPeriph { ltdc::LTDC_PERIPH }
}

impl GetPeriphInstance<ltdc::LtdcPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<ltdc::LtdcPeriph> {
        match index { 
            0 => Some(ltdc::LTDC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dma2d::Dma2d> for Stm32f74x {
    fn get(&self) -> dma2d::Dma2d { dma2d::DMA2D }
}

impl GetPeriph<dma2d::Dma2dPeriph> for Stm32f74x {
    fn get_periph(&self) -> dma2d::Dma2dPeriph { dma2d::DMA2D_PERIPH }
}

impl GetPeriphInstance<dma2d::Dma2dPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<dma2d::Dma2dPeriph> {
        match index { 
            0 => Some(dma2d::DMA2D_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<hash::Hash> for Stm32f74x {
    fn get(&self) -> hash::Hash { hash::HASH }
}

impl GetPeriph<hash::HashPeriph> for Stm32f74x {
    fn get_periph(&self) -> hash::HashPeriph { hash::HASH_PERIPH }
}

impl GetPeriphInstance<hash::HashPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<hash::HashPeriph> {
        match index { 
            0 => Some(hash::HASH_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<cryp::Cryp> for Stm32f74x {
    fn get(&self) -> cryp::Cryp { cryp::CRYP }
}

impl GetPeriph<cryp::CrypPeriph> for Stm32f74x {
    fn get_periph(&self) -> cryp::CrypPeriph { cryp::CRYP_PERIPH }
}

impl GetPeriphInstance<cryp::CrypPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<cryp::CrypPeriph> {
        match index { 
            0 => Some(cryp::CRYP_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<c_adc::CAdc> for Stm32f74x {
    fn get(&self) -> c_adc::CAdc { c_adc::C_ADC }
}

impl GetPeriph<c_adc::CAdcPeriph> for Stm32f74x {
    fn get_periph(&self) -> c_adc::CAdcPeriph { c_adc::C_ADC_PERIPH }
}

impl GetPeriphInstance<c_adc::CAdcPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<c_adc::CAdcPeriph> {
        match index { 
            0 => Some(c_adc::C_ADC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dcmi::Dcmi> for Stm32f74x {
    fn get(&self) -> dcmi::Dcmi { dcmi::DCMI }
}

impl GetPeriph<dcmi::DcmiPeriph> for Stm32f74x {
    fn get_periph(&self) -> dcmi::DcmiPeriph { dcmi::DCMI_PERIPH }
}

impl GetPeriphInstance<dcmi::DcmiPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<dcmi::DcmiPeriph> {
        match index {
            0 => Some(dcmi::DCMI_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<usb_fs_global::UsbFsGlobal> for Stm32f74x {
    fn get(&self) -> usb_fs_global::UsbFsGlobal { usb_fs_global::USB_FS_GLOBAL }
}

impl GetPeriph<usb_fs_global::UsbFsGlobalPeriph> for Stm32f74x {
    fn get_periph(&self) -> usb_fs_global::UsbFsGlobalPeriph { usb_fs_global::USB_FS_GLOBAL_PERIPH }
}

impl GetPeriphInstance<usb_fs_global::UsbFsGlobalPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<usb_fs_global::UsbFsGlobalPeriph> {
        match index {
            0 => Some(usb_fs_global::USB_FS_GLOBAL_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<usb_fs_host::UsbFsHost> for Stm32f74x {
    fn get(&self) -> usb_fs_host::UsbFsHost { usb_fs_host::USB_FS_HOST }
}

impl GetPeriph<usb_fs_host::UsbFsHostPeriph> for Stm32f74x {
    fn get_periph(&self) -> usb_fs_host::UsbFsHostPeriph { usb_fs_host::USB_FS_HOST_PERIPH }
}

impl GetPeriphInstance<usb_fs_host::UsbFsHostPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<usb_fs_host::UsbFsHostPeriph> {
        match index {
            0 => Some(usb_fs_host::USB_FS_HOST_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<usb_fs_device::UsbFsDevice> for Stm32f74x {
    fn get(&self) -> usb_fs_device::UsbFsDevice { usb_fs_device::USB_FS_DEVICE }
}

impl GetPeriph<usb_fs_device::UsbFsDevicePeriph> for Stm32f74x {
    fn get_periph(&self) -> usb_fs_device::UsbFsDevicePeriph { usb_fs_device::USB_FS_DEVICE_PERIPH }
}

impl GetPeriphInstance<usb_fs_device::UsbFsDevicePeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<usb_fs_device::UsbFsDevicePeriph> {
        match index {
            0 => Some(usb_fs_device::USB_FS_DEVICE_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<usb_fs_pwrclk::UsbFsPwrclk> for Stm32f74x {
    fn get(&self) -> usb_fs_pwrclk::UsbFsPwrclk { usb_fs_pwrclk::USB_FS_PWRCLK }
}

impl GetPeriph<usb_fs_pwrclk::UsbFsPwrclkPeriph> for Stm32f74x {
    fn get_periph(&self) -> usb_fs_pwrclk::UsbFsPwrclkPeriph { usb_fs_pwrclk::USB_FS_PWRCLK_PERIPH }
}

impl GetPeriphInstance<usb_fs_pwrclk::UsbFsPwrclkPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<usb_fs_pwrclk::UsbFsPwrclkPeriph> {
        match index {
            0 => Some(usb_fs_pwrclk::USB_FS_PWRCLK_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<iwdg::Iwdg> for Stm32f74x {
    fn get(&self) -> iwdg::Iwdg { iwdg::IWDG }
}

impl GetPeriph<iwdg::IwdgPeriph> for Stm32f74x {
    fn get_periph(&self) -> iwdg::IwdgPeriph { iwdg::IWDG_PERIPH }
}

impl GetPeriphInstance<iwdg::IwdgPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<iwdg::IwdgPeriph> {
        match index {
            0 => Some(iwdg::IWDG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<wwdg::Wwdg> for Stm32f74x {
    fn get(&self) -> wwdg::Wwdg { wwdg::WWDG }
}

impl GetPeriph<wwdg::WwdgPeriph> for Stm32f74x {
    fn get_periph(&self) -> wwdg::WwdgPeriph { wwdg::WWDG_PERIPH }
}

impl GetPeriphInstance<wwdg::WwdgPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<wwdg::WwdgPeriph> {
        match index {
            0 => Some(wwdg::WWDG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<crc::Crc> for Stm32f74x {
    fn get(&self) -> crc::Crc { crc::CRC }
}

impl GetPeriph<crc::CrcPeriph> for Stm32f74x {
    fn get_periph(&self) -> crc::CrcPeriph { crc::CRC_PERIPH }
}

impl GetPeriphInstance<crc::CrcPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<crc::CrcPeriph> {
        match index {
            0 => Some(crc::CRC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<exti::Exti> for Stm32f74x {
    fn get(&self) -> exti::Exti { exti::EXTI }
}

impl GetPeriph<exti::ExtiPeriph> for Stm32f74x {
    fn get_periph(&self) -> exti::ExtiPeriph { exti::EXTI_PERIPH }
}

impl GetPeriphInstance<exti::ExtiPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<exti::ExtiPeriph> {
        match index {
            0 => Some(exti::EXTI_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<tim_bas::Tim6> for Stm32f74x {
    fn get(&self) -> tim_bas::Tim6 { tim_bas::TIM6 }
}

impl Get<tim_bas::Tim7> for Stm32f74x {
    fn get(&self) -> tim_bas::Tim7 { tim_bas::TIM7 }
}

impl GetPeriphInstance<tim_bas::TimBasPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<tim_bas::TimBasPeriph> {
        match index {
            0 => Some(tim_bas::TIM6_PERIPH),
            1 => Some(tim_bas::TIM7_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<tim_gen::Tim2> for Stm32f74x {
    fn get(&self) -> tim_gen::Tim2 { tim_gen::TIM2 }
}

impl Get<tim_gen::Tim3> for Stm32f74x {
    fn get(&self) -> tim_gen::Tim3 { tim_gen::TIM3 }
}

impl Get<tim_gen::Tim4> for Stm32f74x {
    fn get(&self) -> tim_gen::Tim4 { tim_gen::TIM4 }
}

impl Get<tim_gen::Tim5> for Stm32f74x {
    fn get(&self) -> tim_gen::Tim5 { tim_gen::TIM5 }
}

impl Get<tim_gen::Tim9> for Stm32f74x {
    fn get(&self) -> tim_gen::Tim9 { tim_gen::TIM9 }
}

impl Get<tim_gen::Tim10> for Stm32f74x {
    fn get(&self) -> tim_gen::Tim10 { tim_gen::TIM10 }
}

impl Get<tim_gen::Tim11> for Stm32f74x {
    fn get(&self) -> tim_gen::Tim11 { tim_gen::TIM11 }
}

impl Get<tim_gen::Tim12> for Stm32f74x {
    fn get(&self) -> tim_gen::Tim12 { tim_gen::TIM12 }
}

impl Get<tim_gen::Tim13> for Stm32f74x {
    fn get(&self) -> tim_gen::Tim13 { tim_gen::TIM13 }
}

impl Get<tim_gen::Tim14> for Stm32f74x {
    fn get(&self) -> tim_gen::Tim14 { tim_gen::TIM14 }
}

impl GetPeriphInstance<tim_gen::TimGenPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<tim_gen::TimGenPeriph> {
        match index {
            0 => Some(tim_gen::TIM2_PERIPH),
            1 => Some(tim_gen::TIM3_PERIPH),
            2 => Some(tim_gen::TIM4_PERIPH),
            3 => Some(tim_gen::TIM5_PERIPH),
            4 => Some(tim_gen::TIM9_PERIPH),
            5 => Some(tim_gen::TIM10_PERIPH),
            6 => Some(tim_gen::TIM11_PERIPH),
            7 => Some(tim_gen::TIM12_PERIPH),
            8 => Some(tim_gen::TIM13_PERIPH),
            9 => Some(tim_gen::TIM14_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 10 }
}

impl Get<tim_adv::Tim1> for Stm32f74x {
    fn get(&self) -> tim_adv::Tim1 { tim_adv::TIM1 }
}

impl Get<tim_adv::Tim8> for Stm32f74x {
    fn get(&self) -> tim_adv::Tim8 { tim_adv::TIM8 }
}

impl GetPeriphInstance<tim_adv::TimAdvPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<tim_adv::TimAdvPeriph> {
        match index {
            0 => Some(tim_adv::TIM1_PERIPH),
            1 => Some(tim_adv::TIM8_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<lptim::Lptim1> for Stm32f74x {
    fn get(&self) -> lptim::Lptim1 { lptim::LPTIM1 }
}

impl GetPeriph<lptim::LptimPeriph> for Stm32f74x {
    fn get_periph(&self) -> lptim::LptimPeriph { lptim::LPTIM1_PERIPH }
}

impl GetPeriphInstance<lptim::LptimPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<lptim::LptimPeriph> {
        match index {
            0 => Some(lptim::LPTIM1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<adc::Adc1> for Stm32f74x {
    fn get(&self) -> adc::Adc1 { adc::ADC1 }
}

impl Get<adc::Adc2> for Stm32f74x {
    fn get(&self) -> adc::Adc2 { adc::ADC2 }
}

impl Get<adc::Adc3> for Stm32f74x {
    fn get(&self) -> adc::Adc3 { adc::ADC3 }
}

impl GetPeriphInstance<adc::AdcPeriph> for Stm32f74x {
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

impl Get<spi::Spi1> for Stm32f74x {
    fn get(&self) -> spi::Spi1 { spi::SPI1 }
}

impl Get<spi::Spi2> for Stm32f74x {
    fn get(&self) -> spi::Spi2 { spi::SPI2 }
}

impl Get<spi::Spi3> for Stm32f74x {
    fn get(&self) -> spi::Spi3 { spi::SPI3 }
}

impl Get<spi::I2s2ext> for Stm32f74x {
    fn get(&self) -> spi::I2s2ext { spi::I2S2EXT }
}

impl Get<spi::I2s3ext> for Stm32f74x {
    fn get(&self) -> spi::I2s3ext { spi::I2S3EXT }
}

impl Get<spi::Spi4> for Stm32f74x {
    fn get(&self) -> spi::Spi4 { spi::SPI4 }
}

impl Get<spi::Spi5> for Stm32f74x {
    fn get(&self) -> spi::Spi5 { spi::SPI5 }
}

impl Get<spi::Spi6> for Stm32f74x {
    fn get(&self) -> spi::Spi6 { spi::SPI6 }
}

impl GetPeriphInstance<spi::SpiPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<spi::SpiPeriph> {
        match index {
            0 => Some(spi::SPI1_PERIPH),
            1 => Some(spi::SPI2_PERIPH),
            2 => Some(spi::SPI3_PERIPH),
            3 => Some(spi::I2S2EXT_PERIPH),
            4 => Some(spi::I2S3EXT_PERIPH),
            5 => Some(spi::SPI4_PERIPH),
            6 => Some(spi::SPI5_PERIPH),
            7 => Some(spi::SPI6_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 8 }
}

impl Get<i2c::I2c1> for Stm32f74x {
    fn get(&self) -> i2c::I2c1 { i2c::I2C1 }
}

impl Get<i2c::I2c2> for Stm32f74x {
    fn get(&self) -> i2c::I2c2 { i2c::I2C2 }
}

impl Get<i2c::I2c3> for Stm32f74x {
    fn get(&self) -> i2c::I2c3 { i2c::I2C3 }
}

impl Get<i2c::I2c4> for Stm32f74x {
    fn get(&self) -> i2c::I2c4 { i2c::I2C4 }
}

impl GetPeriphInstance<i2c::I2cPeriph> for Stm32f74x {
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

impl Get<can::Can1> for Stm32f74x {
    fn get(&self) -> can::Can1 { can::CAN1 }
}

impl Get<can::Can2> for Stm32f74x {
    fn get(&self) -> can::Can2 { can::CAN2 }
}

impl GetPeriphInstance<can::CanPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<can::CanPeriph> {
        match index {
            0 => Some(can::CAN1_PERIPH),
            1 => Some(can::CAN2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<gpio::Gpioa> for Stm32f74x {
    fn get(&self) -> gpio::Gpioa { gpio::GPIOA }
}

impl Get<gpio::Gpiob> for Stm32f74x {
    fn get(&self) -> gpio::Gpiob { gpio::GPIOB }
}

impl Get<gpio::Gpioc> for Stm32f74x {
    fn get(&self) -> gpio::Gpioc { gpio::GPIOC }
}

impl Get<gpio::Gpiod> for Stm32f74x {
    fn get(&self) -> gpio::Gpiod { gpio::GPIOD }
}

impl Get<gpio::Gpioe> for Stm32f74x {
    fn get(&self) -> gpio::Gpioe { gpio::GPIOE }
}

impl Get<gpio::Gpiof> for Stm32f74x {
    fn get(&self) -> gpio::Gpiof { gpio::GPIOF }
}

impl Get<gpio::Gpiog> for Stm32f74x {
    fn get(&self) -> gpio::Gpiog { gpio::GPIOG }
}

impl Get<gpio::Gpioh> for Stm32f74x {
    fn get(&self) -> gpio::Gpioh { gpio::GPIOH }
}

impl Get<gpio::Gpioi> for Stm32f74x {
    fn get(&self) -> gpio::Gpioi { gpio::GPIOI }
}

impl Get<gpio::Gpioj> for Stm32f74x {
    fn get(&self) -> gpio::Gpioj { gpio::GPIOJ }
}

impl Get<gpio::Gpiok> for Stm32f74x {
    fn get(&self) -> gpio::Gpiok { gpio::GPIOK }
}

impl GetPeriphInstance<gpio::GpioPeriph> for Stm32f74x {
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
            8 => Some(gpio::GPIOI_PERIPH),
            9 => Some(gpio::GPIOJ_PERIPH),
            10 => Some(gpio::GPIOK_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 11 }
}

impl Get<usart::Usart1> for Stm32f74x {
    fn get(&self) -> usart::Usart1 { usart::USART1 }
}

impl Get<usart::Usart2> for Stm32f74x {
    fn get(&self) -> usart::Usart2 { usart::USART2 }
}

impl Get<usart::Usart3> for Stm32f74x {
    fn get(&self) -> usart::Usart3 { usart::USART3 }
}

impl Get<usart::Uart4> for Stm32f74x {
    fn get(&self) -> usart::Uart4 { usart::UART4 }
}

impl Get<usart::Uart5> for Stm32f74x {
    fn get(&self) -> usart::Uart5 { usart::UART5 }
}

impl Get<usart::Usart6> for Stm32f74x {
    fn get(&self) -> usart::Usart6 { usart::USART6 }
}

impl Get<usart::Uart7> for Stm32f74x {
    fn get(&self) -> usart::Uart7 { usart::UART7 }
}

impl Get<usart::Uart8> for Stm32f74x {
    fn get(&self) -> usart::Uart8 { usart::UART8 }
}

impl GetPeriphInstance<usart::UsartPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<usart::UsartPeriph> {
        match index {
            0 => Some(usart::USART1_PERIPH),
            1 => Some(usart::USART2_PERIPH),
            2 => Some(usart::USART3_PERIPH),
            3 => Some(usart::UART4_PERIPH),
            4 => Some(usart::UART5_PERIPH),
            5 => Some(usart::USART6_PERIPH),
            6 => Some(usart::UART7_PERIPH),
            7 => Some(usart::UART8_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 8 }
}

impl Get<dma::Dma1> for Stm32f74x {
    fn get(&self) -> dma::Dma1 { dma::DMA1 }
}

impl Get<dma::Dma2> for Stm32f74x {
    fn get(&self) -> dma::Dma2 { dma::DMA2 }
}

impl GetPeriphInstance<dma::DmaPeriph> for Stm32f74x {
    fn get_periph_instance(&self, index: usize) -> Option<dma::DmaPeriph> {
        match index {
            0 => Some(dma::DMA1_PERIPH),
            1 => Some(dma::DMA2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

