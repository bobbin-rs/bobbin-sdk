#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::gpio::*;

periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x40020000);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x40020400);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x40020800);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x40020c00);
periph!( GPIOE, Gpioe, _GPIOE, GpioPeriph, 0x40021000);
periph!( GPIOF, Gpiof, _GPIOF, GpioPeriph, 0x40021400);
periph!( GPIOG, Gpiog, _GPIOG, GpioPeriph, 0x40021800);
periph!( GPIOH, Gpioh, _GPIOH, GpioPeriph, 0x40021c00);
periph!( GPIOI, Gpioi, _GPIOI, GpioPeriph, 0x40022000);
periph!( GPIOJ, Gpioj, _GPIOJ, GpioPeriph, 0x40022400);
periph!( GPIOK, Gpiok, _GPIOK, GpioPeriph, 0x40022800);













pin!(PA0, Pa0, GPIOA, Gpioa, _PA0, GpioPin, _GPIOA, 0);
   alt_fn!(Pa0, super::sig::Adc1In0, 0);
   alt_fn!(Pa0, super::sig::Adc2In0, 0);
   alt_fn!(Pa0, super::sig::Adc3In0, 0);
   alt_fn!(Pa0, super::sig::Tim2Ch1, 1);
   alt_fn!(Pa0, super::sig::Tim2Etr, 1);
   alt_fn!(Pa0, super::sig::Tim5Ch1, 2);
   alt_fn!(Pa0, super::sig::Tim8Etr, 3);
   alt_fn!(Pa0, super::sig::Usart2Cts, 7);
   alt_fn!(Pa0, super::sig::Uart4Tx, 8);
   alt_fn!(Pa0, super::sig::EthMiiCrs, 11);
   alt_fn!(Pa0, super::sig::Eventout, 15);

pin!(PA1, Pa1, GPIOA, Gpioa, _PA1, GpioPin, _GPIOA, 1);
   alt_fn!(Pa1, super::sig::Adc1In1, 0);
   alt_fn!(Pa1, super::sig::Adc2In1, 0);
   alt_fn!(Pa1, super::sig::Adc3In1, 0);
   alt_fn!(Pa1, super::sig::Tim2Ch2, 1);
   alt_fn!(Pa1, super::sig::Tim5Ch2, 2);
   alt_fn!(Pa1, super::sig::Usart2Rts, 7);
   alt_fn!(Pa1, super::sig::Uart4Rx, 8);
   alt_fn!(Pa1, super::sig::EthMiiRxClk, 11);
   alt_fn!(Pa1, super::sig::EthRmiiRefClk, 11);
   alt_fn!(Pa1, super::sig::Eventout, 15);

pin!(PA2, Pa2, GPIOA, Gpioa, _PA2, GpioPin, _GPIOA, 2);
   alt_fn!(Pa2, super::sig::Adc1In2, 0);
   alt_fn!(Pa2, super::sig::Adc2In2, 0);
   alt_fn!(Pa2, super::sig::Adc3In2, 0);
   alt_fn!(Pa2, super::sig::Tim2Ch3, 1);
   alt_fn!(Pa2, super::sig::Tim5Ch3, 2);
   alt_fn!(Pa2, super::sig::Tim9Ch1, 3);
   alt_fn!(Pa2, super::sig::Usart2Tx, 7);
   alt_fn!(Pa2, super::sig::EthMdio, 11);
   alt_fn!(Pa2, super::sig::Eventout, 15);

pin!(PA3, Pa3, GPIOA, Gpioa, _PA3, GpioPin, _GPIOA, 3);
   alt_fn!(Pa3, super::sig::Adc1In3, 0);
   alt_fn!(Pa3, super::sig::Adc2In3, 0);
   alt_fn!(Pa3, super::sig::Adc3In3, 0);
   alt_fn!(Pa3, super::sig::Tim2Ch4, 1);
   alt_fn!(Pa3, super::sig::Tim5Ch4, 2);
   alt_fn!(Pa3, super::sig::Tim9Ch2, 3);
   alt_fn!(Pa3, super::sig::Usart2Rx, 7);
   alt_fn!(Pa3, super::sig::OtgHsUlpiD0, 10);
   alt_fn!(Pa3, super::sig::EthMiiCol, 11);
   alt_fn!(Pa3, super::sig::LcdB5, 14);
   alt_fn!(Pa3, super::sig::Eventout, 15);

pin!(PA4, Pa4, GPIOA, Gpioa, _PA4, GpioPin, _GPIOA, 4);
   alt_fn!(Pa4, super::sig::Adc1In4, 0);
   alt_fn!(Pa4, super::sig::Adc2In4, 0);
   alt_fn!(Pa4, super::sig::Spi1Nss, 5);
   alt_fn!(Pa4, super::sig::Spi3Nss, 6);
   alt_fn!(Pa4, super::sig::I2s3Ws, 6);
   alt_fn!(Pa4, super::sig::Usart2Ck, 7);
   alt_fn!(Pa4, super::sig::OtgHsSof, 12);
   alt_fn!(Pa4, super::sig::DcmiHsync, 13);
   alt_fn!(Pa4, super::sig::LcdVsync, 14);
   alt_fn!(Pa4, super::sig::Eventout, 15);

pin!(PA5, Pa5, GPIOA, Gpioa, _PA5, GpioPin, _GPIOA, 5);
   alt_fn!(Pa5, super::sig::Adc1In5, 0);
   alt_fn!(Pa5, super::sig::Adc2In5, 0);
   alt_fn!(Pa5, super::sig::Tim2Ch1, 1);
   alt_fn!(Pa5, super::sig::Tim2Etr, 1);
   alt_fn!(Pa5, super::sig::Tim8Ch1n, 3);
   alt_fn!(Pa5, super::sig::Spi1Sck, 5);
   alt_fn!(Pa5, super::sig::OtgHsUlpiCk, 10);
   alt_fn!(Pa5, super::sig::Eventout, 15);

pin!(PA6, Pa6, GPIOA, Gpioa, _PA6, GpioPin, _GPIOA, 6);
   alt_fn!(Pa6, super::sig::Adc1In6, 0);
   alt_fn!(Pa6, super::sig::Adc2In6, 0);
   alt_fn!(Pa6, super::sig::Tim1Bkin, 1);
   alt_fn!(Pa6, super::sig::Tim3Ch1, 2);
   alt_fn!(Pa6, super::sig::Tim8Bkin, 3);
   alt_fn!(Pa6, super::sig::Spi1Miso, 5);
   alt_fn!(Pa6, super::sig::Tim13Ch1, 9);
   alt_fn!(Pa6, super::sig::DcmiPxclk, 13);
   alt_fn!(Pa6, super::sig::LcdG2, 14);
   alt_fn!(Pa6, super::sig::Eventout, 15);

pin!(PA7, Pa7, GPIOA, Gpioa, _PA7, GpioPin, _GPIOA, 7);
   alt_fn!(Pa7, super::sig::Adc1In7, 0);
   alt_fn!(Pa7, super::sig::Adc2In7, 0);
   alt_fn!(Pa7, super::sig::Tim1Ch1n, 1);
   alt_fn!(Pa7, super::sig::Tim3Ch2, 2);
   alt_fn!(Pa7, super::sig::Tim8Ch1n, 3);
   alt_fn!(Pa7, super::sig::Spi1Mosi, 5);
   alt_fn!(Pa7, super::sig::Tim14Ch1, 9);
   alt_fn!(Pa7, super::sig::EthMiiRxDv, 11);
   alt_fn!(Pa7, super::sig::EthRmiiCrsDv, 11);
   alt_fn!(Pa7, super::sig::Eventout, 15);

pin!(PA8, Pa8, GPIOA, Gpioa, _PA8, GpioPin, _GPIOA, 8);
   alt_fn!(Pa8, super::sig::Mco1, 0);
   alt_fn!(Pa8, super::sig::Tim1Ch1, 1);
   alt_fn!(Pa8, super::sig::I2c3Scl, 4);
   alt_fn!(Pa8, super::sig::Usart1Ck, 7);
   alt_fn!(Pa8, super::sig::OtgFsSof, 10);
   alt_fn!(Pa8, super::sig::LcdR6, 14);
   alt_fn!(Pa8, super::sig::Eventout, 15);

pin!(PA9, Pa9, GPIOA, Gpioa, _PA9, GpioPin, _GPIOA, 9);
   alt_fn!(Pa9, super::sig::Tim1Ch2, 1);
   alt_fn!(Pa9, super::sig::I2c3Smba, 4);
   alt_fn!(Pa9, super::sig::Usart1Tx, 7);
   alt_fn!(Pa9, super::sig::DcmiD0, 13);
   alt_fn!(Pa9, super::sig::Eventout, 15);

pin!(PA10, Pa10, GPIOA, Gpioa, _PA10, GpioPin, _GPIOA, 10);
   alt_fn!(Pa10, super::sig::Tim1Ch3, 1);
   alt_fn!(Pa10, super::sig::Usart1Rx, 7);
   alt_fn!(Pa10, super::sig::OtgFsId, 10);
   alt_fn!(Pa10, super::sig::DcmiD1, 13);
   alt_fn!(Pa10, super::sig::Eventout, 15);

pin!(PA11, Pa11, GPIOA, Gpioa, _PA11, GpioPin, _GPIOA, 11);
   alt_fn!(Pa11, super::sig::Tim1Ch4, 1);
   alt_fn!(Pa11, super::sig::Usart1Cts, 7);
   alt_fn!(Pa11, super::sig::Can1Rx, 9);
   alt_fn!(Pa11, super::sig::OtgFsDm, 10);
   alt_fn!(Pa11, super::sig::LcdR4, 14);
   alt_fn!(Pa11, super::sig::Eventout, 15);

