#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::gpio::*;

periph!(GpioPeriph, GPIOA, Gpioa, 0x40020000);
periph!(GpioPeriph, GPIOB, Gpiob, 0x40020400);
periph!(GpioPeriph, GPIOC, Gpioc, 0x40020800);
periph!(GpioPeriph, GPIOD, Gpiod, 0x40020c00);
periph!(GpioPeriph, GPIOE, Gpioe, 0x40021000);
periph!(GpioPeriph, GPIOF, Gpiof, 0x40021400);
periph!(GpioPeriph, GPIOG, Gpiog, 0x40021800);
periph!(GpioPeriph, GPIOH, Gpioh, 0x40021c00);
periph!(GpioPeriph, GPIOI, Gpioi, 0x40022000);











pin!(PA0, Pa0, GPIOA, Gpioa, 0);
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

pin!(PA1, Pa1, GPIOA, Gpioa, 1);
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

pin!(PA2, Pa2, GPIOA, Gpioa, 2);
   alt_fn!(Pa2, super::sig::Adc1In2, 0);
   alt_fn!(Pa2, super::sig::Adc2In2, 0);
   alt_fn!(Pa2, super::sig::Adc3In2, 0);
   alt_fn!(Pa2, super::sig::Tim2Ch3, 1);
   alt_fn!(Pa2, super::sig::Tim5Ch3, 2);
   alt_fn!(Pa2, super::sig::Tim9Ch1, 3);
   alt_fn!(Pa2, super::sig::Usart2Tx, 7);
   alt_fn!(Pa2, super::sig::EthMdio, 11);
   alt_fn!(Pa2, super::sig::Eventout, 15);

pin!(PA3, Pa3, GPIOA, Gpioa, 3);
   alt_fn!(Pa3, super::sig::Adc1In3, 0);
   alt_fn!(Pa3, super::sig::Adc2In3, 0);
   alt_fn!(Pa3, super::sig::Adc3In3, 0);
   alt_fn!(Pa3, super::sig::Tim2Ch4, 1);
   alt_fn!(Pa3, super::sig::Tim5Ch4, 2);
   alt_fn!(Pa3, super::sig::Tim9Ch2, 3);
   alt_fn!(Pa3, super::sig::Usart2Rx, 7);
   alt_fn!(Pa3, super::sig::OtgHsUlpiD0, 10);
   alt_fn!(Pa3, super::sig::EthMiiCol, 11);
   alt_fn!(Pa3, super::sig::Eventout, 15);

pin!(PA4, Pa4, GPIOA, Gpioa, 4);
   alt_fn!(Pa4, super::sig::Adc1In4, 0);
   alt_fn!(Pa4, super::sig::Adc2In4, 0);
   alt_fn!(Pa4, super::sig::Spi1Nss, 5);
   alt_fn!(Pa4, super::sig::Spi3Nss, 6);
   alt_fn!(Pa4, super::sig::I2s3Ws, 6);
   alt_fn!(Pa4, super::sig::Usart2Ck, 7);
   alt_fn!(Pa4, super::sig::OtgHsSof, 12);
   alt_fn!(Pa4, super::sig::DcmiHsync, 13);
   alt_fn!(Pa4, super::sig::Eventout, 15);

pin!(PA5, Pa5, GPIOA, Gpioa, 5);
   alt_fn!(Pa5, super::sig::Adc1In5, 0);
   alt_fn!(Pa5, super::sig::Adc2In5, 0);
   alt_fn!(Pa5, super::sig::Tim2Ch1, 1);
   alt_fn!(Pa5, super::sig::Tim2Etr, 1);
   alt_fn!(Pa5, super::sig::Tim8Ch1n, 3);
   alt_fn!(Pa5, super::sig::Spi1Sck, 5);
   alt_fn!(Pa5, super::sig::OtgHsUlpiCk, 10);
   alt_fn!(Pa5, super::sig::Eventout, 15);

pin!(PA6, Pa6, GPIOA, Gpioa, 6);
   alt_fn!(Pa6, super::sig::Adc1In6, 0);
   alt_fn!(Pa6, super::sig::Adc2In6, 0);
   alt_fn!(Pa6, super::sig::Tim1Bkin, 1);
   alt_fn!(Pa6, super::sig::Tim3Ch1, 2);
   alt_fn!(Pa6, super::sig::Tim8Bkin, 3);
   alt_fn!(Pa6, super::sig::Spi1Miso, 5);
   alt_fn!(Pa6, super::sig::Tim13Ch1, 9);
   alt_fn!(Pa6, super::sig::DcmiPxclk, 13);
   alt_fn!(Pa6, super::sig::Eventout, 15);

pin!(PA7, Pa7, GPIOA, Gpioa, 7);
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

pin!(PA8, Pa8, GPIOA, Gpioa, 8);
   alt_fn!(Pa8, super::sig::Mco1, 0);
   alt_fn!(Pa8, super::sig::Tim1Ch1, 1);
   alt_fn!(Pa8, super::sig::I2c3Scl, 4);
   alt_fn!(Pa8, super::sig::Usart1Ck, 7);
   alt_fn!(Pa8, super::sig::OtgFsSof, 10);
   alt_fn!(Pa8, super::sig::Eventout, 15);

pin!(PA9, Pa9, GPIOA, Gpioa, 9);
   alt_fn!(Pa9, super::sig::Tim1Ch2, 1);
   alt_fn!(Pa9, super::sig::I2c3Smba, 4);
   alt_fn!(Pa9, super::sig::Usart1Tx, 7);
   alt_fn!(Pa9, super::sig::DcmiD0, 13);
   alt_fn!(Pa9, super::sig::Eventout, 15);

pin!(PA10, Pa10, GPIOA, Gpioa, 10);
   alt_fn!(Pa10, super::sig::Tim1Ch3, 1);
   alt_fn!(Pa10, super::sig::Usart1Rx, 7);
   alt_fn!(Pa10, super::sig::OtgFsId, 10);
   alt_fn!(Pa10, super::sig::DcmiD1, 13);
   alt_fn!(Pa10, super::sig::Eventout, 15);

pin!(PA11, Pa11, GPIOA, Gpioa, 11);
   alt_fn!(Pa11, super::sig::Tim1Ch4, 1);
   alt_fn!(Pa11, super::sig::Usart1Cts, 7);
   alt_fn!(Pa11, super::sig::Can1Rx, 9);
   alt_fn!(Pa11, super::sig::OtgFsDm, 10);
   alt_fn!(Pa11, super::sig::Eventout, 15);

