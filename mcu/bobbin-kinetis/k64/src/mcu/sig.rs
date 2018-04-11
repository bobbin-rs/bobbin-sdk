#[allow(unused_imports)] pub use ::bobbin_common::*;

signal_type!(FTM, SigFtm);
signal_type!(SPI_SCK, SigSpiSck);
signal_type!(SPI_SOUT, SigSpiSout);
signal_type!(SPI_SIN, SigSpiSin);
signal_type!(SPI_PCS0, SigSpiPcs0);
signal_type!(SPI_PCS1, SigSpiPcs1);
signal_type!(SPI_PCS2, SigSpiPcs2);
signal_type!(SPI_PCS3, SigSpiPcs3);
signal_type!(SPI_PCS4, SigSpiPcs4);
signal_type!(SPI_PCS5, SigSpiPcs5);
signal_type!(I2C_SCL, SigI2cScl);
signal_type!(I2C_SDA, SigI2cSda);
signal_type!(UART_TX, SigUartTx);
signal_type!(UART_RX, SigUartRx);
signal_type!(CAN_TX, SigCanTx);
signal_type!(CAN_RX, SigCanRx);
signal_type!(GPIO, SigGpio);
signal_type!(ADC_DP, SigAdcDp);
signal_type!(ADC_DM, SigAdcDm);
signal_type!(ADC_SEA, SigAdcSea);
signal_type!(ADC_SEB, SigAdcSeb);
signal_type!(ADC, SigAdc);

// CRC

// WDOG

// FTFE

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

// PIT

// LPTMR

// SPI
periph_signal!(super::spi::Spi0, SigSpiSck);
periph_signal!(super::spi::Spi0, SigSpiSout);
periph_signal!(super::spi::Spi0, SigSpiSin);
periph_signal!(super::spi::Spi0, SigSpiPcs0);
periph_signal!(super::spi::Spi0, SigSpiPcs1);
periph_signal!(super::spi::Spi0, SigSpiPcs2);
periph_signal!(super::spi::Spi0, SigSpiPcs3);
periph_signal!(super::spi::Spi0, SigSpiPcs4);
periph_signal!(super::spi::Spi0, SigSpiPcs5);
periph_signal!(super::spi::Spi1, SigSpiSck);
periph_signal!(super::spi::Spi1, SigSpiSout);
periph_signal!(super::spi::Spi1, SigSpiSin);
periph_signal!(super::spi::Spi1, SigSpiPcs0);
periph_signal!(super::spi::Spi1, SigSpiPcs1);
periph_signal!(super::spi::Spi1, SigSpiPcs2);
periph_signal!(super::spi::Spi1, SigSpiPcs3);
periph_signal!(super::spi::Spi2, SigSpiSck);
periph_signal!(super::spi::Spi2, SigSpiSout);
periph_signal!(super::spi::Spi2, SigSpiSin);
periph_signal!(super::spi::Spi2, SigSpiPcs0);
periph_signal!(super::spi::Spi2, SigSpiPcs1);

// I2C
periph_signal!(super::i2c::I2c0, SigI2cScl);
periph_signal!(super::i2c::I2c0, SigI2cSda);
periph_signal!(super::i2c::I2c1, SigI2cScl);
periph_signal!(super::i2c::I2c1, SigI2cSda);

// UART
periph_signal!(super::uart::Uart0, SigUartTx);
periph_signal!(super::uart::Uart0, SigUartRx);
periph_signal!(super::uart::Uart1, SigUartTx);
periph_signal!(super::uart::Uart1, SigUartRx);
periph_signal!(super::uart::Uart2, SigUartTx);
periph_signal!(super::uart::Uart2, SigUartRx);
periph_signal!(super::uart::Uart3, SigUartTx);
periph_signal!(super::uart::Uart3, SigUartRx);
periph_signal!(super::uart::Uart4, SigUartTx);
periph_signal!(super::uart::Uart4, SigUartRx);
periph_signal!(super::uart::Uart5, SigUartTx);
periph_signal!(super::uart::Uart5, SigUartRx);

// USB

// FLEXCAN
periph_signal!(super::flexcan::Can0, SigCanTx);
periph_signal!(super::flexcan::Can0, SigCanRx);

// DAC