pin!(PA12, Pa12, GPIOA, Gpioa, _PA12, GpioPin, _GPIOA, 12);
   alt_fn!(Pa12, super::sig::Tim1Etr, 1);
   alt_fn!(Pa12, super::sig::Usart1Rts, 7);
   alt_fn!(Pa12, super::sig::Can1Tx, 9);
   alt_fn!(Pa12, super::sig::OtgFsDp, 10);
   alt_fn!(Pa12, super::sig::LcdR5, 14);
   alt_fn!(Pa12, super::sig::Eventout, 15);

pin!(PA13, Pa13, GPIOA, Gpioa, _PA13, GpioPin, _GPIOA, 13);
   alt_fn!(Pa13, super::sig::Jtms, 0);
   alt_fn!(Pa13, super::sig::Swdio, 0);
   alt_fn!(Pa13, super::sig::Eventout, 15);

pin!(PA14, Pa14, GPIOA, Gpioa, _PA14, GpioPin, _GPIOA, 14);
   alt_fn!(Pa14, super::sig::Jtck, 0);
   alt_fn!(Pa14, super::sig::Swclk, 0);
   alt_fn!(Pa14, super::sig::Eventout, 15);

pin!(PA15, Pa15, GPIOA, Gpioa, _PA15, GpioPin, _GPIOA, 15);
   alt_fn!(Pa15, super::sig::Jtdi, 0);
   alt_fn!(Pa15, super::sig::Tim2Ch1, 1);
   alt_fn!(Pa15, super::sig::Tim2Etr, 1);
   alt_fn!(Pa15, super::sig::Spi1Nss, 5);
   alt_fn!(Pa15, super::sig::Spi3Nss, 6);
   alt_fn!(Pa15, super::sig::I2s3Ws, 6);
   alt_fn!(Pa15, super::sig::Eventout, 15);

pin!(PB0, Pb0, GPIOB, Gpiob, _PB0, GpioPin, _GPIOB, 0);
   alt_fn!(Pb0, super::sig::Adc1In8, 0);
   alt_fn!(Pb0, super::sig::Adc2In8, 0);
   alt_fn!(Pb0, super::sig::Tim1Ch2n, 1);
   alt_fn!(Pb0, super::sig::Tim3Ch3, 2);
   alt_fn!(Pb0, super::sig::Tim8Ch2n, 3);
   alt_fn!(Pb0, super::sig::LcdR3, 9);
   alt_fn!(Pb0, super::sig::OtgHsUlpiD1, 10);
   alt_fn!(Pb0, super::sig::EthMiiRxd2, 11);
   alt_fn!(Pb0, super::sig::Eventout, 15);

pin!(PB1, Pb1, GPIOB, Gpiob, _PB1, GpioPin, _GPIOB, 1);
   alt_fn!(Pb1, super::sig::Adc1In9, 0);
   alt_fn!(Pb1, super::sig::Adc2In9, 0);
   alt_fn!(Pb1, super::sig::Tim1Ch3n, 1);
   alt_fn!(Pb1, super::sig::Tim3Ch4, 2);
   alt_fn!(Pb1, super::sig::Tim8Ch3n, 3);
   alt_fn!(Pb1, super::sig::LcdR6, 9);
   alt_fn!(Pb1, super::sig::OtgHsUlpiD2, 10);
   alt_fn!(Pb1, super::sig::EthMiiRxd3, 11);
   alt_fn!(Pb1, super::sig::Eventout, 15);

pin!(PB2, Pb2, GPIOB, Gpiob, _PB2, GpioPin, _GPIOB, 2);
   alt_fn!(Pb2, super::sig::Eventout, 15);

pin!(PB3, Pb3, GPIOB, Gpiob, _PB3, GpioPin, _GPIOB, 3);
   alt_fn!(Pb3, super::sig::Jtdo, 0);
   alt_fn!(Pb3, super::sig::Traceswo, 0);
   alt_fn!(Pb3, super::sig::Tim2Ch2, 1);
   alt_fn!(Pb3, super::sig::Spi1Sck, 5);
   alt_fn!(Pb3, super::sig::Spi3Sck, 6);
   alt_fn!(Pb3, super::sig::Is2cCk, 6);
   alt_fn!(Pb3, super::sig::Eventout, 15);

pin!(PB4, Pb4, GPIOB, Gpiob, _PB4, GpioPin, _GPIOB, 4);
   alt_fn!(Pb4, super::sig::Njtrst, 0);
   alt_fn!(Pb4, super::sig::Tim3Ch1, 2);
   alt_fn!(Pb4, super::sig::Spi1Miso, 5);
   alt_fn!(Pb4, super::sig::Spi3Miso, 6);
   alt_fn!(Pb4, super::sig::I2s3extSd, 7);
   alt_fn!(Pb4, super::sig::Eventout, 15);

pin!(PB5, Pb5, GPIOB, Gpiob, _PB5, GpioPin, _GPIOB, 5);
   alt_fn!(Pb5, super::sig::Tim3Ch2, 2);
   alt_fn!(Pb5, super::sig::I2c1Smba, 4);
   alt_fn!(Pb5, super::sig::Spi1Mosi, 5);
   alt_fn!(Pb5, super::sig::Spi3Mosi, 6);
   alt_fn!(Pb5, super::sig::I2s3Sd, 6);
   alt_fn!(Pb5, super::sig::Can2Rx, 9);
   alt_fn!(Pb5, super::sig::OtgHsUlpiD7, 10);
   alt_fn!(Pb5, super::sig::EthPpsOut, 11);
   alt_fn!(Pb5, super::sig::FmcSdcke1, 12);
   alt_fn!(Pb5, super::sig::DcmiD10, 13);
   alt_fn!(Pb5, super::sig::Eventout, 15);

pin!(PB6, Pb6, GPIOB, Gpiob, _PB6, GpioPin, _GPIOB, 6);
   alt_fn!(Pb6, super::sig::Tim4Ch1, 2);
   alt_fn!(Pb6, super::sig::I2c1Scl, 4);
   alt_fn!(Pb6, super::sig::Usart1Tx, 7);
   alt_fn!(Pb6, super::sig::Can2Tx, 9);
   alt_fn!(Pb6, super::sig::FmcSdne1, 12);
   alt_fn!(Pb6, super::sig::DcmiD5, 13);
   alt_fn!(Pb6, super::sig::Eventout, 15);

pin!(PB7, Pb7, GPIOB, Gpiob, _PB7, GpioPin, _GPIOB, 7);
   alt_fn!(Pb7, super::sig::Tim4Ch2, 2);
   alt_fn!(Pb7, super::sig::I2c1Sda, 4);
   alt_fn!(Pb7, super::sig::Usart1Rx, 7);
   alt_fn!(Pb7, super::sig::FmcNl, 12);
   alt_fn!(Pb7, super::sig::DcmiVsync, 13);
   alt_fn!(Pb7, super::sig::Eventout, 15);

pin!(PB8, Pb8, GPIOB, Gpiob, _PB8, GpioPin, _GPIOB, 8);
   alt_fn!(Pb8, super::sig::Tim4Ch3, 2);
   alt_fn!(Pb8, super::sig::Tim10Ch1, 3);
   alt_fn!(Pb8, super::sig::I2c1Scl, 4);
   alt_fn!(Pb8, super::sig::Can1Rx, 9);
   alt_fn!(Pb8, super::sig::EthMiiTxd3, 11);
   alt_fn!(Pb8, super::sig::SdioD4, 12);
   alt_fn!(Pb8, super::sig::DcmiD6, 13);
   alt_fn!(Pb8, super::sig::LcdB6, 14);
   alt_fn!(Pb8, super::sig::Eventout, 15);

pin!(PB9, Pb9, GPIOB, Gpiob, _PB9, GpioPin, _GPIOB, 9);
   alt_fn!(Pb9, super::sig::Tim4Ch4, 2);
   alt_fn!(Pb9, super::sig::Tim11Ch1, 3);
   alt_fn!(Pb9, super::sig::I2c1Sda, 4);
   alt_fn!(Pb9, super::sig::Spi2Nss, 5);
   alt_fn!(Pb9, super::sig::I2s2Ws, 5);
   alt_fn!(Pb9, super::sig::Can1Tx, 9);
   alt_fn!(Pb9, super::sig::SdioD5, 12);
   alt_fn!(Pb9, super::sig::DcmiD7, 13);
   alt_fn!(Pb9, super::sig::LcdB7, 14);
   alt_fn!(Pb9, super::sig::Eventout, 15);

pin!(PB10, Pb10, GPIOB, Gpiob, _PB10, GpioPin, _GPIOB, 10);
   alt_fn!(Pb10, super::sig::Tim2Ch3, 1);
   alt_fn!(Pb10, super::sig::I2c2Scl, 4);
   alt_fn!(Pb10, super::sig::Spi2Sck, 5);
   alt_fn!(Pb10, super::sig::I2s2Ck, 5);
   alt_fn!(Pb10, super::sig::Usart3Tx, 7);
   alt_fn!(Pb10, super::sig::OtgHsUlpiD3, 10);
   alt_fn!(Pb10, super::sig::EthMiiRxEr, 11);
   alt_fn!(Pb10, super::sig::LcdG4, 14);
   alt_fn!(Pb10, super::sig::Eventout, 15);

