::bobbin_mcu::signal_type!(FTM, SigFtm);
::bobbin_mcu::signal_type!(SPI_SCK, SigSpiSck);
::bobbin_mcu::signal_type!(SPI_SOUT, SigSpiSout);
::bobbin_mcu::signal_type!(SPI_SIN, SigSpiSin);
::bobbin_mcu::signal_type!(SPI_PCS0, SigSpiPcs0);
::bobbin_mcu::signal_type!(SPI_PCS1, SigSpiPcs1);
::bobbin_mcu::signal_type!(SPI_PCS2, SigSpiPcs2);
::bobbin_mcu::signal_type!(SPI_PCS3, SigSpiPcs3);
::bobbin_mcu::signal_type!(SPI_PCS4, SigSpiPcs4);
::bobbin_mcu::signal_type!(SPI_PCS5, SigSpiPcs5);
::bobbin_mcu::signal_type!(I2C_SCL, SigI2cScl);
::bobbin_mcu::signal_type!(I2C_SDA, SigI2cSda);
::bobbin_mcu::signal_type!(UART_TX, SigUartTx);
::bobbin_mcu::signal_type!(UART_RX, SigUartRx);
::bobbin_mcu::signal_type!(CAN_TX, SigCanTx);
::bobbin_mcu::signal_type!(CAN_RX, SigCanRx);
::bobbin_mcu::signal_type!(GPIO, SigGpio);
::bobbin_mcu::signal_type!(ADC_DP, SigAdcDp);
::bobbin_mcu::signal_type!(ADC_DM, SigAdcDm);
::bobbin_mcu::signal_type!(ADC_SEA, SigAdcSea);
::bobbin_mcu::signal_type!(ADC_SEB, SigAdcSeb);
::bobbin_mcu::signal_type!(ADC, SigAdc);

// CRC

// WDOG

// FTFE

// DMAMUX

// EDMA

// FTM
::bobbin_mcu::channel_signal!(super::ftm::Ftm0Ch0, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm0Ch1, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm0Ch2, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm0Ch3, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm0Ch4, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm0Ch5, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm0Ch6, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm0Ch7, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm1Ch0, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm1Ch1, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm1Ch2, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm1Ch3, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm1Ch4, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm1Ch5, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm1Ch6, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm1Ch7, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm2Ch0, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm2Ch1, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm2Ch2, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm2Ch3, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm2Ch4, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm2Ch5, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm2Ch6, SigFtm);
::bobbin_mcu::channel_signal!(super::ftm::Ftm2Ch7, SigFtm);

// PIT

// LPTMR

// SPI
::bobbin_mcu::periph_signal!(super::spi::Spi0, SigSpiSck);
::bobbin_mcu::periph_signal!(super::spi::Spi0, SigSpiSout);
::bobbin_mcu::periph_signal!(super::spi::Spi0, SigSpiSin);
::bobbin_mcu::periph_signal!(super::spi::Spi0, SigSpiPcs0);
::bobbin_mcu::periph_signal!(super::spi::Spi0, SigSpiPcs1);
::bobbin_mcu::periph_signal!(super::spi::Spi0, SigSpiPcs2);
::bobbin_mcu::periph_signal!(super::spi::Spi0, SigSpiPcs3);
::bobbin_mcu::periph_signal!(super::spi::Spi0, SigSpiPcs4);
::bobbin_mcu::periph_signal!(super::spi::Spi0, SigSpiPcs5);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiSck);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiSout);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiSin);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiPcs0);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiPcs1);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiPcs2);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiPcs3);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiSck);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiSout);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiSin);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiPcs0);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiPcs1);

// I2C
::bobbin_mcu::periph_signal!(super::i2c::I2c0, SigI2cScl);
::bobbin_mcu::periph_signal!(super::i2c::I2c0, SigI2cSda);
::bobbin_mcu::periph_signal!(super::i2c::I2c1, SigI2cScl);
::bobbin_mcu::periph_signal!(super::i2c::I2c1, SigI2cSda);

