::bobbin_mcu::signal_type!(TX, SigTx);
::bobbin_mcu::signal_type!(RX, SigRx);
::bobbin_mcu::signal_type!(CTS, SigCts);
::bobbin_mcu::signal_type!(RTS, SigRts);
::bobbin_mcu::signal_type!(CK, SigCk);
::bobbin_mcu::signal_type!(I2C_SCL, SigI2cScl);
::bobbin_mcu::signal_type!(I2C_SDA, SigI2cSda);
::bobbin_mcu::signal_type!(I2C_SMBAL, SigI2cSmbal);
::bobbin_mcu::signal_type!(SPI_NSS, SigSpiNss);
::bobbin_mcu::signal_type!(SPI_MISO, SigSpiMiso);
::bobbin_mcu::signal_type!(SPI_MOSI, SigSpiMosi);
::bobbin_mcu::signal_type!(SPI_SCK, SigSpiSck);
::bobbin_mcu::signal_type!(ADC, SigAdc);
::bobbin_mcu::signal_type!(DAC, SigDac);
::bobbin_mcu::signal_type!(TIM, SigTim);
::bobbin_mcu::signal_type!(TIMN, SigTimn);

// IWDG

// WWDG

// CRC

// EXTI

// GPIO

// USART
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigTx);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigRx);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigCts);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigRts);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigCk);
::bobbin_mcu::periph_signal!(super::usart::Usart2, SigTx);
::bobbin_mcu::periph_signal!(super::usart::Usart2, SigRx);
::bobbin_mcu::periph_signal!(super::usart::Usart2, SigCts);
::bobbin_mcu::periph_signal!(super::usart::Usart2, SigRts);
::bobbin_mcu::periph_signal!(super::usart::Usart2, SigCk);
::bobbin_mcu::periph_signal!(super::usart::Usart3, SigTx);
::bobbin_mcu::periph_signal!(super::usart::Usart3, SigRx);
::bobbin_mcu::periph_signal!(super::usart::Usart3, SigCts);
::bobbin_mcu::periph_signal!(super::usart::Usart3, SigRts);
::bobbin_mcu::periph_signal!(super::usart::Usart3, SigCk);
::bobbin_mcu::periph_signal!(super::usart::Uart4, SigTx);
::bobbin_mcu::periph_signal!(super::usart::Uart4, SigRx);
::bobbin_mcu::periph_signal!(super::usart::Uart4, SigCts);
::bobbin_mcu::periph_signal!(super::usart::Uart4, SigRts);
::bobbin_mcu::periph_signal!(super::usart::Uart4, SigCk);
::bobbin_mcu::periph_signal!(super::usart::Uart5, SigTx);
::bobbin_mcu::periph_signal!(super::usart::Uart5, SigRx);
::bobbin_mcu::periph_signal!(super::usart::Uart5, SigCts);
::bobbin_mcu::periph_signal!(super::usart::Uart5, SigRts);
::bobbin_mcu::periph_signal!(super::usart::Uart5, SigCk);

// I2C
::bobbin_mcu::periph_signal!(super::i2c::I2c1, SigI2cScl);
::bobbin_mcu::periph_signal!(super::i2c::I2c1, SigI2cSda);
::bobbin_mcu::periph_signal!(super::i2c::I2c1, SigI2cSmbal);
::bobbin_mcu::periph_signal!(super::i2c::I2c2, SigI2cScl);
::bobbin_mcu::periph_signal!(super::i2c::I2c2, SigI2cSda);
::bobbin_mcu::periph_signal!(super::i2c::I2c2, SigI2cSmbal);
::bobbin_mcu::periph_signal!(super::i2c::I2c3, SigI2cScl);
::bobbin_mcu::periph_signal!(super::i2c::I2c3, SigI2cSda);
::bobbin_mcu::periph_signal!(super::i2c::I2c3, SigI2cSmbal);

// SPI
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiNss);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiMiso);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiMosi);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiSck);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiNss);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiMiso);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiMosi);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiSck);
::bobbin_mcu::periph_signal!(super::spi::Spi3, SigSpiNss);
::bobbin_mcu::periph_signal!(super::spi::Spi3, SigSpiMiso);
::bobbin_mcu::periph_signal!(super::spi::Spi3, SigSpiMosi);
::bobbin_mcu::periph_signal!(super::spi::Spi3, SigSpiSck);

// ADC
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch1, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch2, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch3, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch4, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch5, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch6, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch7, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch8, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch9, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch10, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch11, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch12, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch13, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch14, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch15, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch1, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch2, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch3, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch4, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch5, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch6, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch7, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch8, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch9, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch10, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch11, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch12, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch13, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch14, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch15, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch1, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch2, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch3, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch4, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch5, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch6, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch7, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch8, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch9, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch10, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch11, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch12, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch13, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch14, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch15, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch1, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch2, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch3, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch4, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch5, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch6, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch7, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch8, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch9, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch10, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch11, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch12, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch13, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch14, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc4Ch15, SigAdc);

// C_ADC

// DAC
::bobbin_mcu::channel_signal!(super::dac::DacCh1, SigDac);
::bobbin_mcu::channel_signal!(super::dac::DacCh2, SigDac);

// TIM_BAS

// TIM_GEN
::bobbin_mcu::channel_signal!(super::tim_gen::Tim2Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim2Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim2Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim2Ch4, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim3Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim3Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim3Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim3Ch4, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim4Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim4Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim4Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim4Ch4, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim15Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim15Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim16Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim16Ch1, SigTimn);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim17Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim17Ch1, SigTimn);

// TIM_ADV
::bobbin_mcu::channel_signal!(super::tim_adv::Tim1Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim1Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim1Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim1Ch4, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim8Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim8Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim8Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim8Ch4, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim20Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim20Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim20Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim20Ch4, SigTim);

// DMA




















































































