pin!(PA12, Pa12, GPIOA, Gpioa, 12);
   alt_fn!(Pa12, super::sig::Tim1Etr, 1);
   alt_fn!(Pa12, super::sig::Usart1Rts, 7);
   alt_fn!(Pa12, super::sig::Can1Tx, 9);
   alt_fn!(Pa12, super::sig::OtgFsDp, 10);
   alt_fn!(Pa12, super::sig::Eventout, 15);

pin!(PA13, Pa13, GPIOA, Gpioa, 13);
   alt_fn!(Pa13, super::sig::Jtms, 0);
   alt_fn!(Pa13, super::sig::Swdio, 0);
   alt_fn!(Pa13, super::sig::Eventout, 15);

pin!(PA14, Pa14, GPIOA, Gpioa, 14);
   alt_fn!(Pa14, super::sig::Jtck, 0);
   alt_fn!(Pa14, super::sig::Swclk, 0);
   alt_fn!(Pa14, super::sig::Eventout, 15);

pin!(PA15, Pa15, GPIOA, Gpioa, 15);
   alt_fn!(Pa15, super::sig::Jtdi, 0);
   alt_fn!(Pa15, super::sig::Tim2Ch1, 1);
   alt_fn!(Pa15, super::sig::Tim2Etr, 1);
   alt_fn!(Pa15, super::sig::Spi1Nss, 5);
   alt_fn!(Pa15, super::sig::Spi3Nss, 6);
   alt_fn!(Pa15, super::sig::I2s3Ws, 6);
   alt_fn!(Pa15, super::sig::Eventout, 15);

pin!(PB0, Pb0, GPIOB, Gpiob, 0);
   alt_fn!(Pb0, super::sig::Adc1In8, 0);
   alt_fn!(Pb0, super::sig::Adc2In8, 0);
   alt_fn!(Pb0, super::sig::Tim1Ch2n, 1);
   alt_fn!(Pb0, super::sig::Tim3Ch3, 2);
   alt_fn!(Pb0, super::sig::Tim8Ch2n, 3);
   alt_fn!(Pb0, super::sig::OtgHsUlpiD1, 10);
   alt_fn!(Pb0, super::sig::EthMiiRxd2, 11);
   alt_fn!(Pb0, super::sig::Eventout, 15);

pin!(PB1, Pb1, GPIOB, Gpiob, 1);
   alt_fn!(Pb1, super::sig::Adc1In9, 0);
   alt_fn!(Pb1, super::sig::Adc2In9, 0);
   alt_fn!(Pb1, super::sig::Tim1Ch3n, 1);
   alt_fn!(Pb1, super::sig::Tim3Ch4, 2);
   alt_fn!(Pb1, super::sig::Tim8Ch3n, 3);
   alt_fn!(Pb1, super::sig::OtgHsUlpiD2, 10);
   alt_fn!(Pb1, super::sig::EthMiiRxd3, 11);
   alt_fn!(Pb1, super::sig::Eventout, 15);

pin!(PB2, Pb2, GPIOB, Gpiob, 2);
   alt_fn!(Pb2, super::sig::Eventout, 15);

pin!(PB3, Pb3, GPIOB, Gpiob, 3);
   alt_fn!(Pb3, super::sig::Jtdo, 0);
   alt_fn!(Pb3, super::sig::Traceswo, 0);
   alt_fn!(Pb3, super::sig::Tim2Ch2, 1);
   alt_fn!(Pb3, super::sig::Spi1Sck, 5);
   alt_fn!(Pb3, super::sig::Spi3Sck, 6);
   alt_fn!(Pb3, super::sig::I2s3Ck, 6);
   alt_fn!(Pb3, super::sig::Eventout, 15);

pin!(PB4, Pb4, GPIOB, Gpiob, 4);
   alt_fn!(Pb4, super::sig::Njtrst, 0);
   alt_fn!(Pb4, super::sig::Tim3Ch1, 2);
   alt_fn!(Pb4, super::sig::Spi1Miso, 5);
   alt_fn!(Pb4, super::sig::Spi3Miso, 6);
   alt_fn!(Pb4, super::sig::I2s3extSd, 7);
   alt_fn!(Pb4, super::sig::Eventout, 15);

pin!(PB5, Pb5, GPIOB, Gpiob, 5);
   alt_fn!(Pb5, super::sig::Tim3Ch2, 2);
   alt_fn!(Pb5, super::sig::I2c1Smba, 4);
   alt_fn!(Pb5, super::sig::Spi1Mosi, 5);
   alt_fn!(Pb5, super::sig::Spi3Mosi, 6);
   alt_fn!(Pb5, super::sig::I2s3Sd, 6);
   alt_fn!(Pb5, super::sig::Can2Rx, 9);
   alt_fn!(Pb5, super::sig::OtgHsUlpiD7, 10);
   alt_fn!(Pb5, super::sig::EthPpsOut, 11);
   alt_fn!(Pb5, super::sig::DcmiD10, 13);
   alt_fn!(Pb5, super::sig::Eventout, 15);

pin!(PB6, Pb6, GPIOB, Gpiob, 6);
   alt_fn!(Pb6, super::sig::Tim4Ch1, 2);
   alt_fn!(Pb6, super::sig::I2c1Scl, 4);
   alt_fn!(Pb6, super::sig::Usart1Tx, 7);
   alt_fn!(Pb6, super::sig::Can2Tx, 9);
   alt_fn!(Pb6, super::sig::DcmiD5, 13);
   alt_fn!(Pb6, super::sig::Eventout, 15);

pin!(PB7, Pb7, GPIOB, Gpiob, 7);
   alt_fn!(Pb7, super::sig::Tim4Ch2, 2);
   alt_fn!(Pb7, super::sig::I2c1Sda, 4);
   alt_fn!(Pb7, super::sig::Usart1Rx, 7);
   alt_fn!(Pb7, super::sig::FsmcNl, 12);
   alt_fn!(Pb7, super::sig::DcmiVsync, 13);
   alt_fn!(Pb7, super::sig::Eventout, 15);

pin!(PB8, Pb8, GPIOB, Gpiob, 8);
   alt_fn!(Pb8, super::sig::Tim4Ch3, 2);
   alt_fn!(Pb8, super::sig::Tim10Ch1, 3);
   alt_fn!(Pb8, super::sig::I2c1Scl, 4);
   alt_fn!(Pb8, super::sig::Can1Rx, 9);
   alt_fn!(Pb8, super::sig::EthMiiTxd3, 11);
   alt_fn!(Pb8, super::sig::SdioD4, 12);
   alt_fn!(Pb8, super::sig::DcmiD6, 13);
   alt_fn!(Pb8, super::sig::Eventout, 15);