pin!(PB11, Pb11, GPIOB, Gpiob, _PB11, GpioPin, _GPIOB, 11);
   alt_fn!(Pb11, super::sig::Sim2Ch4, 1);
   alt_fn!(Pb11, super::sig::I2c2Sda, 4);
   alt_fn!(Pb11, super::sig::Usart3Rx, 7);
   alt_fn!(Pb11, super::sig::OtgHsUlpiD4, 10);
   alt_fn!(Pb11, super::sig::EthMiiTxEn, 11);
   alt_fn!(Pb11, super::sig::EthRmiiTxEn, 11);
   alt_fn!(Pb11, super::sig::LcdG5, 13);
   alt_fn!(Pb11, super::sig::Eventout, 15);

pin!(PB12, Pb12, GPIOB, Gpiob, _PB12, GpioPin, _GPIOB, 12);
   alt_fn!(Pb12, super::sig::Tim1Bkin, 1);
   alt_fn!(Pb12, super::sig::I2c2Smba, 4);
   alt_fn!(Pb12, super::sig::Spi2Nss, 5);
   alt_fn!(Pb12, super::sig::I2s2Ws, 5);
   alt_fn!(Pb12, super::sig::Usart3Ck, 7);
   alt_fn!(Pb12, super::sig::Can2Rx, 9);
   alt_fn!(Pb12, super::sig::OtgHsUlpiD5, 10);
   alt_fn!(Pb12, super::sig::EthMiiTxd0, 11);
   alt_fn!(Pb12, super::sig::EthRmiiTxd0, 11);
   alt_fn!(Pb12, super::sig::OtgHsId, 12);
   alt_fn!(Pb12, super::sig::Eventout, 15);

pin!(PB13, Pb13, GPIOB, Gpiob, _PB13, GpioPin, _GPIOB, 13);
   alt_fn!(Pb13, super::sig::Tim1Ch1n, 1);
   alt_fn!(Pb13, super::sig::Spi2Sck, 5);
   alt_fn!(Pb13, super::sig::I2s2Ck, 5);
   alt_fn!(Pb13, super::sig::Usart3Cts, 7);
   alt_fn!(Pb13, super::sig::Can2Tx, 9);
   alt_fn!(Pb13, super::sig::OtgHsUlpiD6, 10);
   alt_fn!(Pb13, super::sig::EthMiiTxd1, 11);
   alt_fn!(Pb13, super::sig::EthRmiiTxd1, 11);
   alt_fn!(Pb13, super::sig::Eventout, 15);

pin!(PB14, Pb14, GPIOB, Gpiob, _PB14, GpioPin, _GPIOB, 14);
   alt_fn!(Pb14, super::sig::Tim2Ch2n, 1);
   alt_fn!(Pb14, super::sig::Tim8Ch2n, 3);
   alt_fn!(Pb14, super::sig::Spi2Miso, 5);
   alt_fn!(Pb14, super::sig::I2s2extSd, 6);
   alt_fn!(Pb14, super::sig::Usart3Rts, 7);
   alt_fn!(Pb14, super::sig::Tim12Ch1, 9);
   alt_fn!(Pb14, super::sig::OtgHsDm, 12);
   alt_fn!(Pb14, super::sig::Eventout, 15);

pin!(PB15, Pb15, GPIOB, Gpiob, _PB15, GpioPin, _GPIOB, 15);
   alt_fn!(Pb15, super::sig::RtcRefin, 0);
   alt_fn!(Pb15, super::sig::Tim1Ch3n, 1);
   alt_fn!(Pb15, super::sig::Tim8Ch3n, 3);
   alt_fn!(Pb15, super::sig::Spi2Mosi, 5);
   alt_fn!(Pb15, super::sig::I2s2Sd, 5);
   alt_fn!(Pb15, super::sig::Tim12Ch2, 9);
   alt_fn!(Pb15, super::sig::OtgHsDp, 12);
   alt_fn!(Pb15, super::sig::Eventout, 15);

pin!(PC0, Pc0, GPIOC, Gpioc, _PC0, GpioPin, _GPIOC, 0);
   alt_fn!(Pc0, super::sig::Adc1In10, 0);
   alt_fn!(Pc0, super::sig::Adc2In10, 0);
   alt_fn!(Pc0, super::sig::Adc3In10, 0);
   alt_fn!(Pc0, super::sig::OtgHsUlpiStp, 10);
   alt_fn!(Pc0, super::sig::FmcSdnwe, 12);
   alt_fn!(Pc0, super::sig::Eventout, 15);

pin!(PC1, Pc1, GPIOC, Gpioc, _PC1, GpioPin, _GPIOC, 1);
   alt_fn!(Pc1, super::sig::Adc1In11, 0);
   alt_fn!(Pc1, super::sig::Adc2In11, 0);
   alt_fn!(Pc1, super::sig::Adc3In11, 0);
   alt_fn!(Pc1, super::sig::EthMdc, 11);
   alt_fn!(Pc1, super::sig::Eventout, 15);

pin!(PC2, Pc2, GPIOC, Gpioc, _PC2, GpioPin, _GPIOC, 2);
   alt_fn!(Pc2, super::sig::Adc1In12, 0);
   alt_fn!(Pc2, super::sig::Adc2In12, 0);
   alt_fn!(Pc2, super::sig::Adc3In12, 0);
   alt_fn!(Pc2, super::sig::OtgHsUlpiDir, 10);
   alt_fn!(Pc2, super::sig::EthMiiTxd2, 11);
   alt_fn!(Pc2, super::sig::FmcSdne0, 12);
   alt_fn!(Pc2, super::sig::Eventout, 15);

pin!(PC3, Pc3, GPIOC, Gpioc, _PC3, GpioPin, _GPIOC, 3);
   alt_fn!(Pc3, super::sig::Adc1In13, 0);
   alt_fn!(Pc3, super::sig::Adc2In13, 0);
   alt_fn!(Pc3, super::sig::Adc3In13, 0);
   alt_fn!(Pc3, super::sig::OtgHsUlpiNxt, 10);
   alt_fn!(Pc3, super::sig::EthMiiTxClk, 11);
   alt_fn!(Pc3, super::sig::FmcSdcke0, 12);
   alt_fn!(Pc3, super::sig::Eventout, 15);

pin!(PC4, Pc4, GPIOC, Gpioc, _PC4, GpioPin, _GPIOC, 4);
   alt_fn!(Pc4, super::sig::Adc1In14, 0);
   alt_fn!(Pc4, super::sig::Adc2In14, 0);
   alt_fn!(Pc4, super::sig::EthMiiRxd0, 11);
   alt_fn!(Pc4, super::sig::EthRmiiRxd0, 11);
   alt_fn!(Pc4, super::sig::Eventout, 15);

pin!(PC5, Pc5, GPIOC, Gpioc, _PC5, GpioPin, _GPIOC, 5);
   alt_fn!(Pc5, super::sig::Adc1In15, 0);
   alt_fn!(Pc5, super::sig::Adc2In15, 0);
   alt_fn!(Pc5, super::sig::EthMiiRxd1, 11);
   alt_fn!(Pc5, super::sig::EthRmiiRxd1, 11);
   alt_fn!(Pc5, super::sig::Eventout, 15);

pin!(PC6, Pc6, GPIOC, Gpioc, _PC6, GpioPin, _GPIOC, 6);
   alt_fn!(Pc6, super::sig::Tim3Ch1, 2);
   alt_fn!(Pc6, super::sig::Tim8Ch1, 3);
   alt_fn!(Pc6, super::sig::I2s2Mck, 5);
   alt_fn!(Pc6, super::sig::Usart6Tx, 8);
   alt_fn!(Pc6, super::sig::SdioD6, 12);
   alt_fn!(Pc6, super::sig::DcmiD0, 13);
   alt_fn!(Pc6, super::sig::LcdHsync, 14);
   alt_fn!(Pc6, super::sig::Eventout, 15);

pin!(PC7, Pc7, GPIOC, Gpioc, _PC7, GpioPin, _GPIOC, 7);
   alt_fn!(Pc7, super::sig::Tim3Ch2, 2);
   alt_fn!(Pc7, super::sig::Tim8Ch2, 3);
   alt_fn!(Pc7, super::sig::I2s3Mck, 6);
   alt_fn!(Pc7, super::sig::Usart6Rx, 8);
   alt_fn!(Pc7, super::sig::SdioD7, 12);
   alt_fn!(Pc7, super::sig::DcmiD1, 13);
   alt_fn!(Pc7, super::sig::LcdG6, 14);
   alt_fn!(Pc7, super::sig::Eventout, 15);

pin!(PC8, Pc8, GPIOC, Gpioc, _PC8, GpioPin, _GPIOC, 8);
   alt_fn!(Pc8, super::sig::Tim3Ch3, 2);
   alt_fn!(Pc8, super::sig::Tim8Ch3, 3);
   alt_fn!(Pc8, super::sig::Usart6Ck, 8);
   alt_fn!(Pc8, super::sig::SdioD0, 12);
   alt_fn!(Pc8, super::sig::DcmiD2, 13);
   alt_fn!(Pc8, super::sig::Eventout, 15);

pin!(PC9, Pc9, GPIOC, Gpioc, _PC9, GpioPin, _GPIOC, 9);
   alt_fn!(Pc9, super::sig::Mco2, 0);
   alt_fn!(Pc9, super::sig::Tim3Ch4, 2);
   alt_fn!(Pc9, super::sig::Tim8Ch4, 3);
   alt_fn!(Pc9, super::sig::I2c3Sda, 4);
   alt_fn!(Pc9, super::sig::I2sCkin, 5);
   alt_fn!(Pc9, super::sig::SdioD1, 12);
   alt_fn!(Pc9, super::sig::DcmiD3, 13);
   alt_fn!(Pc9, super::sig::Eventout, 15);