// GPIO
channel_signal!(super::gpio::Pa0, SigGpio);
channel_signal!(super::gpio::Pa1, SigGpio);
channel_signal!(super::gpio::Pa2, SigGpio);
channel_signal!(super::gpio::Pa3, SigGpio);
channel_signal!(super::gpio::Pa4, SigGpio);
channel_signal!(super::gpio::Pa5, SigGpio);
channel_signal!(super::gpio::Pa6, SigGpio);
channel_signal!(super::gpio::Pa7, SigGpio);
channel_signal!(super::gpio::Pa8, SigGpio);
channel_signal!(super::gpio::Pa9, SigGpio);
channel_signal!(super::gpio::Pa10, SigGpio);
channel_signal!(super::gpio::Pa11, SigGpio);
channel_signal!(super::gpio::Pa12, SigGpio);
channel_signal!(super::gpio::Pa13, SigGpio);
channel_signal!(super::gpio::Pa14, SigGpio);
channel_signal!(super::gpio::Pa15, SigGpio);
channel_signal!(super::gpio::Pa16, SigGpio);
channel_signal!(super::gpio::Pa17, SigGpio);
channel_signal!(super::gpio::Pa18, SigGpio);
channel_signal!(super::gpio::Pa19, SigGpio);
channel_signal!(super::gpio::Pa24, SigGpio);
channel_signal!(super::gpio::Pa25, SigGpio);
channel_signal!(super::gpio::Pa26, SigGpio);
channel_signal!(super::gpio::Pa27, SigGpio);
channel_signal!(super::gpio::Pa28, SigGpio);
channel_signal!(super::gpio::Pa29, SigGpio);
channel_signal!(super::gpio::Pb0, SigGpio);
channel_signal!(super::gpio::Pb1, SigGpio);
channel_signal!(super::gpio::Pb2, SigGpio);
channel_signal!(super::gpio::Pb3, SigGpio);
channel_signal!(super::gpio::Pb4, SigGpio);
channel_signal!(super::gpio::Pb5, SigGpio);
channel_signal!(super::gpio::Pb6, SigGpio);
channel_signal!(super::gpio::Pb7, SigGpio);
channel_signal!(super::gpio::Pb8, SigGpio);
channel_signal!(super::gpio::Pb9, SigGpio);
channel_signal!(super::gpio::Pb10, SigGpio);
channel_signal!(super::gpio::Pb11, SigGpio);
channel_signal!(super::gpio::Pb12, SigGpio);
channel_signal!(super::gpio::Pb13, SigGpio);
channel_signal!(super::gpio::Pb16, SigGpio);
channel_signal!(super::gpio::Pb17, SigGpio);
channel_signal!(super::gpio::Pb18, SigGpio);
channel_signal!(super::gpio::Pb19, SigGpio);
channel_signal!(super::gpio::Pb20, SigGpio);
channel_signal!(super::gpio::Pb21, SigGpio);
channel_signal!(super::gpio::Pb22, SigGpio);
channel_signal!(super::gpio::Pb23, SigGpio);
channel_signal!(super::gpio::Pc0, SigGpio);
channel_signal!(super::gpio::Pc1, SigGpio);
channel_signal!(super::gpio::Pc2, SigGpio);
channel_signal!(super::gpio::Pc3, SigGpio);
channel_signal!(super::gpio::Pc4, SigGpio);
channel_signal!(super::gpio::Pc5, SigGpio);
channel_signal!(super::gpio::Pc6, SigGpio);
channel_signal!(super::gpio::Pc7, SigGpio);
channel_signal!(super::gpio::Pc8, SigGpio);
channel_signal!(super::gpio::Pc9, SigGpio);
channel_signal!(super::gpio::Pc10, SigGpio);
channel_signal!(super::gpio::Pc11, SigGpio);
channel_signal!(super::gpio::Pc12, SigGpio);
channel_signal!(super::gpio::Pc13, SigGpio);
channel_signal!(super::gpio::Pc14, SigGpio);
channel_signal!(super::gpio::Pc15, SigGpio);
channel_signal!(super::gpio::Pc16, SigGpio);
channel_signal!(super::gpio::Pc17, SigGpio);
channel_signal!(super::gpio::Pc18, SigGpio);
channel_signal!(super::gpio::Pc19, SigGpio);
channel_signal!(super::gpio::Pd0, SigGpio);
channel_signal!(super::gpio::Pd1, SigGpio);
channel_signal!(super::gpio::Pd2, SigGpio);
channel_signal!(super::gpio::Pd3, SigGpio);
channel_signal!(super::gpio::Pd4, SigGpio);
channel_signal!(super::gpio::Pd5, SigGpio);
channel_signal!(super::gpio::Pd6, SigGpio);
channel_signal!(super::gpio::Pd7, SigGpio);
channel_signal!(super::gpio::Pd8, SigGpio);
channel_signal!(super::gpio::Pd9, SigGpio);
channel_signal!(super::gpio::Pd10, SigGpio);
channel_signal!(super::gpio::Pd11, SigGpio);
channel_signal!(super::gpio::Pd12, SigGpio);
channel_signal!(super::gpio::Pd13, SigGpio);
channel_signal!(super::gpio::Pd14, SigGpio);
channel_signal!(super::gpio::Pd15, SigGpio);
channel_signal!(super::gpio::Pe0, SigGpio);
channel_signal!(super::gpio::Pe1, SigGpio);
channel_signal!(super::gpio::Pe2, SigGpio);
channel_signal!(super::gpio::Pe3, SigGpio);
channel_signal!(super::gpio::Pe4, SigGpio);
channel_signal!(super::gpio::Pe5, SigGpio);
channel_signal!(super::gpio::Pe6, SigGpio);
channel_signal!(super::gpio::Pe7, SigGpio);
channel_signal!(super::gpio::Pe8, SigGpio);
channel_signal!(super::gpio::Pe9, SigGpio);
channel_signal!(super::gpio::Pe10, SigGpio);
channel_signal!(super::gpio::Pe11, SigGpio);
channel_signal!(super::gpio::Pe12, SigGpio);
channel_signal!(super::gpio::Pe24, SigGpio);
channel_signal!(super::gpio::Pe25, SigGpio);
channel_signal!(super::gpio::Pe26, SigGpio);
channel_signal!(super::gpio::Pe27, SigGpio);
channel_signal!(super::gpio::Pe28, SigGpio);

