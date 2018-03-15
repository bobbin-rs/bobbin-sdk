#[allow(unused_imports)] pub use ::bobbin_common::*;
signal_type!(TPM, SigTpm);
signal_type!(LPTMR_ALT1, SigLptmrAlt1);
signal_type!(LPTMR_ALT2, SigLptmrAlt2);
signal_type!(SPI_SCK, SigSpiSck);
signal_type!(SPI_MISO, SigSpiMiso);
signal_type!(SPI_MOSI, SigSpiMosi);
signal_type!(SPI_PCS0, SigSpiPcs0);
signal_type!(SPI_SOUT, SigSpiSout);
signal_type!(SPI_SIN, SigSpiSin);
signal_type!(SPI_PCS1, SigSpiPcs1);
signal_type!(SPI_PCS2, SigSpiPcs2);
signal_type!(SPI_PCS3, SigSpiPcs3);
signal_type!(I2C_SCL, SigI2cScl);
signal_type!(I2C_SDA, SigI2cSda);
signal_type!(TX, SigTx);
signal_type!(RX, SigRx);
signal_type!(ADC_DP, SigAdcDp);
signal_type!(ADC_DM, SigAdcDm);
signal_type!(ADC_SEA, SigAdcSea);
signal_type!(ADC_SEB, SigAdcSeb);
signal_type!(ADC, SigAdc);

// DMAMUX

// DMA

// TPM
channel_signal!(super::tpm::Tpm0Ch0, SigTpm);
channel_signal!(super::tpm::Tpm0Ch1, SigTpm);
channel_signal!(super::tpm::Tpm0Ch2, SigTpm);
channel_signal!(super::tpm::Tpm0Ch3, SigTpm);
channel_signal!(super::tpm::Tpm0Ch4, SigTpm);
channel_signal!(super::tpm::Tpm0Ch5, SigTpm);
channel_signal!(super::tpm::Tpm1Ch0, SigTpm);
channel_signal!(super::tpm::Tpm1Ch1, SigTpm);
channel_signal!(super::tpm::Tpm1Ch2, SigTpm);
channel_signal!(super::tpm::Tpm1Ch3, SigTpm);
channel_signal!(super::tpm::Tpm1Ch4, SigTpm);
channel_signal!(super::tpm::Tpm1Ch5, SigTpm);
channel_signal!(super::tpm::Tpm2Ch0, SigTpm);
channel_signal!(super::tpm::Tpm2Ch1, SigTpm);
channel_signal!(super::tpm::Tpm2Ch2, SigTpm);
channel_signal!(super::tpm::Tpm2Ch3, SigTpm);
channel_signal!(super::tpm::Tpm2Ch4, SigTpm);
channel_signal!(super::tpm::Tpm2Ch5, SigTpm);

// PIT

// LPTMR
periph_signal!(super::lptmr::Lptmr0, SigLptmrAlt1);
periph_signal!(super::lptmr::Lptmr0, SigLptmrAlt2);

// SPI
periph_signal!(super::spi::Spi0, SigSpiSck);
periph_signal!(super::spi::Spi0, SigSpiMiso);
periph_signal!(super::spi::Spi0, SigSpiMosi);
periph_signal!(super::spi::Spi0, SigSpiPcs0);
periph_signal!(super::spi::Spi1, SigSpiSck);
periph_signal!(super::spi::Spi1, SigSpiSout);
periph_signal!(super::spi::Spi1, SigSpiSin);
periph_signal!(super::spi::Spi1, SigSpiPcs0);
periph_signal!(super::spi::Spi1, SigSpiPcs1);
periph_signal!(super::spi::Spi1, SigSpiPcs2);
periph_signal!(super::spi::Spi1, SigSpiPcs3);

// I2C
periph_signal!(super::i2c::I2c0, SigI2cScl);
periph_signal!(super::i2c::I2c0, SigI2cSda);
periph_signal!(super::i2c::I2c1, SigI2cScl);
periph_signal!(super::i2c::I2c1, SigI2cSda);

// PORT

// GPIO

// UART0
periph_signal!(super::uart0::Uart0, SigTx);
periph_signal!(super::uart0::Uart0, SigRx);

// UART
periph_signal!(super::uart::Uart1, SigTx);
periph_signal!(super::uart::Uart1, SigRx);
periph_signal!(super::uart::Uart2, SigTx);
periph_signal!(super::uart::Uart2, SigRx);

// ADC
channel_signal!(super::adc::Adc0Ch0, SigAdcDp);
channel_signal!(super::adc::Adc0Ch0, SigAdcDm);
channel_signal!(super::adc::Adc0Ch1, SigAdcDp);
channel_signal!(super::adc::Adc0Ch1, SigAdcDm);
channel_signal!(super::adc::Adc0Ch2, SigAdcDp);
channel_signal!(super::adc::Adc0Ch2, SigAdcDm);
channel_signal!(super::adc::Adc0Ch3, SigAdcDp);
channel_signal!(super::adc::Adc0Ch3, SigAdcDm);
channel_signal!(super::adc::Adc0Ch4, SigAdcSea);
channel_signal!(super::adc::Adc0Ch4, SigAdcSeb);
channel_signal!(super::adc::Adc0Ch5, SigAdcSea);
channel_signal!(super::adc::Adc0Ch5, SigAdcSeb);
channel_signal!(super::adc::Adc0Ch6, SigAdcSea);
channel_signal!(super::adc::Adc0Ch6, SigAdcSeb);
channel_signal!(super::adc::Adc0Ch7, SigAdcSea);
channel_signal!(super::adc::Adc0Ch7, SigAdcSeb);
channel_signal!(super::adc::Adc0Ch8, SigAdc);
channel_signal!(super::adc::Adc0Ch9, SigAdc);
channel_signal!(super::adc::Adc0Ch10, SigAdc);
channel_signal!(super::adc::Adc0Ch11, SigAdc);
channel_signal!(super::adc::Adc0Ch12, SigAdc);
channel_signal!(super::adc::Adc0Ch13, SigAdc);
channel_signal!(super::adc::Adc0Ch14, SigAdc);
channel_signal!(super::adc::Adc0Ch15, SigAdc);
channel_signal!(super::adc::Adc0Ch16, SigAdc);
channel_signal!(super::adc::Adc0Ch17, SigAdc);
channel_signal!(super::adc::Adc0Ch18, SigAdc);
channel_signal!(super::adc::Adc0Ch19, SigAdc);
channel_signal!(super::adc::Adc0Ch20, SigAdc);
channel_signal!(super::adc::Adc0Ch21, SigAdc);
channel_signal!(super::adc::Adc0Ch22, SigAdc);
channel_signal!(super::adc::Adc0Ch23, SigAdc);





































































