pin!(PC10, Pc10, GPIOC, Gpioc, _PC10, GpioPin, _GPIOC, 10);
   alt_fn!(Pc10, super::sig::Spi3Sck, 6);
   alt_fn!(Pc10, super::sig::I2s3Ck, 6);
   alt_fn!(Pc10, super::sig::Usart3Tx, 7);
   alt_fn!(Pc10, super::sig::SdioD2, 12);
   alt_fn!(Pc10, super::sig::DcmiD8, 13);
   alt_fn!(Pc10, super::sig::LcdR2, 14);
   alt_fn!(Pc10, super::sig::Eventout, 15);

pin!(PC11, Pc11, GPIOC, Gpioc, _PC11, GpioPin, _GPIOC, 11);
   alt_fn!(Pc11, super::sig::Spi3Miso, 6);
   alt_fn!(Pc11, super::sig::Usart3Rx, 7);
   alt_fn!(Pc11, super::sig::SdioD3, 12);
   alt_fn!(Pc11, super::sig::DcmiD4, 13);
   alt_fn!(Pc11, super::sig::Eventout, 15);

pin!(PC12, Pc12, GPIOC, Gpioc, _PC12, GpioPin, _GPIOC, 12);
   alt_fn!(Pc12, super::sig::Spi3Mosi, 6);
   alt_fn!(Pc12, super::sig::I2s3Sd, 6);
   alt_fn!(Pc12, super::sig::Usart3Ck, 7);
   alt_fn!(Pc12, super::sig::SdioCk, 12);
   alt_fn!(Pc12, super::sig::DcmiD9, 13);
   alt_fn!(Pc12, super::sig::Eventout, 15);

pin!(PC13, Pc13, GPIOC, Gpioc, _PC13, GpioPin, _GPIOC, 13);
   alt_fn!(Pc13, super::sig::Eventout, 15);

pin!(PC14, Pc14, GPIOC, Gpioc, _PC14, GpioPin, _GPIOC, 14);
   alt_fn!(Pc14, super::sig::Eventout, 15);

pin!(PC15, Pc15, GPIOC, Gpioc, _PC15, GpioPin, _GPIOC, 15);
   alt_fn!(Pc15, super::sig::Eventout, 15);

pin!(PD0, Pd0, GPIOD, Gpiod, _PD0, GpioPin, _GPIOD, 0);
   alt_fn!(Pd0, super::sig::Can1Rx, 9);
   alt_fn!(Pd0, super::sig::FmcD2, 12);
   alt_fn!(Pd0, super::sig::Eventout, 15);

pin!(PD1, Pd1, GPIOD, Gpiod, _PD1, GpioPin, _GPIOD, 1);
   alt_fn!(Pd1, super::sig::Can1Tx, 9);
   alt_fn!(Pd1, super::sig::FmcD3, 12);
   alt_fn!(Pd1, super::sig::Eventout, 15);

pin!(PD2, Pd2, GPIOD, Gpiod, _PD2, GpioPin, _GPIOD, 2);
   alt_fn!(Pd2, super::sig::Tim3Etr, 2);
   alt_fn!(Pd2, super::sig::Uart5Rx, 8);
   alt_fn!(Pd2, super::sig::SdioCmd, 12);
   alt_fn!(Pd2, super::sig::DcmiD11, 13);
   alt_fn!(Pd2, super::sig::Eventout, 15);

pin!(PD3, Pd3, GPIOD, Gpiod, _PD3, GpioPin, _GPIOD, 3);
   alt_fn!(Pd3, super::sig::Spi2Sck, 5);
   alt_fn!(Pd3, super::sig::I2s2Ck, 5);
   alt_fn!(Pd3, super::sig::Usart2Cts, 7);
   alt_fn!(Pd3, super::sig::FmcClk, 12);
   alt_fn!(Pd3, super::sig::DcmiD5, 13);
   alt_fn!(Pd3, super::sig::LcdG7, 14);
   alt_fn!(Pd3, super::sig::Eventout, 15);

pin!(PD4, Pd4, GPIOD, Gpiod, _PD4, GpioPin, _GPIOD, 4);
   alt_fn!(Pd4, super::sig::Usart2Rts, 7);
   alt_fn!(Pd4, super::sig::FmcNoe, 12);
   alt_fn!(Pd4, super::sig::Eventout, 15);

pin!(PD5, Pd5, GPIOD, Gpiod, _PD5, GpioPin, _GPIOD, 5);
   alt_fn!(Pd5, super::sig::Usart2Tx, 7);
   alt_fn!(Pd5, super::sig::FmcNwe, 12);
   alt_fn!(Pd5, super::sig::Eventout, 15);

pin!(PD6, Pd6, GPIOD, Gpiod, _PD6, GpioPin, _GPIOD, 6);
   alt_fn!(Pd6, super::sig::Spi3Mosi, 5);
   alt_fn!(Pd6, super::sig::I2s3Sd, 5);
   alt_fn!(Pd6, super::sig::Sai1SdA, 6);
   alt_fn!(Pd6, super::sig::Usart2Rx, 7);
   alt_fn!(Pd6, super::sig::FmcNwait, 12);
   alt_fn!(Pd6, super::sig::DcmiD10, 13);
   alt_fn!(Pd6, super::sig::LcdB2, 14);
   alt_fn!(Pd6, super::sig::Eventout, 15);

pin!(PD7, Pd7, GPIOD, Gpiod, _PD7, GpioPin, _GPIOD, 7);
   alt_fn!(Pd7, super::sig::Usart2Ck, 7);
   alt_fn!(Pd7, super::sig::FmcNe1, 12);
   alt_fn!(Pd7, super::sig::FmcNce2, 12);
   alt_fn!(Pd7, super::sig::Eventout, 15);

pin!(PD8, Pd8, GPIOD, Gpiod, _PD8, GpioPin, _GPIOD, 8);
   alt_fn!(Pd8, super::sig::Usart3Tx, 7);
   alt_fn!(Pd8, super::sig::FmcD13, 12);
   alt_fn!(Pd8, super::sig::Eventout, 15);

pin!(PD9, Pd9, GPIOD, Gpiod, _PD9, GpioPin, _GPIOD, 9);
   alt_fn!(Pd9, super::sig::Usart3Rx, 7);
   alt_fn!(Pd9, super::sig::FmcD14, 12);
   alt_fn!(Pd9, super::sig::Eventout, 15);

pin!(PD10, Pd10, GPIOD, Gpiod, _PD10, GpioPin, _GPIOD, 10);
   alt_fn!(Pd10, super::sig::Usart3Ck, 7);
   alt_fn!(Pd10, super::sig::FmcD15, 12);
   alt_fn!(Pd10, super::sig::Eventout, 15);

pin!(PD11, Pd11, GPIOD, Gpiod, _PD11, GpioPin, _GPIOD, 11);
   alt_fn!(Pd11, super::sig::Usart3Cts, 7);
   alt_fn!(Pd11, super::sig::FmcA16, 12);
   alt_fn!(Pd11, super::sig::Eventout, 15);

pin!(PD12, Pd12, GPIOD, Gpiod, _PD12, GpioPin, _GPIOD, 12);
   alt_fn!(Pd12, super::sig::Tim4Ch1, 2);
   alt_fn!(Pd12, super::sig::Usart3Rts, 7);
   alt_fn!(Pd12, super::sig::FmcA17, 12);
   alt_fn!(Pd12, super::sig::Eventout, 15);

pin!(PD13, Pd13, GPIOD, Gpiod, _PD13, GpioPin, _GPIOD, 13);
   alt_fn!(Pd13, super::sig::Tim4Ch2, 2);
   alt_fn!(Pd13, super::sig::FmcA18, 12);
   alt_fn!(Pd13, super::sig::Eventout, 15);

pin!(PD14, Pd14, GPIOD, Gpiod, _PD14, GpioPin, _GPIOD, 14);
   alt_fn!(Pd14, super::sig::Tim4Ch3, 2);
   alt_fn!(Pd14, super::sig::FmcD0, 12);
   alt_fn!(Pd14, super::sig::Eventout, 15);

pin!(PD15, Pd15, GPIOD, Gpiod, _PD15, GpioPin, _GPIOD, 15);
   alt_fn!(Pd15, super::sig::Tim4Ch4, 2);
   alt_fn!(Pd15, super::sig::FmcD1, 12);
   alt_fn!(Pd15, super::sig::Eventout, 15);

pin!(PE0, Pe0, GPIOE, Gpioe, _PE0, GpioPin, _GPIOE, 0);
   alt_fn!(Pe0, super::sig::Tim4Etr, 2);
   alt_fn!(Pe0, super::sig::Uart8Rx, 8);
   alt_fn!(Pe0, super::sig::FmcNbl0, 12);
   alt_fn!(Pe0, super::sig::Eventout, 15);

pin!(PE1, Pe1, GPIOE, Gpioe, _PE1, GpioPin, _GPIOE, 1);
   alt_fn!(Pe1, super::sig::Uart8Tx, 8);
   alt_fn!(Pe1, super::sig::FmcNbl1, 12);
   alt_fn!(Pe1, super::sig::Eventout, 15);

pin!(PE2, Pe2, GPIOE, Gpioe, _PE2, GpioPin, _GPIOE, 2);
   alt_fn!(Pe2, super::sig::Traceclk, 0);
   alt_fn!(Pe2, super::sig::Spi4Sck, 5);
   alt_fn!(Pe2, super::sig::Sai1MclkA, 6);
   alt_fn!(Pe2, super::sig::EthMiiTxd3, 11);
   alt_fn!(Pe2, super::sig::FmcA23, 12);
   alt_fn!(Pe2, super::sig::DcmiD2, 13);
   alt_fn!(Pe2, super::sig::Eventout, 15);

