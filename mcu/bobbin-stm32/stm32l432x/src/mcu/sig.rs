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

// RNG

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

// TIM_ADV

// TIM_BAS

// TIM_GEN
periph_signal!(super::tim_gen::Tim2, SigEtr);
channel_signal!(super::tim_gen::Tim2Ch1, SigTim);
channel_signal!(super::tim_gen::Tim2Ch2, SigTim);
channel_signal!(super::tim_gen::Tim2Ch3, SigTim);
channel_signal!(super::tim_gen::Tim2Ch4, SigTim);
channel_signal!(super::tim_gen::Tim3Ch1, SigTim);
channel_signal!(super::tim_gen::Tim3Ch2, SigTim);
channel_signal!(super::tim_gen::Tim3Ch3, SigTim);
channel_signal!(super::tim_gen::Tim3Ch4, SigTim);
channel_signal!(super::tim_gen::Tim4Ch1, SigTim);
channel_signal!(super::tim_gen::Tim4Ch2, SigTim);
channel_signal!(super::tim_gen::Tim4Ch3, SigTim);
channel_signal!(super::tim_gen::Tim4Ch4, SigTim);
channel_signal!(super::tim_gen::Tim5Ch1, SigTim);
channel_signal!(super::tim_gen::Tim5Ch2, SigTim);
channel_signal!(super::tim_gen::Tim5Ch3, SigTim);
channel_signal!(super::tim_gen::Tim5Ch4, SigTim);
channel_signal!(super::tim_gen::Tim15Ch1, SigTim);
channel_signal!(super::tim_gen::Tim15Ch2, SigTim);
channel_signal!(super::tim_gen::Tim16Ch1, SigTim);
channel_signal!(super::tim_gen::Tim16Ch2, SigTim);

// GPIO

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
periph_signal!(super::usart::Uart5, SigTx);
periph_signal!(super::usart::Uart5, SigRx);
periph_signal!(super::usart::Uart5, SigCts);
periph_signal!(super::usart::Uart5, SigRts);
periph_signal!(super::usart::Uart5, SigCk);

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
channel_signal!(super::adc::Adc2Ch1, SigAdc);
channel_signal!(super::adc::Adc2Ch2, SigAdc);
channel_signal!(super::adc::Adc2Ch3, SigAdc);
channel_signal!(super::adc::Adc2Ch4, SigAdc);
channel_signal!(super::adc::Adc2Ch5, SigAdc);
channel_signal!(super::adc::Adc2Ch6, SigAdc);
channel_signal!(super::adc::Adc2Ch7, SigAdc);
channel_signal!(super::adc::Adc2Ch8, SigAdc);
channel_signal!(super::adc::Adc2Ch9, SigAdc);
channel_signal!(super::adc::Adc2Ch10, SigAdc);
channel_signal!(super::adc::Adc2Ch11, SigAdc);
channel_signal!(super::adc::Adc2Ch12, SigAdc);
channel_signal!(super::adc::Adc2Ch13, SigAdc);
channel_signal!(super::adc::Adc2Ch14, SigAdc);
channel_signal!(super::adc::Adc2Ch15, SigAdc);
channel_signal!(super::adc::Adc3Ch1, SigAdc);
channel_signal!(super::adc::Adc3Ch2, SigAdc);
channel_signal!(super::adc::Adc3Ch3, SigAdc);
channel_signal!(super::adc::Adc3Ch4, SigAdc);
channel_signal!(super::adc::Adc3Ch5, SigAdc);
channel_signal!(super::adc::Adc3Ch6, SigAdc);
channel_signal!(super::adc::Adc3Ch7, SigAdc);
channel_signal!(super::adc::Adc3Ch8, SigAdc);
channel_signal!(super::adc::Adc3Ch9, SigAdc);
channel_signal!(super::adc::Adc3Ch10, SigAdc);
channel_signal!(super::adc::Adc3Ch11, SigAdc);
channel_signal!(super::adc::Adc3Ch12, SigAdc);
channel_signal!(super::adc::Adc3Ch13, SigAdc);
channel_signal!(super::adc::Adc3Ch14, SigAdc);
channel_signal!(super::adc::Adc3Ch15, SigAdc);




