pin!(PB9, Pb9, GPIOB, Gpiob, 9);
   alt_fn!(Pb9, super::sig::Tim4Ch4, 2);
   alt_fn!(Pb9, super::sig::Tim11Ch1, 3);
   alt_fn!(Pb9, super::sig::I2c1Sda, 4);
   alt_fn!(Pb9, super::sig::Spi2Nss, 5);
   alt_fn!(Pb9, super::sig::I2s2Ws, 5);
   alt_fn!(Pb9, super::sig::Can1Tx, 9);
   alt_fn!(Pb9, super::sig::SdioD5, 12);
   alt_fn!(Pb9, super::sig::DcmiD7, 13);
   alt_fn!(Pb9, super::sig::Eventout, 15);

pin!(PB10, Pb10, GPIOB, Gpiob, 10);
   alt_fn!(Pb10, super::sig::Tim2Ch3, 1);
   alt_fn!(Pb10, super::sig::I2c2Scl, 4);
   alt_fn!(Pb10, super::sig::Spi2Sck, 5);
   alt_fn!(Pb10, super::sig::I2s2Ck, 5);
   alt_fn!(Pb10, super::sig::Usart3Tx, 7);
   alt_fn!(Pb10, super::sig::OtgHsUlpiD3, 10);
   alt_fn!(Pb10, super::sig::EthMiiRxEr, 11);
   alt_fn!(Pb10, super::sig::Eventout, 15);

pin!(PB11, Pb11, GPIOB, Gpiob, 11);
   alt_fn!(Pb11, super::sig::Tim2Ch4, 1);
   alt_fn!(Pb11, super::sig::I2c2Sda, 4);
   alt_fn!(Pb11, super::sig::Usart3Rx, 7);
   alt_fn!(Pb11, super::sig::OtgHsUlpiD4, 10);
   alt_fn!(Pb11, super::sig::EthMiiTxEn, 11);
   alt_fn!(Pb11, super::sig::EthRmiiTxEn, 11);
   alt_fn!(Pb11, super::sig::Eventout, 15);

pin!(PB12, Pb12, GPIOB, Gpiob, 12);
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

pin!(PB13, Pb13, GPIOB, Gpiob, 13);
   alt_fn!(Pb13, super::sig::Tim1Ch1n, 1);
   alt_fn!(Pb13, super::sig::Spi2Sck, 5);
   alt_fn!(Pb13, super::sig::I2s2Ck, 5);
   alt_fn!(Pb13, super::sig::Usart3Cts, 7);
   alt_fn!(Pb13, super::sig::Can2Tx, 9);
   alt_fn!(Pb13, super::sig::OtgHsUlpiD6, 10);
   alt_fn!(Pb13, super::sig::EthMiiTxd1, 11);
   alt_fn!(Pb13, super::sig::EthRmiiTxd1, 11);
   alt_fn!(Pb13, super::sig::Eventout, 15);

pin!(PB14, Pb14, GPIOB, Gpiob, 14);
   alt_fn!(Pb14, super::sig::Tim1Ch2n, 1);
   alt_fn!(Pb14, super::sig::Tim8Ch2n, 3);
   alt_fn!(Pb14, super::sig::Spi2Miso, 5);
   alt_fn!(Pb14, super::sig::I2s2extSd, 6);
   alt_fn!(Pb14, super::sig::Usart3Rts, 7);
   alt_fn!(Pb14, super::sig::Tim12Ch1, 9);
   alt_fn!(Pb14, super::sig::OtgHsDm, 12);
   alt_fn!(Pb14, super::sig::Eventout, 15);

pin!(PB15, Pb15, GPIOB, Gpiob, 15);
   alt_fn!(Pb15, super::sig::RtcRefin, 0);
   alt_fn!(Pb15, super::sig::Tim1Ch3n, 1);
   alt_fn!(Pb15, super::sig::Tim8Ch3n, 3);
   alt_fn!(Pb15, super::sig::Spi2Mosi, 5);
   alt_fn!(Pb15, super::sig::I2s2Sd, 5);
   alt_fn!(Pb15, super::sig::Tim12Ch2, 9);
   alt_fn!(Pb15, super::sig::OtgHsDp, 12);
   alt_fn!(Pb15, super::sig::Eventout, 15);

pin!(PC0, Pc0, GPIOC, Gpioc, 0);
   alt_fn!(Pc0, super::sig::Adc1In10, 0);
   alt_fn!(Pc0, super::sig::Adc2In10, 0);
   alt_fn!(Pc0, super::sig::Adc3In10, 0);
   alt_fn!(Pc0, super::sig::OtgHsUlpiStp, 10);
   alt_fn!(Pc0, super::sig::Eventout, 15);

pin!(PC1, Pc1, GPIOC, Gpioc, 1);
   alt_fn!(Pc1, super::sig::Adc1In11, 0);
   alt_fn!(Pc1, super::sig::Adc2In11, 0);
   alt_fn!(Pc1, super::sig::Adc3In11, 0);
   alt_fn!(Pc1, super::sig::EthMdc, 11);
   alt_fn!(Pc1, super::sig::Eventout, 15);

pin!(PC2, Pc2, GPIOC, Gpioc, 2);
   alt_fn!(Pc2, super::sig::Adc1In12, 0);
   alt_fn!(Pc2, super::sig::Adc2In12, 0);
   alt_fn!(Pc2, super::sig::Adc3In12, 0);
   alt_fn!(Pc2, super::sig::Spi2Miso, 5);
   alt_fn!(Pc2, super::sig::I2s2extSd, 6);
   alt_fn!(Pc2, super::sig::OtgHsUlpiDir, 10);
   alt_fn!(Pc2, super::sig::EthMiiTxd2, 11);
   alt_fn!(Pc2, super::sig::Eventout, 15);

pin!(PC3, Pc3, GPIOC, Gpioc, 3);
   alt_fn!(Pc3, super::sig::Adc1In13, 0);
   alt_fn!(Pc3, super::sig::Adc2In13, 0);
   alt_fn!(Pc3, super::sig::Adc3In13, 0);
   alt_fn!(Pc3, super::sig::Spi2Mosi, 5);
   alt_fn!(Pc3, super::sig::I2s2Sd, 5);
   alt_fn!(Pc3, super::sig::OtgHsUlpiNxt, 10);
   alt_fn!(Pc3, super::sig::EthMiiTxClk, 11);
   alt_fn!(Pc3, super::sig::Eventout, 15);