pin!(PE3, Pe3, GPIOE, Gpioe, _PE3, GpioPin, _GPIOE, 3);
   alt_fn!(Pe3, super::sig::Traced0, 0);
   alt_fn!(Pe3, super::sig::Sai1SdB, 6);
   alt_fn!(Pe3, super::sig::FmcA19, 12);
   alt_fn!(Pe3, super::sig::DcmiD3, 13);
   alt_fn!(Pe3, super::sig::Eventout, 15);

pin!(PE4, Pe4, GPIOE, Gpioe, _PE4, GpioPin, _GPIOE, 4);
   alt_fn!(Pe4, super::sig::Traced1, 0);
   alt_fn!(Pe4, super::sig::Spi4Nss, 5);
   alt_fn!(Pe4, super::sig::Sai1FsA, 6);
   alt_fn!(Pe4, super::sig::FmcA20, 12);
   alt_fn!(Pe4, super::sig::Eventout, 15);

pin!(PE5, Pe5, GPIOE, Gpioe, _PE5, GpioPin, _GPIOE, 5);
   alt_fn!(Pe5, super::sig::Traced2, 0);
   alt_fn!(Pe5, super::sig::Tim9Ch1, 3);
   alt_fn!(Pe5, super::sig::Spi4Miso, 5);
   alt_fn!(Pe5, super::sig::Sai1SckA, 6);
   alt_fn!(Pe5, super::sig::FmcA21, 12);
   alt_fn!(Pe5, super::sig::Eventout, 15);

pin!(PE6, Pe6, GPIOE, Gpioe, _PE6, GpioPin, _GPIOE, 6);
   alt_fn!(Pe6, super::sig::Traced3, 0);
   alt_fn!(Pe6, super::sig::Tim9Ch2, 3);
   alt_fn!(Pe6, super::sig::Spi4Mosi, 5);
   alt_fn!(Pe6, super::sig::Sai1SdA, 6);
   alt_fn!(Pe6, super::sig::FmcA22, 12);
   alt_fn!(Pe6, super::sig::DcmiD4, 13);
   alt_fn!(Pe6, super::sig::Eventout, 15);

pin!(PE7, Pe7, GPIOE, Gpioe, _PE7, GpioPin, _GPIOE, 7);
   alt_fn!(Pe7, super::sig::Tim1Etr, 1);
   alt_fn!(Pe7, super::sig::FmcD4, 12);
   alt_fn!(Pe7, super::sig::DcmiD6, 13);
   alt_fn!(Pe7, super::sig::Eventout, 15);

pin!(PE8, Pe8, GPIOE, Gpioe, _PE8, GpioPin, _GPIOE, 8);
   alt_fn!(Pe8, super::sig::Tim1Ch1n, 1);
   alt_fn!(Pe8, super::sig::FmcD5, 12);
   alt_fn!(Pe8, super::sig::DcmiD7, 13);
   alt_fn!(Pe8, super::sig::Eventout, 15);

pin!(PE9, Pe9, GPIOE, Gpioe, _PE9, GpioPin, _GPIOE, 9);
   alt_fn!(Pe9, super::sig::Tim1Ch1, 1);
   alt_fn!(Pe9, super::sig::FmcD6, 12);
   alt_fn!(Pe9, super::sig::Eventout, 15);

pin!(PE10, Pe10, GPIOE, Gpioe, _PE10, GpioPin, _GPIOE, 10);
   alt_fn!(Pe10, super::sig::Tim1Ch2n, 1);
   alt_fn!(Pe10, super::sig::FmcD7, 12);
   alt_fn!(Pe10, super::sig::Eventout, 15);

pin!(PE11, Pe11, GPIOE, Gpioe, _PE11, GpioPin, _GPIOE, 11);
   alt_fn!(Pe11, super::sig::Tim1Ch2, 1);
   alt_fn!(Pe11, super::sig::Spi4Nss, 5);
   alt_fn!(Pe11, super::sig::FmcD8, 12);
   alt_fn!(Pe11, super::sig::LcdG3, 14);
   alt_fn!(Pe11, super::sig::Eventout, 15);

pin!(PE12, Pe12, GPIOE, Gpioe, _PE12, GpioPin, _GPIOE, 12);
   alt_fn!(Pe12, super::sig::Tim1Ch3n, 1);
   alt_fn!(Pe12, super::sig::Spi4Sck, 5);
   alt_fn!(Pe12, super::sig::FmcD9, 12);
   alt_fn!(Pe12, super::sig::LcdB4, 14);
   alt_fn!(Pe12, super::sig::Eventout, 15);

pin!(PE13, Pe13, GPIOE, Gpioe, _PE13, GpioPin, _GPIOE, 13);
   alt_fn!(Pe13, super::sig::Tim1Ch3, 1);
   alt_fn!(Pe13, super::sig::Spi4Miso, 5);
   alt_fn!(Pe13, super::sig::FmcD10, 12);
   alt_fn!(Pe13, super::sig::LcdDe, 14);
   alt_fn!(Pe13, super::sig::Eventout, 15);

pin!(PE14, Pe14, GPIOE, Gpioe, _PE14, GpioPin, _GPIOE, 14);
   alt_fn!(Pe14, super::sig::Tim1Ch4, 1);
   alt_fn!(Pe14, super::sig::Spi4Mosi, 5);
   alt_fn!(Pe14, super::sig::FmcD11, 12);
   alt_fn!(Pe14, super::sig::LcdClk, 14);
   alt_fn!(Pe14, super::sig::Eventout, 15);

pin!(PE15, Pe15, GPIOE, Gpioe, _PE15, GpioPin, _GPIOE, 15);
   alt_fn!(Pe15, super::sig::Tim1Bkin, 1);
   alt_fn!(Pe15, super::sig::FmcD12, 12);
   alt_fn!(Pe15, super::sig::LcdR7, 14);
   alt_fn!(Pe15, super::sig::Eventout, 15);

pin!(PF0, Pf0, GPIOF, Gpiof, _PF0, GpioPin, _GPIOF, 0);
   alt_fn!(Pf0, super::sig::I2c2Sda, 4);
   alt_fn!(Pf0, super::sig::FmcA0, 12);
   alt_fn!(Pf0, super::sig::Eventout, 15);

pin!(PF1, Pf1, GPIOF, Gpiof, _PF1, GpioPin, _GPIOF, 1);
   alt_fn!(Pf1, super::sig::I2c2Scl, 4);
   alt_fn!(Pf1, super::sig::FmcA1, 12);
   alt_fn!(Pf1, super::sig::Eventout, 15);

pin!(PF2, Pf2, GPIOF, Gpiof, _PF2, GpioPin, _GPIOF, 2);
   alt_fn!(Pf2, super::sig::I2c2Smba, 4);
   alt_fn!(Pf2, super::sig::FmcA2, 12);
   alt_fn!(Pf2, super::sig::Eventout, 15);

pin!(PF3, Pf3, GPIOF, Gpiof, _PF3, GpioPin, _GPIOF, 3);
   alt_fn!(Pf3, super::sig::Adc3In9, 0);
   alt_fn!(Pf3, super::sig::FmcA3, 12);
   alt_fn!(Pf3, super::sig::Eventout, 15);

pin!(PF4, Pf4, GPIOF, Gpiof, _PF4, GpioPin, _GPIOF, 4);
   alt_fn!(Pf4, super::sig::Adc3In14, 0);
   alt_fn!(Pf4, super::sig::FmcA4, 12);
   alt_fn!(Pf4, super::sig::Eventout, 15);

pin!(PF5, Pf5, GPIOF, Gpiof, _PF5, GpioPin, _GPIOF, 5);
   alt_fn!(Pf5, super::sig::Adc3In15, 0);
   alt_fn!(Pf5, super::sig::FmcA5, 12);
   alt_fn!(Pf5, super::sig::Eventout, 15);

pin!(PF6, Pf6, GPIOF, Gpiof, _PF6, GpioPin, _GPIOF, 6);
   alt_fn!(Pf6, super::sig::Adc3In4, 0);
   alt_fn!(Pf6, super::sig::Tim10Ch1, 3);
   alt_fn!(Pf6, super::sig::Spi5Nss, 5);
   alt_fn!(Pf6, super::sig::Sai1SdB, 6);
   alt_fn!(Pf6, super::sig::Uart7Rx, 8);
   alt_fn!(Pf6, super::sig::FmcNiord, 12);
   alt_fn!(Pf6, super::sig::Eventout, 15);

pin!(PF7, Pf7, GPIOF, Gpiof, _PF7, GpioPin, _GPIOF, 7);
   alt_fn!(Pf7, super::sig::Adc3In5, 0);
   alt_fn!(Pf7, super::sig::Tim11Ch1, 3);
   alt_fn!(Pf7, super::sig::Spi5Sck, 5);
   alt_fn!(Pf7, super::sig::Sai1MclkB, 6);
   alt_fn!(Pf7, super::sig::Uart7Tx, 8);
   alt_fn!(Pf7, super::sig::FmcNreg, 12);
   alt_fn!(Pf7, super::sig::Eventout, 15);

pin!(PF8, Pf8, GPIOF, Gpiof, _PF8, GpioPin, _GPIOF, 8);
   alt_fn!(Pf8, super::sig::Adc3In6, 0);
   alt_fn!(Pf8, super::sig::Spi5Miso, 5);
   alt_fn!(Pf8, super::sig::SaiSckB, 6);
   alt_fn!(Pf8, super::sig::Tim13Ch1, 9);
   alt_fn!(Pf8, super::sig::FmcNiowr, 12);
   alt_fn!(Pf8, super::sig::Eventout, 15);

