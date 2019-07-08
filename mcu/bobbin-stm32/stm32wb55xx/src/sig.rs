::bobbin_mcu::signal_type!(I2C_SMBA, SigI2cSmba);
::bobbin_mcu::signal_type!(I2C_SCL, SigI2cScl);
::bobbin_mcu::signal_type!(I2C_SDA, SigI2cSda);
::bobbin_mcu::signal_type!(ADC, SigAdc);
::bobbin_mcu::signal_type!(TX, SigTx);
::bobbin_mcu::signal_type!(RX, SigRx);
::bobbin_mcu::signal_type!(CTS, SigCts);
::bobbin_mcu::signal_type!(RTS, SigRts);
::bobbin_mcu::signal_type!(CK, SigCk);
::bobbin_mcu::signal_type!(SPI_NSS, SigSpiNss);
::bobbin_mcu::signal_type!(SPI_MISO, SigSpiMiso);
::bobbin_mcu::signal_type!(SPI_MOSI, SigSpiMosi);
::bobbin_mcu::signal_type!(SPI_SCK, SigSpiSck);

// DMA

// CRC

// IWDG

// WWDG

// I2C
::bobbin_mcu::periph_signal!(super::i2c::I2c1, SigI2cSmba);
::bobbin_mcu::periph_signal!(super::i2c::I2c1, SigI2cScl);
::bobbin_mcu::periph_signal!(super::i2c::I2c1, SigI2cSda);

// RNG

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

// GPIO

// LPTIM

// USART
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigTx);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigRx);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigCts);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigRts);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigCk);

// LPUART
::bobbin_mcu::periph_signal!(super::lpuart::Lpuart1, SigTx);
::bobbin_mcu::periph_signal!(super::lpuart::Lpuart1, SigRx);
::bobbin_mcu::periph_signal!(super::lpuart::Lpuart1, SigCts);
::bobbin_mcu::periph_signal!(super::lpuart::Lpuart1, SigRts);

// SPI
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiNss);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiMiso);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiMosi);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiSck);

// RTC

// EXTI

// DMAMUX

// LCD

// TSC

// COMP

// QUADSPI

// AES1

// Hardware_Semaphore

// SAI

// TIM2

// TIM16

// TIM17

// TIM1

// VREF

// USB

// CRS

// DBGMCU

// PKA

// IPCC









































