// UART
::bobbin_mcu::periph_signal!(super::uart::Uart0, SigUartTx);
::bobbin_mcu::periph_signal!(super::uart::Uart0, SigUartRx);
::bobbin_mcu::periph_signal!(super::uart::Uart1, SigUartTx);
::bobbin_mcu::periph_signal!(super::uart::Uart1, SigUartRx);
::bobbin_mcu::periph_signal!(super::uart::Uart2, SigUartTx);
::bobbin_mcu::periph_signal!(super::uart::Uart2, SigUartRx);
::bobbin_mcu::periph_signal!(super::uart::Uart3, SigUartTx);
::bobbin_mcu::periph_signal!(super::uart::Uart3, SigUartRx);
::bobbin_mcu::periph_signal!(super::uart::Uart4, SigUartTx);
::bobbin_mcu::periph_signal!(super::uart::Uart4, SigUartRx);
::bobbin_mcu::periph_signal!(super::uart::Uart5, SigUartTx);
::bobbin_mcu::periph_signal!(super::uart::Uart5, SigUartRx);

// USB

// FLEXCAN
::bobbin_mcu::periph_signal!(super::flexcan::Can0, SigCanTx);
::bobbin_mcu::periph_signal!(super::flexcan::Can0, SigCanRx);

// DAC

// GPIO
::bobbin_mcu::channel_signal!(super::gpio::Pa0, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa1, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa2, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa3, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa4, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa5, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa6, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa7, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa8, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa9, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa10, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa11, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa12, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa13, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa14, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa15, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa16, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa17, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa18, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa19, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa24, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa25, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa26, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa27, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa28, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pa29, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb0, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb1, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb2, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb3, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb4, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb5, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb6, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb7, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb8, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb9, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb10, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb11, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb12, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb13, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb16, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb17, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb18, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb19, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb20, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb21, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb22, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pb23, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc0, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc1, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc2, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc3, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc4, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc5, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc6, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc7, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc8, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc9, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc10, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc11, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc12, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc13, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc14, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc15, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc16, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc17, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc18, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pc19, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd0, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd1, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd2, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd3, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd4, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd5, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd6, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd7, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd8, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd9, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd10, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd11, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd12, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd13, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd14, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pd15, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe0, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe1, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe2, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe3, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe4, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe5, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe6, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe7, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe8, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe9, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe10, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe11, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe12, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe24, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe25, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe26, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe27, SigGpio);
::bobbin_mcu::channel_signal!(super::gpio::Pe28, SigGpio);

// PORT

// ADC
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch0, SigAdcDp);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch0, SigAdcDm);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch1, SigAdcDp);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch1, SigAdcDm);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch2, SigAdcDp);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch2, SigAdcDm);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch3, SigAdcDp);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch3, SigAdcDm);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch4, SigAdcSea);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch4, SigAdcSeb);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch5, SigAdcSea);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch5, SigAdcSeb);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch6, SigAdcSea);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch6, SigAdcSeb);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch7, SigAdcSea);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch7, SigAdcSeb);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch8, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch9, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch10, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch11, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch12, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch13, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch14, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch15, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch16, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch17, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch18, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch19, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch20, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch21, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch22, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc0Ch23, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch0, SigAdcDp);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch0, SigAdcDm);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch1, SigAdcDp);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch1, SigAdcDm);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch2, SigAdcDp);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch2, SigAdcDm);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch3, SigAdcDp);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch3, SigAdcDm);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch4, SigAdcSea);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch4, SigAdcSeb);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch5, SigAdcSea);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch5, SigAdcSeb);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch6, SigAdcSea);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch6, SigAdcSeb);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch7, SigAdcSea);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch7, SigAdcSeb);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch8, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch9, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch10, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch11, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch12, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch13, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch14, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch15, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch16, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch17, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch18, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch19, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch20, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch21, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch22, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch23, SigAdc);







































































































