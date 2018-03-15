#[allow(unused_imports)] pub use ::bobbin_common::*;
signal_type!(I2C_SMBA, SigI2cSmba);
signal_type!(I2C_SCL, SigI2cScl);
signal_type!(I2C_SDA, SigI2cSda);
signal_type!(ETR, SigEtr);
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
signal_type!(ADC, SigAdc);

// IWDG

// WWDG

// CRC

// RTC

// LPTIM

// EXTI

// DMA

// I2C
periph_signal!(super::i2c::I2c1, SigI2cSmba);
periph_signal!(super::i2c::I2c1, SigI2cScl);
periph_signal!(super::i2c::I2c1, SigI2cSda);

// TIM_GEN
periph_signal!(super::tim_gen::Tim2, SigEtr);
channel_signal!(super::tim_gen::Tim2Ch1, SigTim);
channel_signal!(super::tim_gen::Tim2Ch2, SigTim);
channel_signal!(super::tim_gen::Tim2Ch3, SigTim);
channel_signal!(super::tim_gen::Tim2Ch4, SigTim);
periph_signal!(super::tim_gen::Tim21, SigEtr);
channel_signal!(super::tim_gen::Tim21Ch1, SigTim);
channel_signal!(super::tim_gen::Tim21Ch2, SigTim);
periph_signal!(super::tim_gen::Tim22, SigEtr);
channel_signal!(super::tim_gen::Tim22Ch1, SigTim);
channel_signal!(super::tim_gen::Tim22Ch2, SigTim);

// GPIO

// USART
periph_signal!(super::usart::Usart2, SigTx);
periph_signal!(super::usart::Usart2, SigRx);
periph_signal!(super::usart::Usart2, SigCts);
periph_signal!(super::usart::Usart2, SigRts);
periph_signal!(super::usart::Usart2, SigCk);

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

// ADC
channel_signal!(super::adc::Adc1Ch0, SigAdc);
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






































