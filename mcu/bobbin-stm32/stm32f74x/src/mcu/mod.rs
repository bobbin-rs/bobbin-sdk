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

pub struct Mcu {}
pub const MCU: Mcu = Mcu {};

impl Mcu {
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