pin!(PC4, Pc4, GPIOC, Gpioc, 4);
   alt_fn!(Pc4, super::sig::Adc1In14, 0);
   alt_fn!(Pc4, super::sig::Adc2In14, 0);
   alt_fn!(Pc4, super::sig::EthMiiRxd0, 11);
   alt_fn!(Pc4, super::sig::EthRmiiRxd0, 11);
   alt_fn!(Pc4, super::sig::Eventout, 15);

pin!(PC5, Pc5, GPIOC, Gpioc, 5);
   alt_fn!(Pc5, super::sig::Adc1In15, 0);
   alt_fn!(Pc5, super::sig::Adc2In15, 0);
   alt_fn!(Pc5, super::sig::EthMiiRxd1, 11);
   alt_fn!(Pc5, super::sig::EthRmiiRxd1, 11);
   alt_fn!(Pc5, super::sig::Eventout, 15);

pin!(PC6, Pc6, GPIOC, Gpioc, 6);
   alt_fn!(Pc6, super::sig::Tim3Ch1, 2);
   alt_fn!(Pc6, super::sig::Tim8Ch1, 3);
   alt_fn!(Pc6, super::sig::I2s2Mck, 5);
   alt_fn!(Pc6, super::sig::Usart6Tx, 8);
   alt_fn!(Pc6, super::sig::SdioD6, 12);
   alt_fn!(Pc6, super::sig::DcmiD0, 13);
   alt_fn!(Pc6, super::sig::Eventout, 15);

pin!(PC7, Pc7, GPIOC, Gpioc, 7);
   alt_fn!(Pc7, super::sig::Tim3Ch2, 2);
   alt_fn!(Pc7, super::sig::Tim8Ch2, 3);
   alt_fn!(Pc7, super::sig::I2s3Mck, 6);
   alt_fn!(Pc7, super::sig::Usart6Rx, 8);
   alt_fn!(Pc7, super::sig::SdioD7, 12);
   alt_fn!(Pc7, super::sig::DcmiD1, 13);
   alt_fn!(Pc7, super::sig::Eventout, 15);

pin!(PC8, Pc8, GPIOC, Gpioc, 8);
   alt_fn!(Pc8, super::sig::Tim3Ch3, 2);
   alt_fn!(Pc8, super::sig::Tim8Ch3, 3);
   alt_fn!(Pc8, super::sig::Usart6Ck, 8);
   alt_fn!(Pc8, super::sig::SdioD0, 12);
   alt_fn!(Pc8, super::sig::DcmiD2, 13);
   alt_fn!(Pc8, super::sig::Eventout, 15);

pin!(PC9, Pc9, GPIOC, Gpioc, 9);
   alt_fn!(Pc9, super::sig::Mco2, 0);
   alt_fn!(Pc9, super::sig::Tim3Ch4, 2);
   alt_fn!(Pc9, super::sig::Tim8Ch4, 3);
   alt_fn!(Pc9, super::sig::I2c3Sda, 4);
   alt_fn!(Pc9, super::sig::I2sCkin, 5);
   alt_fn!(Pc9, super::sig::SdioD1, 12);
   alt_fn!(Pc9, super::sig::DcmiD3, 13);
   alt_fn!(Pc9, super::sig::Eventout, 15);

pin!(PC10, Pc10, GPIOC, Gpioc, 10);
   alt_fn!(Pc10, super::sig::Spi3Sck, 6);
   alt_fn!(Pc10, super::sig::I2s3Ck, 6);
   alt_fn!(Pc10, super::sig::Usart3Tx, 7);
   alt_fn!(Pc10, super::sig::Uart4Tx, 8);
   alt_fn!(Pc10, super::sig::SdioD2, 12);
   alt_fn!(Pc10, super::sig::DcmiD8, 13);
   alt_fn!(Pc10, super::sig::Eventout, 15);

pin!(PC11, Pc11, GPIOC, Gpioc, 11);
   alt_fn!(Pc11, super::sig::I2s3extSd, 5);
   alt_fn!(Pc11, super::sig::Spi3Miso, 6);
   alt_fn!(Pc11, super::sig::Usart3Rx, 7);
   alt_fn!(Pc11, super::sig::Uart4Rx, 8);
   alt_fn!(Pc11, super::sig::SdioD3, 12);
   alt_fn!(Pc11, super::sig::DcmiD4, 13);
   alt_fn!(Pc11, super::sig::Eventout, 15);

pin!(PC12, Pc12, GPIOC, Gpioc, 12);
   alt_fn!(Pc12, super::sig::Spi3Mosi, 6);
   alt_fn!(Pc12, super::sig::I2s3Sd, 6);
   alt_fn!(Pc12, super::sig::Usart3Ck, 7);
   alt_fn!(Pc12, super::sig::Uart5Tx, 8);
   alt_fn!(Pc12, super::sig::SdioCk, 12);
   alt_fn!(Pc12, super::sig::DcmiD9, 13);
   alt_fn!(Pc12, super::sig::Eventout, 15);

pin!(PC13, Pc13, GPIOC, Gpioc, 13);
   alt_fn!(Pc13, super::sig::Eventout, 15);

pin!(PC14, Pc14, GPIOC, Gpioc, 14);
   alt_fn!(Pc14, super::sig::Eventout, 15);

pin!(PC15, Pc15, GPIOC, Gpioc, 15);
   alt_fn!(Pc15, super::sig::Eventout, 15);

pin!(PD0, Pd0, GPIOD, Gpiod, 0);
   alt_fn!(Pd0, super::sig::Can1Rx, 9);
   alt_fn!(Pd0, super::sig::FsmcD2, 12);
   alt_fn!(Pd0, super::sig::Eventout, 15);

pin!(PD1, Pd1, GPIOD, Gpiod, 1);
   alt_fn!(Pd1, super::sig::Can1Tx, 9);
   alt_fn!(Pd1, super::sig::FsmcD3, 12);
   alt_fn!(Pd1, super::sig::Eventout, 15);

pin!(PD2, Pd2, GPIOD, Gpiod, 2);
   alt_fn!(Pd2, super::sig::Tim3Etr, 2);
   alt_fn!(Pd2, super::sig::Uart5Rx, 8);
   alt_fn!(Pd2, super::sig::SdioCmd, 12);
   alt_fn!(Pd2, super::sig::DcmiD11, 13);
   alt_fn!(Pd2, super::sig::Eventout, 15);