pin!(PF9, Pf9, GPIOF, Gpiof, _PF9, GpioPin, _GPIOF, 9);
   alt_fn!(Pf9, super::sig::Adc3In7, 0);
   alt_fn!(Pf9, super::sig::Spi5Mosi, 5);
   alt_fn!(Pf9, super::sig::SaiFsB, 6);
   alt_fn!(Pf9, super::sig::Tim14Ch1, 9);
   alt_fn!(Pf9, super::sig::FmcCd, 12);
   alt_fn!(Pf9, super::sig::Eventout, 15);

pin!(PF10, Pf10, GPIOF, Gpiof, _PF10, GpioPin, _GPIOF, 10);
   alt_fn!(Pf10, super::sig::Adc3In8, 0);
   alt_fn!(Pf10, super::sig::FmcIntr, 12);
   alt_fn!(Pf10, super::sig::DcmiD11, 13);
   alt_fn!(Pf10, super::sig::LcdDe, 14);
   alt_fn!(Pf10, super::sig::Eventout, 15);

pin!(PF11, Pf11, GPIOF, Gpiof, _PF11, GpioPin, _GPIOF, 11);
   alt_fn!(Pf11, super::sig::Spi5Mosi, 5);
   alt_fn!(Pf11, super::sig::FmcSdnras, 12);
   alt_fn!(Pf11, super::sig::DcmiD12, 13);
   alt_fn!(Pf11, super::sig::Eventout, 15);

pin!(PF12, Pf12, GPIOF, Gpiof, _PF12, GpioPin, _GPIOF, 12);
   alt_fn!(Pf12, super::sig::FmcA6, 12);
   alt_fn!(Pf12, super::sig::Eventout, 15);

pin!(PF13, Pf13, GPIOF, Gpiof, _PF13, GpioPin, _GPIOF, 13);
   alt_fn!(Pf13, super::sig::FmcA7, 12);
   alt_fn!(Pf13, super::sig::Eventout, 15);

pin!(PF14, Pf14, GPIOF, Gpiof, _PF14, GpioPin, _GPIOF, 14);
   alt_fn!(Pf14, super::sig::FmcA8, 12);
   alt_fn!(Pf14, super::sig::Eventout, 15);

pin!(PF15, Pf15, GPIOF, Gpiof, _PF15, GpioPin, _GPIOF, 15);
   alt_fn!(Pf15, super::sig::FmcA9, 12);
   alt_fn!(Pf15, super::sig::Eventout, 15);

pin!(PG0, Pg0, GPIOG, Gpiog, _PG0, GpioPin, _GPIOG, 0);
   alt_fn!(Pg0, super::sig::FmcA10, 12);
   alt_fn!(Pg0, super::sig::Eventout, 15);

pin!(PG1, Pg1, GPIOG, Gpiog, _PG1, GpioPin, _GPIOG, 1);
   alt_fn!(Pg1, super::sig::FmcA11, 12);
   alt_fn!(Pg1, super::sig::Eventout, 15);

pin!(PG2, Pg2, GPIOG, Gpiog, _PG2, GpioPin, _GPIOG, 2);
   alt_fn!(Pg2, super::sig::FmcA12, 12);
   alt_fn!(Pg2, super::sig::Eventout, 15);

pin!(PG3, Pg3, GPIOG, Gpiog, _PG3, GpioPin, _GPIOG, 3);
   alt_fn!(Pg3, super::sig::FmcA13, 12);
   alt_fn!(Pg3, super::sig::Eventout, 15);

pin!(PG4, Pg4, GPIOG, Gpiog, _PG4, GpioPin, _GPIOG, 4);
   alt_fn!(Pg4, super::sig::FmcA14, 12);
   alt_fn!(Pg4, super::sig::FmcBa0, 12);
   alt_fn!(Pg4, super::sig::Eventout, 15);

pin!(PG5, Pg5, GPIOG, Gpiog, _PG5, GpioPin, _GPIOG, 5);
   alt_fn!(Pg5, super::sig::FmcA15, 12);
   alt_fn!(Pg5, super::sig::FmcBa1, 12);
   alt_fn!(Pg5, super::sig::Eventout, 15);

pin!(PG6, Pg6, GPIOG, Gpiog, _PG6, GpioPin, _GPIOG, 6);
   alt_fn!(Pg6, super::sig::FmcInt2, 12);
   alt_fn!(Pg6, super::sig::DcmiD12, 13);
   alt_fn!(Pg6, super::sig::LcdR7, 14);
   alt_fn!(Pg6, super::sig::Eventout, 15);

pin!(PG7, Pg7, GPIOG, Gpiog, _PG7, GpioPin, _GPIOG, 7);
   alt_fn!(Pg7, super::sig::Usart6Ck, 8);
   alt_fn!(Pg7, super::sig::FmcInt3, 12);
   alt_fn!(Pg7, super::sig::DcmiD13, 13);
   alt_fn!(Pg7, super::sig::LcdClk, 14);
   alt_fn!(Pg7, super::sig::Eventout, 15);

pin!(PG8, Pg8, GPIOG, Gpiog, _PG8, GpioPin, _GPIOG, 8);
   alt_fn!(Pg8, super::sig::Spi6Nss, 5);
   alt_fn!(Pg8, super::sig::Usart6Rts, 8);
   alt_fn!(Pg8, super::sig::EthPsOut, 11);
   alt_fn!(Pg8, super::sig::FmcSdclk, 12);
   alt_fn!(Pg8, super::sig::Eventout, 15);

pin!(PG9, Pg9, GPIOG, Gpiog, _PG9, GpioPin, _GPIOG, 9);
   alt_fn!(Pg9, super::sig::Usart6Rx, 8);
   alt_fn!(Pg9, super::sig::FmcNe2, 12);
   alt_fn!(Pg9, super::sig::FmcNce3, 12);
   alt_fn!(Pg9, super::sig::DcmiVsync, 13);
   alt_fn!(Pg9, super::sig::Eventout, 15);

pin!(PG10, Pg10, GPIOG, Gpiog, _PG10, GpioPin, _GPIOG, 10);
   alt_fn!(Pg10, super::sig::LcdG3, 9);
   alt_fn!(Pg10, super::sig::FmcNce41, 12);
   alt_fn!(Pg10, super::sig::FmcNe3, 12);
   alt_fn!(Pg10, super::sig::DcmiD2, 13);
   alt_fn!(Pg10, super::sig::LcdB2, 14);
   alt_fn!(Pg10, super::sig::Eventout, 15);

pin!(PG11, Pg11, GPIOG, Gpiog, _PG11, GpioPin, _GPIOG, 11);
   alt_fn!(Pg11, super::sig::Spi6Miso, 5);
   alt_fn!(Pg11, super::sig::Usart6Rts, 8);
   alt_fn!(Pg11, super::sig::EthMiiTxEn, 11);
   alt_fn!(Pg11, super::sig::EthRmiiTxEn, 11);
   alt_fn!(Pg11, super::sig::FmcNce42, 12);
   alt_fn!(Pg11, super::sig::DcmiD3, 13);
   alt_fn!(Pg11, super::sig::LcdB3, 14);
   alt_fn!(Pg11, super::sig::Eventout, 15);

pin!(PG12, Pg12, GPIOG, Gpiog, _PG12, GpioPin, _GPIOG, 12);
   alt_fn!(Pg12, super::sig::Spi6Sck, 5);
   alt_fn!(Pg12, super::sig::Usart6Cts, 8);
   alt_fn!(Pg12, super::sig::LcdB4, 9);
   alt_fn!(Pg12, super::sig::FmcNe4, 12);
   alt_fn!(Pg12, super::sig::LcdB1, 14);
   alt_fn!(Pg12, super::sig::Eventout, 15);

pin!(PG13, Pg13, GPIOG, Gpiog, _PG13, GpioPin, _GPIOG, 13);
   alt_fn!(Pg13, super::sig::Spi6Mosi, 5);
   alt_fn!(Pg13, super::sig::Usart6Tx, 8);
   alt_fn!(Pg13, super::sig::EthMiiTxd0, 11);
   alt_fn!(Pg13, super::sig::EthRmiiTxd0, 11);
   alt_fn!(Pg13, super::sig::FmcA24, 12);
   alt_fn!(Pg13, super::sig::Eventout, 15);

pin!(PG14, Pg14, GPIOG, Gpiog, _PG14, GpioPin, _GPIOG, 14);
   alt_fn!(Pg14, super::sig::Usart6Cts, 8);
   alt_fn!(Pg14, super::sig::EthMiiTxd1, 11);
   alt_fn!(Pg14, super::sig::EthRmiiTxd1, 11);
   alt_fn!(Pg14, super::sig::FmcA25, 12);
   alt_fn!(Pg14, super::sig::Eventout, 15);

pin!(PG15, Pg15, GPIOG, Gpiog, _PG15, GpioPin, _GPIOG, 15);
   alt_fn!(Pg15, super::sig::FmcSndcas, 12);
   alt_fn!(Pg15, super::sig::DcmiD13, 13);
   alt_fn!(Pg15, super::sig::Eventout, 15);

pin!(PH0, Ph0, GPIOH, Gpioh, _PH0, GpioPin, _GPIOH, 0);
   alt_fn!(Ph0, super::sig::Eventout, 15);

pin!(PH1, Ph1, GPIOH, Gpioh, _PH1, GpioPin, _GPIOH, 1);
   alt_fn!(Ph1, super::sig::Eventout, 15);