// PORT

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
channel_signal!(super::adc::Adc1Ch0, SigAdcDp);
channel_signal!(super::adc::Adc1Ch0, SigAdcDm);
channel_signal!(super::adc::Adc1Ch1, SigAdcDp);
channel_signal!(super::adc::Adc1Ch1, SigAdcDm);
channel_signal!(super::adc::Adc1Ch2, SigAdcDp);
channel_signal!(super::adc::Adc1Ch2, SigAdcDm);
channel_signal!(super::adc::Adc1Ch3, SigAdcDp);
channel_signal!(super::adc::Adc1Ch3, SigAdcDm);
channel_signal!(super::adc::Adc1Ch4, SigAdcSea);
channel_signal!(super::adc::Adc1Ch4, SigAdcSeb);
channel_signal!(super::adc::Adc1Ch5, SigAdcSea);
channel_signal!(super::adc::Adc1Ch5, SigAdcSeb);
channel_signal!(super::adc::Adc1Ch6, SigAdcSea);
channel_signal!(super::adc::Adc1Ch6, SigAdcSeb);
channel_signal!(super::adc::Adc1Ch7, SigAdcSea);
channel_signal!(super::adc::Adc1Ch7, SigAdcSeb);
channel_signal!(super::adc::Adc1Ch8, SigAdc);
channel_signal!(super::adc::Adc1Ch9, SigAdc);
channel_signal!(super::adc::Adc1Ch10, SigAdc);
channel_signal!(super::adc::Adc1Ch11, SigAdc);
channel_signal!(super::adc::Adc1Ch12, SigAdc);
channel_signal!(super::adc::Adc1Ch13, SigAdc);
channel_signal!(super::adc::Adc1Ch14, SigAdc);
channel_signal!(super::adc::Adc1Ch15, SigAdc);
channel_signal!(super::adc::Adc1Ch16, SigAdc);
channel_signal!(super::adc::Adc1Ch17, SigAdc);
channel_signal!(super::adc::Adc1Ch18, SigAdc);
channel_signal!(super::adc::Adc1Ch19, SigAdc);
channel_signal!(super::adc::Adc1Ch20, SigAdc);
channel_signal!(super::adc::Adc1Ch21, SigAdc);
channel_signal!(super::adc::Adc1Ch22, SigAdc);
channel_signal!(super::adc::Adc1Ch23, SigAdc);







































































