pin!(PD3, Pd3, GPIOD, Gpiod, 3);
   alt_fn!(Pd3, super::sig::Usart2Cts, 7);
   alt_fn!(Pd3, super::sig::FsmcClk, 12);
   alt_fn!(Pd3, super::sig::Eventout, 15);

pin!(PD4, Pd4, GPIOD, Gpiod, 4);
   alt_fn!(Pd4, super::sig::Usart2Rts, 7);
   alt_fn!(Pd4, super::sig::FsmcNoe, 12);
   alt_fn!(Pd4, super::sig::Eventout, 15);

pin!(PD5, Pd5, GPIOD, Gpiod, 5);
   alt_fn!(Pd5, super::sig::Usart2Tx, 7);
   alt_fn!(Pd5, super::sig::FsmcNwe, 12);
   alt_fn!(Pd5, super::sig::Eventout, 15);

pin!(PD6, Pd6, GPIOD, Gpiod, 6);
   alt_fn!(Pd6, super::sig::Usart2Rx, 7);
   alt_fn!(Pd6, super::sig::FsmcNwait, 12);
   alt_fn!(Pd6, super::sig::Eventout, 15);

pin!(PD7, Pd7, GPIOD, Gpiod, 7);
   alt_fn!(Pd7, super::sig::Usart2Ck, 7);
   alt_fn!(Pd7, super::sig::FsmcNe1, 12);
   alt_fn!(Pd7, super::sig::FsmcNce2, 12);
   alt_fn!(Pd7, super::sig::Eventout, 15);

pin!(PD8, Pd8, GPIOD, Gpiod, 8);
   alt_fn!(Pd8, super::sig::Usart3Tx, 7);
   alt_fn!(Pd8, super::sig::FsmcD13, 12);
   alt_fn!(Pd8, super::sig::Eventout, 15);

pin!(PD9, Pd9, GPIOD, Gpiod, 9);
   alt_fn!(Pd9, super::sig::Usart3Rx, 7);
   alt_fn!(Pd9, super::sig::FsmcD14, 12);
   alt_fn!(Pd9, super::sig::Eventout, 15);

pin!(PD10, Pd10, GPIOD, Gpiod, 10);
   alt_fn!(Pd10, super::sig::Usart3Ck, 7);
   alt_fn!(Pd10, super::sig::FsmcD15, 12);
   alt_fn!(Pd10, super::sig::Eventout, 15);

pin!(PD11, Pd11, GPIOD, Gpiod, 11);
   alt_fn!(Pd11, super::sig::Usart3Cts, 7);
   alt_fn!(Pd11, super::sig::FsmcA16, 12);
   alt_fn!(Pd11, super::sig::Eventout, 15);

pin!(PD12, Pd12, GPIOD, Gpiod, 12);
   alt_fn!(Pd12, super::sig::Tim4Ch1, 2);
   alt_fn!(Pd12, super::sig::Usart3Rts, 7);
   alt_fn!(Pd12, super::sig::FsmcA17, 12);
   alt_fn!(Pd12, super::sig::Eventout, 15);

pin!(PD13, Pd13, GPIOD, Gpiod, 13);
   alt_fn!(Pd13, super::sig::Tim4Ch2, 2);
   alt_fn!(Pd13, super::sig::FsmcA18, 12);
   alt_fn!(Pd13, super::sig::Eventout, 15);

pin!(PD14, Pd14, GPIOD, Gpiod, 14);
   alt_fn!(Pd14, super::sig::Tim4Ch3, 2);
   alt_fn!(Pd14, super::sig::FsmcD0, 12);
   alt_fn!(Pd14, super::sig::Eventout, 15);

pin!(PD15, Pd15, GPIOD, Gpiod, 15);
   alt_fn!(Pd15, super::sig::Tim4Ch4, 2);
   alt_fn!(Pd15, super::sig::FsmcD1, 12);
   alt_fn!(Pd15, super::sig::Eventout, 15);

pin!(PE0, Pe0, GPIOE, Gpioe, 0);
   alt_fn!(Pe0, super::sig::Tim4Etr, 2);
   alt_fn!(Pe0, super::sig::FsmcNbl0, 12);
   alt_fn!(Pe0, super::sig::DcmiD2, 13);
   alt_fn!(Pe0, super::sig::Eventout, 15);

pin!(PE1, Pe1, GPIOE, Gpioe, 1);
   alt_fn!(Pe1, super::sig::FsmcNbl1, 12);
   alt_fn!(Pe1, super::sig::DcmiD3, 13);
   alt_fn!(Pe1, super::sig::Eventout, 15);

pin!(PE2, Pe2, GPIOE, Gpioe, 2);
   alt_fn!(Pe2, super::sig::Traceclk, 0);
   alt_fn!(Pe2, super::sig::EthMiiTxd3, 11);
   alt_fn!(Pe2, super::sig::FsmcA23, 12);
   alt_fn!(Pe2, super::sig::Eventout, 15);

pin!(PE3, Pe3, GPIOE, Gpioe, 3);
   alt_fn!(Pe3, super::sig::Traced0, 0);
   alt_fn!(Pe3, super::sig::FsmcA19, 12);
   alt_fn!(Pe3, super::sig::Eventout, 15);

pin!(PE4, Pe4, GPIOE, Gpioe, 4);
   alt_fn!(Pe4, super::sig::Traced1, 0);
   alt_fn!(Pe4, super::sig::FsmcA20, 12);
   alt_fn!(Pe4, super::sig::DcmiD4, 13);
   alt_fn!(Pe4, super::sig::Eventout, 15);

pin!(PE5, Pe5, GPIOE, Gpioe, 5);
   alt_fn!(Pe5, super::sig::Traced2, 0);
   alt_fn!(Pe5, super::sig::Tim9Ch1, 3);
   alt_fn!(Pe5, super::sig::FsmcA21, 12);
   alt_fn!(Pe5, super::sig::DcmiD6, 13);
   alt_fn!(Pe5, super::sig::Eventout, 15);

pin!(PE6, Pe6, GPIOE, Gpioe, 6);
   alt_fn!(Pe6, super::sig::Traced3, 0);
   alt_fn!(Pe6, super::sig::Tim9Ch2, 3);
   alt_fn!(Pe6, super::sig::FsmcA22, 12);
   alt_fn!(Pe6, super::sig::DcmiD7, 13);
   alt_fn!(Pe6, super::sig::Eventout, 15);