pin!(PH2, Ph2, GPIOH, Gpioh, _PH2, GpioPin, _GPIOH, 2);
   alt_fn!(Ph2, super::sig::EthMiiCrs, 11);
   alt_fn!(Ph2, super::sig::FmcSdcke0, 12);
   alt_fn!(Ph2, super::sig::LcdR0, 14);
   alt_fn!(Ph2, super::sig::Eventout, 15);

pin!(PH3, Ph3, GPIOH, Gpioh, _PH3, GpioPin, _GPIOH, 3);
   alt_fn!(Ph3, super::sig::EthMiiCol, 11);
   alt_fn!(Ph3, super::sig::FmcSdne0, 12);
   alt_fn!(Ph3, super::sig::LcdR1, 14);
   alt_fn!(Ph3, super::sig::Eventout, 15);

pin!(PH4, Ph4, GPIOH, Gpioh, _PH4, GpioPin, _GPIOH, 4);
   alt_fn!(Ph4, super::sig::I2c2Scl, 4);
   alt_fn!(Ph4, super::sig::OtgHsUlpiNxt, 10);
   alt_fn!(Ph4, super::sig::Eventout, 15);

pin!(PH5, Ph5, GPIOH, Gpioh, _PH5, GpioPin, _GPIOH, 5);
   alt_fn!(Ph5, super::sig::I2c2Sda, 4);
   alt_fn!(Ph5, super::sig::Spi5Nss, 5);
   alt_fn!(Ph5, super::sig::FmcSdnwe, 12);
   alt_fn!(Ph5, super::sig::Eventout, 15);

pin!(PH6, Ph6, GPIOH, Gpioh, _PH6, GpioPin, _GPIOH, 6);
   alt_fn!(Ph6, super::sig::I2c2Smba, 4);
   alt_fn!(Ph6, super::sig::Spi5Sck, 5);
   alt_fn!(Ph6, super::sig::Tim12Ch1, 9);
   alt_fn!(Ph6, super::sig::FmcSdne1, 12);
   alt_fn!(Ph6, super::sig::DcmiD8, 13);
   alt_fn!(Ph6, super::sig::Eventout, 15);

pin!(PH7, Ph7, GPIOH, Gpioh, _PH7, GpioPin, _GPIOH, 7);
   alt_fn!(Ph7, super::sig::I2c3Scl, 4);
   alt_fn!(Ph7, super::sig::Spi5Miso, 5);
   alt_fn!(Ph7, super::sig::EthMiiRxd3, 11);
   alt_fn!(Ph7, super::sig::FmcSdcke1, 12);
   alt_fn!(Ph7, super::sig::DcmiD9, 13);
   alt_fn!(Ph7, super::sig::Eventout, 15);

pin!(PH8, Ph8, GPIOH, Gpioh, _PH8, GpioPin, _GPIOH, 8);
   alt_fn!(Ph8, super::sig::I2c3Sda, 4);
   alt_fn!(Ph8, super::sig::FmcD16, 12);
   alt_fn!(Ph8, super::sig::DcmiHsync, 13);
   alt_fn!(Ph8, super::sig::LcdR2, 14);
   alt_fn!(Ph8, super::sig::Eventout, 15);

pin!(PH9, Ph9, GPIOH, Gpioh, _PH9, GpioPin, _GPIOH, 9);
   alt_fn!(Ph9, super::sig::I2c3Smba, 4);
   alt_fn!(Ph9, super::sig::Tim2Ch2, 9);
   alt_fn!(Ph9, super::sig::FmcD17, 12);
   alt_fn!(Ph9, super::sig::DcmiD0, 13);
   alt_fn!(Ph9, super::sig::LcdR3, 14);
   alt_fn!(Ph9, super::sig::Eventout, 15);

pin!(PH10, Ph10, GPIOH, Gpioh, _PH10, GpioPin, _GPIOH, 10);
   alt_fn!(Ph10, super::sig::Tim5Ch1, 2);
   alt_fn!(Ph10, super::sig::FmcD18, 12);
   alt_fn!(Ph10, super::sig::DcmiD1, 13);
   alt_fn!(Ph10, super::sig::LcdR4, 14);
   alt_fn!(Ph10, super::sig::Eventout, 15);

pin!(PH11, Ph11, GPIOH, Gpioh, _PH11, GpioPin, _GPIOH, 11);
   alt_fn!(Ph11, super::sig::Tim5Ch2, 2);
   alt_fn!(Ph11, super::sig::FmcD19, 12);
   alt_fn!(Ph11, super::sig::DcmiD2, 13);
   alt_fn!(Ph11, super::sig::LcdR5, 14);
   alt_fn!(Ph11, super::sig::Eventout, 15);

pin!(PH12, Ph12, GPIOH, Gpioh, _PH12, GpioPin, _GPIOH, 12);
   alt_fn!(Ph12, super::sig::Tim5Ch3, 2);
   alt_fn!(Ph12, super::sig::FmcD20, 12);
   alt_fn!(Ph12, super::sig::DcmiD3, 13);
   alt_fn!(Ph12, super::sig::LcdR6, 14);
   alt_fn!(Ph12, super::sig::Eventout, 15);

pin!(PH13, Ph13, GPIOH, Gpioh, _PH13, GpioPin, _GPIOH, 13);
   alt_fn!(Ph13, super::sig::Tim8Ch1n, 3);
   alt_fn!(Ph13, super::sig::Can1Tx, 9);
   alt_fn!(Ph13, super::sig::FmcD21, 12);
   alt_fn!(Ph13, super::sig::LcdG2, 14);
   alt_fn!(Ph13, super::sig::Eventout, 15);

pin!(PH14, Ph14, GPIOH, Gpioh, _PH14, GpioPin, _GPIOH, 14);
   alt_fn!(Ph14, super::sig::Tim8Ch2n, 3);
   alt_fn!(Ph14, super::sig::FmcD22, 12);
   alt_fn!(Ph14, super::sig::DcmiD4, 13);
   alt_fn!(Ph14, super::sig::LcdG3, 14);
   alt_fn!(Ph14, super::sig::Eventout, 15);

pin!(PH15, Ph15, GPIOH, Gpioh, _PH15, GpioPin, _GPIOH, 15);
   alt_fn!(Ph15, super::sig::Tim8Ch3n, 3);
   alt_fn!(Ph15, super::sig::FmcD23, 12);
   alt_fn!(Ph15, super::sig::DcmiD11, 13);
   alt_fn!(Ph15, super::sig::LcdG4, 14);
   alt_fn!(Ph15, super::sig::Eventout, 15);

pin!(PI0, Pi0, GPIOI, Gpioi, _PI0, GpioPin, _GPIOI, 0);
   alt_fn!(Pi0, super::sig::Tim5Ch4, 2);
   alt_fn!(Pi0, super::sig::Spi2Nss, 5);
   alt_fn!(Pi0, super::sig::I2s2Ws, 5);
   alt_fn!(Pi0, super::sig::FmcD24, 12);
   alt_fn!(Pi0, super::sig::DcmiD13, 13);
   alt_fn!(Pi0, super::sig::LcdG5, 14);
   alt_fn!(Pi0, super::sig::Eventout, 15);

pin!(PI1, Pi1, GPIOI, Gpioi, _PI1, GpioPin, _GPIOI, 1);
   alt_fn!(Pi1, super::sig::Spi2Sck, 5);
   alt_fn!(Pi1, super::sig::I2s2Ck, 5);
   alt_fn!(Pi1, super::sig::FmcD25, 12);
   alt_fn!(Pi1, super::sig::DcmiD8, 13);
   alt_fn!(Pi1, super::sig::LcdG6, 14);
   alt_fn!(Pi1, super::sig::Eventout, 15);

pin!(PI2, Pi2, GPIOI, Gpioi, _PI2, GpioPin, _GPIOI, 2);
   alt_fn!(Pi2, super::sig::Tim8Ch4, 3);
   alt_fn!(Pi2, super::sig::Spi2Miso, 5);
   alt_fn!(Pi2, super::sig::I2s2extSd, 6);
   alt_fn!(Pi2, super::sig::FmcD26, 12);
   alt_fn!(Pi2, super::sig::DcmiD9, 13);
   alt_fn!(Pi2, super::sig::LcdG7, 14);
   alt_fn!(Pi2, super::sig::Eventout, 15);

pin!(PI3, Pi3, GPIOI, Gpioi, _PI3, GpioPin, _GPIOI, 3);
   alt_fn!(Pi3, super::sig::Tim8Etr, 3);
   alt_fn!(Pi3, super::sig::Spi2Mosi, 5);
   alt_fn!(Pi3, super::sig::I2s2Sd, 5);
   alt_fn!(Pi3, super::sig::FmcD27, 12);
   alt_fn!(Pi3, super::sig::DcmiD10, 13);
   alt_fn!(Pi3, super::sig::Eventout, 15);

pin!(PI4, Pi4, GPIOI, Gpioi, _PI4, GpioPin, _GPIOI, 4);
   alt_fn!(Pi4, super::sig::Tim8Bkin, 3);
   alt_fn!(Pi4, super::sig::FmcNbl2, 12);
   alt_fn!(Pi4, super::sig::DcmiD5, 13);
   alt_fn!(Pi4, super::sig::LcdB4, 14);
   alt_fn!(Pi4, super::sig::Eventout, 15);

pin!(PI5, Pi5, GPIOI, Gpioi, _PI5, GpioPin, _GPIOI, 5);
   alt_fn!(Pi5, super::sig::Tim8Ch1, 3);
   alt_fn!(Pi5, super::sig::FmcNbl3, 12);
   alt_fn!(Pi5, super::sig::DcmVsync, 13);
   alt_fn!(Pi5, super::sig::LcdB5, 14);
   alt_fn!(Pi5, super::sig::Eventout, 15);

