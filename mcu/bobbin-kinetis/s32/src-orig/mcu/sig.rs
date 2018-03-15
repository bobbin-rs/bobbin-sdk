#[allow(unused_imports)] pub use ::bobbin_common::*;
signal_type!(FTM, SigFtm);
signal_type!(TX, SigTx);
signal_type!(RX, SigRx);
signal_type!(CTS, SigCts);
signal_type!(RTS, SigRts);
signal_type!(I2C_HREQ, SigI2cHreq);
signal_type!(I2C_SCL, SigI2cScl);
signal_type!(I2C_SDA, SigI2cSda);
signal_type!(I2C_SCLS, SigI2cScls);
signal_type!(I2C_SDAS, SigI2cSdas);
signal_type!(SPI_SCK, SigSpiSck);
signal_type!(SPI_SOUT, SigSpiSout);
signal_type!(SPI_SIN, SigSpiSin);
signal_type!(SPI_PCS0, SigSpiPcs0);
signal_type!(SPI_PCS1, SigSpiPcs1);
signal_type!(SPI_PCS2, SigSpiPcs2);
signal_type!(SPI_PCS3, SigSpiPcs3);
signal_type!(ADC, SigAdc);

// CRC

// DMAMUX

// EDMA

// FTM
channel_signal!(super::ftm::Ftm0Ch0, SigFtm);
channel_signal!(super::ftm::Ftm0Ch1, SigFtm);
channel_signal!(super::ftm::Ftm0Ch2, SigFtm);
channel_signal!(super::ftm::Ftm0Ch3, SigFtm);
channel_signal!(super::ftm::Ftm0Ch4, SigFtm);
channel_signal!(super::ftm::Ftm0Ch5, SigFtm);
channel_signal!(super::ftm::Ftm0Ch6, SigFtm);
channel_signal!(super::ftm::Ftm0Ch7, SigFtm);
channel_signal!(super::ftm::Ftm1Ch0, SigFtm);
channel_signal!(super::ftm::Ftm1Ch1, SigFtm);
channel_signal!(super::ftm::Ftm1Ch2, SigFtm);
channel_signal!(super::ftm::Ftm1Ch3, SigFtm);
channel_signal!(super::ftm::Ftm1Ch4, SigFtm);
channel_signal!(super::ftm::Ftm1Ch5, SigFtm);
channel_signal!(super::ftm::Ftm1Ch6, SigFtm);
channel_signal!(super::ftm::Ftm1Ch7, SigFtm);
channel_signal!(super::ftm::Ftm2Ch0, SigFtm);
channel_signal!(super::ftm::Ftm2Ch1, SigFtm);
channel_signal!(super::ftm::Ftm2Ch2, SigFtm);
channel_signal!(super::ftm::Ftm2Ch3, SigFtm);
channel_signal!(super::ftm::Ftm2Ch4, SigFtm);
channel_signal!(super::ftm::Ftm2Ch5, SigFtm);
channel_signal!(super::ftm::Ftm2Ch6, SigFtm);
channel_signal!(super::ftm::Ftm2Ch7, SigFtm);
channel_signal!(super::ftm::Ftm3Ch0, SigFtm);
channel_signal!(super::ftm::Ftm3Ch1, SigFtm);
channel_signal!(super::ftm::Ftm3Ch2, SigFtm);
channel_signal!(super::ftm::Ftm3Ch3, SigFtm);
channel_signal!(super::ftm::Ftm3Ch4, SigFtm);
channel_signal!(super::ftm::Ftm3Ch5, SigFtm);
channel_signal!(super::ftm::Ftm3Ch6, SigFtm);
channel_signal!(super::ftm::Ftm3Ch7, SigFtm);

// LPIT

// LPTMR

// FLEXCAN

// PORT

// GPIO