pin!(PE7, Pe7, GPIOE, Gpioe, 7);
   alt_fn!(Pe7, super::sig::Tim1Etr, 1);
   alt_fn!(Pe7, super::sig::FsmcD4, 12);
   alt_fn!(Pe7, super::sig::Eventout, 15);

pin!(PE8, Pe8, GPIOE, Gpioe, 8);
   alt_fn!(Pe8, super::sig::Tim1Ch1n, 1);
   alt_fn!(Pe8, super::sig::FsmcD5, 12);
   alt_fn!(Pe8, super::sig::Eventout, 15);

pin!(PE9, Pe9, GPIOE, Gpioe, 9);
   alt_fn!(Pe9, super::sig::Tim1Ch1, 1);
   alt_fn!(Pe9, super::sig::FsmcD6, 12);
   alt_fn!(Pe9, super::sig::Eventout, 15);

pin!(PE10, Pe10, GPIOE, Gpioe, 10);
   alt_fn!(Pe10, super::sig::Tim1Ch2n, 1);
   alt_fn!(Pe10, super::sig::FsmcD7, 12);
   alt_fn!(Pe10, super::sig::Eventout, 15);

pin!(PE11, Pe11, GPIOE, Gpioe, 11);
   alt_fn!(Pe11, super::sig::Tim1Ch2, 1);
   alt_fn!(Pe11, super::sig::FsmcD8, 12);
   alt_fn!(Pe11, super::sig::Eventout, 15);

pin!(PE12, Pe12, GPIOE, Gpioe, 12);
   alt_fn!(Pe12, super::sig::Tim1Ch3n, 1);
   alt_fn!(Pe12, super::sig::FsmcD9, 12);
   alt_fn!(Pe12, super::sig::Eventout, 15);

pin!(PE13, Pe13, GPIOE, Gpioe, 13);
   alt_fn!(Pe13, super::sig::Tim1Ch3, 1);
   alt_fn!(Pe13, super::sig::FsmcD10, 12);
   alt_fn!(Pe13, super::sig::Eventout, 15);

pin!(PE14, Pe14, GPIOE, Gpioe, 14);
   alt_fn!(Pe14, super::sig::Tim1Ch4, 1);
   alt_fn!(Pe14, super::sig::FsmcD11, 12);
   alt_fn!(Pe14, super::sig::Eventout, 15);

pin!(PE15, Pe15, GPIOE, Gpioe, 15);
   alt_fn!(Pe15, super::sig::Tim1Bkin, 1);
   alt_fn!(Pe15, super::sig::FsmcD12, 12);
   alt_fn!(Pe15, super::sig::Eventout, 15);

pin!(PF0, Pf0, GPIOF, Gpiof, 0);
   alt_fn!(Pf0, super::sig::I2c2Sda, 4);
   alt_fn!(Pf0, super::sig::FsmcA0, 12);
   alt_fn!(Pf0, super::sig::Eventout, 15);

pin!(PF1, Pf1, GPIOF, Gpiof, 1);
   alt_fn!(Pf1, super::sig::I2c2Scl, 4);
   alt_fn!(Pf1, super::sig::FsmcA1, 12);
   alt_fn!(Pf1, super::sig::Eventout, 15);

pin!(PF2, Pf2, GPIOF, Gpiof, 2);
   alt_fn!(Pf2, super::sig::I2c2Smba, 4);
   alt_fn!(Pf2, super::sig::FsmcA2, 12);
   alt_fn!(Pf2, super::sig::Eventout, 15);

pin!(PF3, Pf3, GPIOF, Gpiof, 3);
   alt_fn!(Pf3, super::sig::Adc3In9, 0);
   alt_fn!(Pf3, super::sig::FsmcA3, 12);
   alt_fn!(Pf3, super::sig::Eventout, 15);

pin!(PF4, Pf4, GPIOF, Gpiof, 4);
   alt_fn!(Pf4, super::sig::Adc3In14, 0);
   alt_fn!(Pf4, super::sig::FsmcA4, 12);
   alt_fn!(Pf4, super::sig::Eventout, 15);

pin!(PF5, Pf5, GPIOF, Gpiof, 5);
   alt_fn!(Pf5, super::sig::Adc3In15, 0);
   alt_fn!(Pf5, super::sig::FsmcA5, 12);
   alt_fn!(Pf5, super::sig::Eventout, 15);

pin!(PF6, Pf6, GPIOF, Gpiof, 6);
   alt_fn!(Pf6, super::sig::Adc3In4, 0);
   alt_fn!(Pf6, super::sig::Tim10Ch1, 3);
   alt_fn!(Pf6, super::sig::FsmcNiord, 12);
   alt_fn!(Pf6, super::sig::Eventout, 15);

pin!(PF7, Pf7, GPIOF, Gpiof, 7);
   alt_fn!(Pf7, super::sig::Adc3In5, 0);
   alt_fn!(Pf7, super::sig::Tim11Ch1, 3);
   alt_fn!(Pf7, super::sig::FsmcNreg, 12);
   alt_fn!(Pf7, super::sig::Eventout, 15);

pin!(PF8, Pf8, GPIOF, Gpiof, 8);
   alt_fn!(Pf8, super::sig::Adc3In6, 0);
   alt_fn!(Pf8, super::sig::Tim13Ch1, 9);
   alt_fn!(Pf8, super::sig::FsmcNiowr, 12);
   alt_fn!(Pf8, super::sig::Eventout, 15);

pin!(PF9, Pf9, GPIOF, Gpiof, 9);
   alt_fn!(Pf9, super::sig::Adc3In7, 0);
   alt_fn!(Pf9, super::sig::Tim14Ch1, 9);
   alt_fn!(Pf9, super::sig::FsmcCd, 12);
   alt_fn!(Pf9, super::sig::Eventout, 15);

pin!(PF10, Pf10, GPIOF, Gpiof, 10);
   alt_fn!(Pf10, super::sig::Adc3In8, 0);
   alt_fn!(Pf10, super::sig::FsmcIntr, 12);
   alt_fn!(Pf10, super::sig::Eventout, 15);

pin!(PF11, Pf11, GPIOF, Gpiof, 11);
   alt_fn!(Pf11, super::sig::DcmiD12, 13);
   alt_fn!(Pf11, super::sig::Eventout, 15);

pin!(PF12, Pf12, GPIOF, Gpiof, 12);
   alt_fn!(Pf12, super::sig::FsmcA6, 12);
   alt_fn!(Pf12, super::sig::Eventout, 15);