pin!(PI6, Pi6, GPIOI, Gpioi, _PI6, GpioPin, _GPIOI, 6);
   alt_fn!(Pi6, super::sig::Tim8Ch2, 3);
   alt_fn!(Pi6, super::sig::FmcD28, 12);
   alt_fn!(Pi6, super::sig::DcmiD6, 13);
   alt_fn!(Pi6, super::sig::LcdB6, 14);
   alt_fn!(Pi6, super::sig::Eventout, 15);

pin!(PI7, Pi7, GPIOI, Gpioi, _PI7, GpioPin, _GPIOI, 7);
   alt_fn!(Pi7, super::sig::Tim8Ch3, 3);
   alt_fn!(Pi7, super::sig::FmcD29, 12);
   alt_fn!(Pi7, super::sig::DcmiD7, 13);
   alt_fn!(Pi7, super::sig::LcdB7, 14);
   alt_fn!(Pi7, super::sig::Eventout, 15);

pin!(PI8, Pi8, GPIOI, Gpioi, _PI8, GpioPin, _GPIOI, 8);
   alt_fn!(Pi8, super::sig::Eventout, 15);

pin!(PI9, Pi9, GPIOI, Gpioi, _PI9, GpioPin, _GPIOI, 9);
   alt_fn!(Pi9, super::sig::Can1Rx, 9);
   alt_fn!(Pi9, super::sig::FmcD30, 12);
   alt_fn!(Pi9, super::sig::LcdVsync, 14);
   alt_fn!(Pi9, super::sig::Eventout, 15);

pin!(PI10, Pi10, GPIOI, Gpioi, _PI10, GpioPin, _GPIOI, 10);
   alt_fn!(Pi10, super::sig::EthMiiRxEr, 11);
   alt_fn!(Pi10, super::sig::FmcD31, 12);
   alt_fn!(Pi10, super::sig::LcdHsync, 14);
   alt_fn!(Pi10, super::sig::Eventout, 15);

pin!(PI11, Pi11, GPIOI, Gpioi, _PI11, GpioPin, _GPIOI, 11);
   alt_fn!(Pi11, super::sig::OtgHsUlpiDir, 10);
   alt_fn!(Pi11, super::sig::Eventout, 15);

pin!(PI12, Pi12, GPIOI, Gpioi, _PI12, GpioPin, _GPIOI, 12);
   alt_fn!(Pi12, super::sig::LcdHsync, 14);
   alt_fn!(Pi12, super::sig::Eventout, 15);

pin!(PI13, Pi13, GPIOI, Gpioi, _PI13, GpioPin, _GPIOI, 13);
   alt_fn!(Pi13, super::sig::LcdVsync, 14);
   alt_fn!(Pi13, super::sig::Eventout, 15);

pin!(PI14, Pi14, GPIOI, Gpioi, _PI14, GpioPin, _GPIOI, 14);
   alt_fn!(Pi14, super::sig::LcdClk, 14);
   alt_fn!(Pi14, super::sig::Eventout, 15);

pin!(PI15, Pi15, GPIOI, Gpioi, _PI15, GpioPin, _GPIOI, 15);
   alt_fn!(Pi15, super::sig::LcdR0, 14);
   alt_fn!(Pi15, super::sig::Eventout, 15);

pin!(PJ0, Pj0, GPIOJ, Gpioj, _PJ0, GpioPin, _GPIOJ, 0);
   alt_fn!(Pj0, super::sig::LcdR1, 14);
   alt_fn!(Pj0, super::sig::Eventout, 15);

pin!(PJ1, Pj1, GPIOJ, Gpioj, _PJ1, GpioPin, _GPIOJ, 1);
   alt_fn!(Pj1, super::sig::LcdR2, 14);
   alt_fn!(Pj1, super::sig::Eventout, 15);

pin!(PJ2, Pj2, GPIOJ, Gpioj, _PJ2, GpioPin, _GPIOJ, 2);
   alt_fn!(Pj2, super::sig::LcdR3, 14);
   alt_fn!(Pj2, super::sig::Eventout, 15);

pin!(PJ3, Pj3, GPIOJ, Gpioj, _PJ3, GpioPin, _GPIOJ, 3);
   alt_fn!(Pj3, super::sig::LcdR4, 14);
   alt_fn!(Pj3, super::sig::Eventout, 15);

pin!(PJ4, Pj4, GPIOJ, Gpioj, _PJ4, GpioPin, _GPIOJ, 4);
   alt_fn!(Pj4, super::sig::LcdR5, 14);
   alt_fn!(Pj4, super::sig::Eventout, 15);

pin!(PJ5, Pj5, GPIOJ, Gpioj, _PJ5, GpioPin, _GPIOJ, 5);
   alt_fn!(Pj5, super::sig::LcdR6, 14);
   alt_fn!(Pj5, super::sig::Eventout, 15);

pin!(PJ6, Pj6, GPIOJ, Gpioj, _PJ6, GpioPin, _GPIOJ, 6);
   alt_fn!(Pj6, super::sig::LcdR7, 14);
   alt_fn!(Pj6, super::sig::Eventout, 15);

pin!(PJ7, Pj7, GPIOJ, Gpioj, _PJ7, GpioPin, _GPIOJ, 7);
   alt_fn!(Pj7, super::sig::LcdG0, 14);
   alt_fn!(Pj7, super::sig::Eventout, 15);

pin!(PJ8, Pj8, GPIOJ, Gpioj, _PJ8, GpioPin, _GPIOJ, 8);
   alt_fn!(Pj8, super::sig::LcdG1, 14);
   alt_fn!(Pj8, super::sig::Eventout, 15);

pin!(PJ9, Pj9, GPIOJ, Gpioj, _PJ9, GpioPin, _GPIOJ, 9);
   alt_fn!(Pj9, super::sig::LcdG2, 14);
   alt_fn!(Pj9, super::sig::Eventout, 15);

pin!(PJ10, Pj10, GPIOJ, Gpioj, _PJ10, GpioPin, _GPIOJ, 10);
   alt_fn!(Pj10, super::sig::LcdG3, 14);
   alt_fn!(Pj10, super::sig::Eventout, 15);

pin!(PJ11, Pj11, GPIOJ, Gpioj, _PJ11, GpioPin, _GPIOJ, 11);
   alt_fn!(Pj11, super::sig::LcdG4, 14);
   alt_fn!(Pj11, super::sig::Eventout, 15);

pin!(PJ12, Pj12, GPIOJ, Gpioj, _PJ12, GpioPin, _GPIOJ, 12);
   alt_fn!(Pj12, super::sig::LcdB0, 14);
   alt_fn!(Pj12, super::sig::Eventout, 15);

pin!(PJ13, Pj13, GPIOJ, Gpioj, _PJ13, GpioPin, _GPIOJ, 13);
   alt_fn!(Pj13, super::sig::LcdB1, 14);
   alt_fn!(Pj13, super::sig::Eventout, 15);

pin!(PJ14, Pj14, GPIOJ, Gpioj, _PJ14, GpioPin, _GPIOJ, 14);
   alt_fn!(Pj14, super::sig::LcdB2, 14);
   alt_fn!(Pj14, super::sig::Eventout, 15);

pin!(PJ15, Pj15, GPIOJ, Gpioj, _PJ15, GpioPin, _GPIOJ, 15);
   alt_fn!(Pj15, super::sig::LcdB3, 14);
   alt_fn!(Pj15, super::sig::Eventout, 15);

pin!(PK0, Pk0, GPIOK, Gpiok, _PK0, GpioPin, _GPIOK, 0);
   alt_fn!(Pk0, super::sig::LcdG5, 14);
   alt_fn!(Pk0, super::sig::Eventout, 15);

pin!(PK1, Pk1, GPIOK, Gpiok, _PK1, GpioPin, _GPIOK, 1);
   alt_fn!(Pk1, super::sig::LcdG6, 14);
   alt_fn!(Pk1, super::sig::Eventout, 15);

pin!(PK2, Pk2, GPIOK, Gpiok, _PK2, GpioPin, _GPIOK, 2);
   alt_fn!(Pk2, super::sig::LcdG7, 14);
   alt_fn!(Pk2, super::sig::Eventout, 15);

pin!(PK3, Pk3, GPIOK, Gpiok, _PK3, GpioPin, _GPIOK, 3);
   alt_fn!(Pk3, super::sig::LcdB4, 14);
   alt_fn!(Pk3, super::sig::Eventout, 15);

pin!(PK4, Pk4, GPIOK, Gpiok, _PK4, GpioPin, _GPIOK, 4);
   alt_fn!(Pk4, super::sig::LcdB5, 14);
   alt_fn!(Pk4, super::sig::Eventout, 15);

pin!(PK5, Pk5, GPIOK, Gpiok, _PK5, GpioPin, _GPIOK, 5);
   alt_fn!(Pk5, super::sig::LcdB6, 14);
   alt_fn!(Pk5, super::sig::Eventout, 15);

pin!(PK6, Pk6, GPIOK, Gpiok, _PK6, GpioPin, _GPIOK, 6);
   alt_fn!(Pk6, super::sig::LcdB7, 14);
   alt_fn!(Pk6, super::sig::Eventout, 15);

pin!(PK7, Pk7, GPIOK, Gpiok, _PK7, GpioPin, _GPIOK, 7);
   alt_fn!(Pk7, super::sig::LcdDe, 14);
   alt_fn!(Pk7, super::sig::Eventout, 15);

