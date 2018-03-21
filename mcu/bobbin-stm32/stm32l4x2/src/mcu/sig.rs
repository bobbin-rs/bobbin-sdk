#[allow(unused_imports)] pub use ::bobbin_common::*;

signal_type!(I2C_SMBA, SigI2cSmba);
signal_type!(I2C_SCL, SigI2cScl);
signal_type!(I2C_SDA, SigI2cSda);
signal_type!(ADC, SigAdc);
signal_type!(TIM, SigTim);
signal_type!(TX, SigTx);
signal_type!(RX, SigRx);
signal_type!(CTS, SigCts);
signal_type!(RTS, SigRts);
signal_type!(CK, SigCk);
signal_type!(SPI_NSS, SigSpiNss);
signal_type!(SPI_MISO, SigSpiMiso);
signal_type!(SPI_MOSI, SigSpiMosi);
signal_type!(SPI_SCK, SigSpiSck);
signal_type!(DFSDM_CKOUT, SigDfsdmCkout);
signal_type!(DFSDM_CKIN0, SigDfsdmCkin0);
signal_type!(DFSDM_CKIN1, SigDfsdmCkin1);
signal_type!(DFSDM_CKIN2, SigDfsdmCkin2);
signal_type!(DFSDM_CKIN3, SigDfsdmCkin3);
signal_type!(DFSDM_DATIN0, SigDfsdmDatin0);
signal_type!(DFSDM_DATIN1, SigDfsdmDatin1);
signal_type!(DFSDM_DATIN2, SigDfsdmDatin2);
signal_type!(DFSDM_DATIN3, SigDfsdmDatin3);

// DAC

// DMA

// CRC

// LCD

// TSC

// IWDG

// WWDG

// COMP

// Firewall

// I2C
periph_signal!(super::i2c::I2c1, SigI2cSmba);
periph_signal!(super::i2c::I2c1, SigI2cScl);
periph_signal!(super::i2c::I2c1, SigI2cSda);
periph_signal!(super::i2c::I2c2, SigI2cSmba);
periph_signal!(super::i2c::I2c2, SigI2cScl);
periph_signal!(super::i2c::I2c2, SigI2cSda);
periph_signal!(super::i2c::I2c3, SigI2cSmba);
periph_signal!(super::i2c::I2c3, SigI2cScl);
periph_signal!(super::i2c::I2c3, SigI2cSda);
periph_signal!(super::i2c::I2c4, SigI2cSmba);
periph_signal!(super::i2c::I2c4, SigI2cScl);
periph_signal!(super::i2c::I2c4, SigI2cSda);

// Flash

// PWR

// SYSCFG

// RNG

// AES

// ADC
channel_signal!(super::adc::Adc1Ch1, SigAdc);
channel_signal!(super::adc::Adc1Ch2, SigAdc);
channel_signal!(super::adc::Adc1Ch3, SigAdc);
channel_signal!(super::adc::Adc1Ch4, SigAdc);
channel_signal!(super::adc::Adc1Ch5, SigAdc);
channel_signal!(super::adc::Adc1Ch6, SigAdc);
channel_signal!(super::adc::Adc1Ch7, SigAdc);
channel_signal!(super::adc::Adc1Ch8, SigAdc);
channel_signal!(super::adc::Adc1Ch9, SigAdc);
channel_signal!(super::adc::Adc1Ch10, SigAdc);
channel_signal!(super::adc::Adc1Ch11, SigAdc);
channel_signal!(super::adc::Adc1Ch12, SigAdc);
channel_signal!(super::adc::Adc1Ch13, SigAdc);
channel_signal!(super::adc::Adc1Ch14, SigAdc);
channel_signal!(super::adc::Adc1Ch15, SigAdc);

// GPIO

// SAI