pin!(PF13, Pf13, GPIOF, Gpiof, 13);
   alt_fn!(Pf13, super::sig::FsmcA7, 12);
   alt_fn!(Pf13, super::sig::Eventout, 15);

pin!(PF14, Pf14, GPIOF, Gpiof, 14);
   alt_fn!(Pf14, super::sig::FsmcA8, 12);
   alt_fn!(Pf14, super::sig::Eventout, 15);

pin!(PF15, Pf15, GPIOF, Gpiof, 15);
   alt_fn!(Pf15, super::sig::FsmcA9, 12);
   alt_fn!(Pf15, super::sig::Eventout, 15);

pin!(PG0, Pg0, GPIOG, Gpiog, 0);
   alt_fn!(Pg0, super::sig::FsmcA10, 12);
   alt_fn!(Pg0, super::sig::Eventout, 15);

pin!(PG1, Pg1, GPIOG, Gpiog, 1);
   alt_fn!(Pg1, super::sig::FsmcA11, 12);
   alt_fn!(Pg1, super::sig::Eventout, 15);

pin!(PG2, Pg2, GPIOG, Gpiog, 2);
   alt_fn!(Pg2, super::sig::FsmcA12, 12);
   alt_fn!(Pg2, super::sig::Eventout, 15);

pin!(PG3, Pg3, GPIOG, Gpiog, 3);
   alt_fn!(Pg3, super::sig::FsmcA13, 12);
   alt_fn!(Pg3, super::sig::Eventout, 15);

pin!(PG4, Pg4, GPIOG, Gpiog, 4);
   alt_fn!(Pg4, super::sig::FsmcA14, 12);
   alt_fn!(Pg4, super::sig::Eventout, 15);

pin!(PG5, Pg5, GPIOG, Gpiog, 5);
   alt_fn!(Pg5, super::sig::FsmcA15, 12);
   alt_fn!(Pg5, super::sig::Eventout, 15);

pin!(PG6, Pg6, GPIOG, Gpiog, 6);
   alt_fn!(Pg6, super::sig::FsmcInt2, 12);
   alt_fn!(Pg6, super::sig::Eventout, 15);

pin!(PG7, Pg7, GPIOG, Gpiog, 7);
   alt_fn!(Pg7, super::sig::Usart6Ck, 8);
   alt_fn!(Pg7, super::sig::FsmcInt3, 12);
   alt_fn!(Pg7, super::sig::Eventout, 15);

pin!(PG8, Pg8, GPIOG, Gpiog, 8);
   alt_fn!(Pg8, super::sig::Usart6Rts, 8);
   alt_fn!(Pg8, super::sig::EthPpsOut, 11);
   alt_fn!(Pg8, super::sig::Eventout, 15);

pin!(PG9, Pg9, GPIOG, Gpiog, 9);
   alt_fn!(Pg9, super::sig::Usart6Rx, 8);
   alt_fn!(Pg9, super::sig::FsmcNe2, 12);
   alt_fn!(Pg9, super::sig::FsmcNce3, 12);
   alt_fn!(Pg9, super::sig::Eventout, 15);

pin!(PG10, Pg10, GPIOG, Gpiog, 10);
   alt_fn!(Pg10, super::sig::FsmcNce41, 12);
   alt_fn!(Pg10, super::sig::FsmcNe3, 12);
   alt_fn!(Pg10, super::sig::Eventout, 15);

pin!(PG11, Pg11, GPIOG, Gpiog, 11);
   alt_fn!(Pg11, super::sig::EthMiiTxEn, 11);
   alt_fn!(Pg11, super::sig::EthRmiiTxEn, 11);
   alt_fn!(Pg11, super::sig::FsmcNce42, 12);
   alt_fn!(Pg11, super::sig::Eventout, 15);

pin!(PG12, Pg12, GPIOG, Gpiog, 12);
   alt_fn!(Pg12, super::sig::Usart6Rts, 8);
   alt_fn!(Pg12, super::sig::FsmcNe4, 12);
   alt_fn!(Pg12, super::sig::Eventout, 15);

pin!(PG13, Pg13, GPIOG, Gpiog, 13);
   alt_fn!(Pg13, super::sig::Usart6Cts, 8);
   alt_fn!(Pg13, super::sig::EthMiiTxd0, 11);
   alt_fn!(Pg13, super::sig::EthRmiiTxd0, 11);
   alt_fn!(Pg13, super::sig::FsmcA24, 12);
   alt_fn!(Pg13, super::sig::Eventout, 15);

pin!(PG14, Pg14, GPIOG, Gpiog, 14);
   alt_fn!(Pg14, super::sig::Usart6Tx, 8);
   alt_fn!(Pg14, super::sig::EthMiiTxd1, 11);
   alt_fn!(Pg14, super::sig::EthRmiiTxd1, 11);
   alt_fn!(Pg14, super::sig::FsmcA25, 12);
   alt_fn!(Pg14, super::sig::Eventout, 15);

pin!(PG15, Pg15, GPIOG, Gpiog, 15);
   alt_fn!(Pg15, super::sig::Usart6Cts, 8);
   alt_fn!(Pg15, super::sig::DcmiD13, 13);
   alt_fn!(Pg15, super::sig::Eventout, 15);

pin!(PH0, Ph0, GPIOH, Gpioh, 0);
   alt_fn!(Ph0, super::sig::Eventout, 15);

pin!(PH1, Ph1, GPIOH, Gpioh, 1);
   alt_fn!(Ph1, super::sig::Eventout, 15);

pin!(PH2, Ph2, GPIOH, Gpioh, 2);
   alt_fn!(Ph2, super::sig::EthMiiCrs, 11);
   alt_fn!(Ph2, super::sig::Eventout, 15);

pin!(PH3, Ph3, GPIOH, Gpioh, 3);
   alt_fn!(Ph3, super::sig::EthMiiCol, 11);
   alt_fn!(Ph3, super::sig::Eventout, 15);

pin!(PH4, Ph4, GPIOH, Gpioh, 4);
   alt_fn!(Ph4, super::sig::I2c2Scl, 4);
   alt_fn!(Ph4, super::sig::OtgHsUlpiNxt, 10);
   alt_fn!(Ph4, super::sig::Eventout, 15);

pin!(PH5, Ph5, GPIOH, Gpioh, 5);
   alt_fn!(Ph5, super::sig::I2c2Sda, 4);
   alt_fn!(Ph5, super::sig::Eventout, 15);