// LPUART
periph_signal!(super::lpuart::Lpuart0, SigTx);
periph_signal!(super::lpuart::Lpuart0, SigRx);
periph_signal!(super::lpuart::Lpuart0, SigCts);
periph_signal!(super::lpuart::Lpuart0, SigRts);
periph_signal!(super::lpuart::Lpuart1, SigTx);
periph_signal!(super::lpuart::Lpuart1, SigRx);
periph_signal!(super::lpuart::Lpuart1, SigCts);
periph_signal!(super::lpuart::Lpuart1, SigRts);
periph_signal!(super::lpuart::Lpuart2, SigTx);
periph_signal!(super::lpuart::Lpuart2, SigRx);
periph_signal!(super::lpuart::Lpuart2, SigCts);
periph_signal!(super::lpuart::Lpuart2, SigRts);

// LPI2C
periph_signal!(super::lpi2c::Lpi2c0, SigI2cHreq);
periph_signal!(super::lpi2c::Lpi2c0, SigI2cScl);
periph_signal!(super::lpi2c::Lpi2c0, SigI2cSda);
periph_signal!(super::lpi2c::Lpi2c0, SigI2cScls);
periph_signal!(super::lpi2c::Lpi2c0, SigI2cSdas);

// LPSPI
periph_signal!(super::lpspi::Lpspi0, SigSpiSck);
periph_signal!(super::lpspi::Lpspi0, SigSpiSout);
periph_signal!(super::lpspi::Lpspi0, SigSpiSin);
periph_signal!(super::lpspi::Lpspi0, SigSpiPcs0);
periph_signal!(super::lpspi::Lpspi0, SigSpiPcs1);
periph_signal!(super::lpspi::Lpspi0, SigSpiPcs2);
periph_signal!(super::lpspi::Lpspi0, SigSpiPcs3);
periph_signal!(super::lpspi::Lpspi1, SigSpiSck);
periph_signal!(super::lpspi::Lpspi1, SigSpiSout);
periph_signal!(super::lpspi::Lpspi1, SigSpiSin);
periph_signal!(super::lpspi::Lpspi1, SigSpiPcs0);
periph_signal!(super::lpspi::Lpspi1, SigSpiPcs1);
periph_signal!(super::lpspi::Lpspi1, SigSpiPcs2);
periph_signal!(super::lpspi::Lpspi1, SigSpiPcs3);
periph_signal!(super::lpspi::Lpspi2, SigSpiSck);
periph_signal!(super::lpspi::Lpspi2, SigSpiSout);
periph_signal!(super::lpspi::Lpspi2, SigSpiSin);
periph_signal!(super::lpspi::Lpspi2, SigSpiPcs0);
periph_signal!(super::lpspi::Lpspi2, SigSpiPcs1);
periph_signal!(super::lpspi::Lpspi2, SigSpiPcs2);
periph_signal!(super::lpspi::Lpspi2, SigSpiPcs3);

// ADC
channel_signal!(super::adc::Adc0Ch0, SigAdc);
channel_signal!(super::adc::Adc0Ch1, SigAdc);
channel_signal!(super::adc::Adc0Ch2, SigAdc);
channel_signal!(super::adc::Adc0Ch3, SigAdc);
channel_signal!(super::adc::Adc0Ch4, SigAdc);
channel_signal!(super::adc::Adc0Ch5, SigAdc);
channel_signal!(super::adc::Adc0Ch6, SigAdc);
channel_signal!(super::adc::Adc0Ch7, SigAdc);
channel_signal!(super::adc::Adc0Ch8, SigAdc);
channel_signal!(super::adc::Adc0Ch9, SigAdc);
channel_signal!(super::adc::Adc0Ch10, SigAdc);
channel_signal!(super::adc::Adc0Ch11, SigAdc);
channel_signal!(super::adc::Adc0Ch12, SigAdc);
channel_signal!(super::adc::Adc0Ch13, SigAdc);
channel_signal!(super::adc::Adc0Ch14, SigAdc);
channel_signal!(super::adc::Adc0Ch15, SigAdc);
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



















































































































































