// TIM_GEN
channel_signal!(super::tim_gen::Tim2Ch1, SigTim);
channel_signal!(super::tim_gen::Tim2Ch2, SigTim);
channel_signal!(super::tim_gen::Tim2Ch3, SigTim);
channel_signal!(super::tim_gen::Tim2Ch4, SigTim);
channel_signal!(super::tim_gen::Tim3Ch1, SigTim);
channel_signal!(super::tim_gen::Tim3Ch2, SigTim);
channel_signal!(super::tim_gen::Tim3Ch3, SigTim);
channel_signal!(super::tim_gen::Tim3Ch4, SigTim);
channel_signal!(super::tim_gen::Tim15Ch1, SigTim);
channel_signal!(super::tim_gen::Tim15Ch2, SigTim);
channel_signal!(super::tim_gen::Tim16Ch1, SigTim);

// TIM_ADV
channel_signal!(super::tim_adv::Tim1Ch1, SigTim);
channel_signal!(super::tim_adv::Tim1Ch2, SigTim);
channel_signal!(super::tim_adv::Tim1Ch3, SigTim);
channel_signal!(super::tim_adv::Tim1Ch4, SigTim);

// TIM_BAS

// LPTIM

// USART
periph_signal!(super::usart::Usart1, SigTx);
periph_signal!(super::usart::Usart1, SigRx);
periph_signal!(super::usart::Usart1, SigCts);
periph_signal!(super::usart::Usart1, SigRts);
periph_signal!(super::usart::Usart1, SigCk);
periph_signal!(super::usart::Usart2, SigTx);
periph_signal!(super::usart::Usart2, SigRx);
periph_signal!(super::usart::Usart2, SigCts);
periph_signal!(super::usart::Usart2, SigRts);
periph_signal!(super::usart::Usart2, SigCk);
periph_signal!(super::usart::Usart3, SigTx);
periph_signal!(super::usart::Usart3, SigRx);
periph_signal!(super::usart::Usart3, SigCts);
periph_signal!(super::usart::Usart3, SigRts);
periph_signal!(super::usart::Usart3, SigCk);
periph_signal!(super::usart::Uart4, SigTx);
periph_signal!(super::usart::Uart4, SigRx);
periph_signal!(super::usart::Uart4, SigCts);
periph_signal!(super::usart::Uart4, SigRts);
periph_signal!(super::usart::Uart4, SigCk);

// LPUART
periph_signal!(super::lpuart::Lpuart1, SigTx);
periph_signal!(super::lpuart::Lpuart1, SigRx);
periph_signal!(super::lpuart::Lpuart1, SigCts);
periph_signal!(super::lpuart::Lpuart1, SigRts);

// SPI
periph_signal!(super::spi::Spi1, SigSpiNss);
periph_signal!(super::spi::Spi1, SigSpiMiso);
periph_signal!(super::spi::Spi1, SigSpiMosi);
periph_signal!(super::spi::Spi1, SigSpiSck);
periph_signal!(super::spi::Spi2, SigSpiNss);
periph_signal!(super::spi::Spi2, SigSpiMiso);
periph_signal!(super::spi::Spi2, SigSpiMosi);
periph_signal!(super::spi::Spi2, SigSpiSck);
periph_signal!(super::spi::Spi3, SigSpiNss);
periph_signal!(super::spi::Spi3, SigSpiMiso);
periph_signal!(super::spi::Spi3, SigSpiMosi);
periph_signal!(super::spi::Spi3, SigSpiSck);

// SDIO

// EXTI

// VREF

// CAN

// RTC

// SWPMI

// OPAMP

// CRS

// USB

// DFSDM
periph_signal!(super::dfsdm::Dfsdm, SigDfsdmCkout);
periph_signal!(super::dfsdm::Dfsdm, SigDfsdmCkin0);
periph_signal!(super::dfsdm::Dfsdm, SigDfsdmCkin1);
periph_signal!(super::dfsdm::Dfsdm, SigDfsdmCkin2);
periph_signal!(super::dfsdm::Dfsdm, SigDfsdmCkin3);
periph_signal!(super::dfsdm::Dfsdm, SigDfsdmDatin0);
periph_signal!(super::dfsdm::Dfsdm, SigDfsdmDatin1);
periph_signal!(super::dfsdm::Dfsdm, SigDfsdmDatin2);
periph_signal!(super::dfsdm::Dfsdm, SigDfsdmDatin3);

// QUADSPI

// DBGMCU




















































