pin!(PH6, Ph6, GPIOH, Gpioh, 6);
   alt_fn!(Ph6, super::sig::I2c2Smba, 4);
   alt_fn!(Ph6, super::sig::Tim12Ch1, 9);
   alt_fn!(Ph6, super::sig::EthMiiRxd2, 11);
   alt_fn!(Ph6, super::sig::Eventout, 15);

pin!(PH7, Ph7, GPIOH, Gpioh, 7);
   alt_fn!(Ph7, super::sig::I2c3Scl, 4);
   alt_fn!(Ph7, super::sig::EthMiiRxd3, 11);
   alt_fn!(Ph7, super::sig::Eventout, 15);

pin!(PH8, Ph8, GPIOH, Gpioh, 8);
   alt_fn!(Ph8, super::sig::I2c3Sda, 4);
   alt_fn!(Ph8, super::sig::DcmiHsync, 13);
   alt_fn!(Ph8, super::sig::Eventout, 15);

pin!(PH9, Ph9, GPIOH, Gpioh, 9);
   alt_fn!(Ph9, super::sig::I2c3Smba, 4);
   alt_fn!(Ph9, super::sig::Tim12Ch2, 9);
   alt_fn!(Ph9, super::sig::DcmiD0, 13);
   alt_fn!(Ph9, super::sig::Eventout, 15);

pin!(PH10, Ph10, GPIOH, Gpioh, 10);
   alt_fn!(Ph10, super::sig::Tim5Ch1, 2);
   alt_fn!(Ph10, super::sig::DcmiD1, 13);
   alt_fn!(Ph10, super::sig::Eventout, 15);

pin!(PH11, Ph11, GPIOH, Gpioh, 11);
   alt_fn!(Ph11, super::sig::Tim5Ch2, 2);
   alt_fn!(Ph11, super::sig::DcmiD2, 13);
   alt_fn!(Ph11, super::sig::Eventout, 15);

pin!(PH12, Ph12, GPIOH, Gpioh, 12);
   alt_fn!(Ph12, super::sig::Tim5Ch3, 2);
   alt_fn!(Ph12, super::sig::DcmiD3, 13);
   alt_fn!(Ph12, super::sig::Eventout, 15);

pin!(PH13, Ph13, GPIOH, Gpioh, 13);
   alt_fn!(Ph13, super::sig::Tim8Ch1n, 3);
   alt_fn!(Ph13, super::sig::Can1Tx, 9);
   alt_fn!(Ph13, super::sig::Eventout, 15);

pin!(PH14, Ph14, GPIOH, Gpioh, 14);
   alt_fn!(Ph14, super::sig::Tim8Ch2n, 3);
   alt_fn!(Ph14, super::sig::DcmiD4, 13);
   alt_fn!(Ph14, super::sig::Eventout, 15);

pin!(PH15, Ph15, GPIOH, Gpioh, 15);
   alt_fn!(Ph15, super::sig::Tim8Ch3n, 3);
   alt_fn!(Ph15, super::sig::DcmiD11, 13);
   alt_fn!(Ph15, super::sig::Eventout, 15);

pin!(PI0, Pi0, GPIOI, Gpioi, 0);
   alt_fn!(Pi0, super::sig::Tim5Ch4, 2);
   alt_fn!(Pi0, super::sig::Spi2Nss, 5);
   alt_fn!(Pi0, super::sig::I2s2Ws, 5);
   alt_fn!(Pi0, super::sig::DcmiD13, 13);
   alt_fn!(Pi0, super::sig::Eventout, 15);

pin!(PI1, Pi1, GPIOI, Gpioi, 1);
   alt_fn!(Pi1, super::sig::Spi2Sck, 5);
   alt_fn!(Pi1, super::sig::I2s2Ck, 5);
   alt_fn!(Pi1, super::sig::DcmiD8, 13);
   alt_fn!(Pi1, super::sig::Eventout, 15);

pin!(PI2, Pi2, GPIOI, Gpioi, 2);
   alt_fn!(Pi2, super::sig::Tim8Ch4, 3);
   alt_fn!(Pi2, super::sig::Spi2Miso, 5);
   alt_fn!(Pi2, super::sig::I2s2extSd, 6);
   alt_fn!(Pi2, super::sig::DcmiD9, 13);
   alt_fn!(Pi2, super::sig::Eventout, 15);

pin!(PI3, Pi3, GPIOI, Gpioi, 3);
   alt_fn!(Pi3, super::sig::Tim8Etr, 3);
   alt_fn!(Pi3, super::sig::Spi2Mosi, 5);
   alt_fn!(Pi3, super::sig::I2s2Sd, 5);
   alt_fn!(Pi3, super::sig::DcmiD10, 13);
   alt_fn!(Pi3, super::sig::Eventout, 15);

pin!(PI4, Pi4, GPIOI, Gpioi, 4);
   alt_fn!(Pi4, super::sig::Tim8Bkin, 3);
   alt_fn!(Pi4, super::sig::DcmiD5, 13);
   alt_fn!(Pi4, super::sig::Eventout, 15);

pin!(PI5, Pi5, GPIOI, Gpioi, 5);
   alt_fn!(Pi5, super::sig::Tim8Ch1, 3);
   alt_fn!(Pi5, super::sig::DcmiVsync, 13);
   alt_fn!(Pi5, super::sig::Eventout, 15);

pin!(PI6, Pi6, GPIOI, Gpioi, 6);
   alt_fn!(Pi6, super::sig::Tim8Ch2, 3);
   alt_fn!(Pi6, super::sig::DcmiD6, 13);
   alt_fn!(Pi6, super::sig::Eventout, 15);

pin!(PI7, Pi7, GPIOI, Gpioi, 7);
   alt_fn!(Pi7, super::sig::Tim8Ch3, 3);
   alt_fn!(Pi7, super::sig::DcmiD7, 13);
   alt_fn!(Pi7, super::sig::Eventout, 15);

pin!(PI8, Pi8, GPIOI, Gpioi, 8);
   alt_fn!(Pi8, super::sig::Eventout, 15);

pin!(PI9, Pi9, GPIOI, Gpioi, 9);
   alt_fn!(Pi9, super::sig::Can1Rx, 9);
   alt_fn!(Pi9, super::sig::Eventout, 15);

pin!(PI10, Pi10, GPIOI, Gpioi, 10);
   alt_fn!(Pi10, super::sig::EthMiiRxEr, 11);
   alt_fn!(Pi10, super::sig::Eventout, 15);

