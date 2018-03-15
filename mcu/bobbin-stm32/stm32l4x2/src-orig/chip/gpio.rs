#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::gpio::*;

periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x48000000);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x48000400);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x48000800);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x48000c00);
periph!( GPIOE, Gpioe, _GPIOE, GpioPeriph, 0x48001000);
periph!( GPIOF, Gpiof, _GPIOF, GpioPeriph, 0x48001400);
periph!( GPIOG, Gpiog, _GPIOG, GpioPeriph, 0x48001800);
periph!( GPIOH, Gpioh, _GPIOH, GpioPeriph, 0x48001c00);










pin!(PA0, Pa0, GPIOA, Gpioa, _PA0, GpioPin, _GPIOA, 0);
    alt_fn!(Pa0, super::sig::Tim2Ch1, 1);
    alt_fn!(Pa0, super::sig::Usart2Cts, 7);
    alt_fn!(Pa0, super::sig::Uart4Tx, 8);
    alt_fn!(Pa0, super::sig::Comp1Out, 12);
    alt_fn!(Pa0, super::sig::Sai1Extclk, 13);
    alt_fn!(Pa0, super::sig::Tim2Etr, 14);
    alt_fn!(Pa0, super::sig::Eventout, 15);

pin!(PA1, Pa1, GPIOA, Gpioa, _PA1, GpioPin, _GPIOA, 1);
    alt_fn!(Pa1, super::sig::Tim2Ch2, 1);
    alt_fn!(Pa1, super::sig::I2c1Smba, 4);
    alt_fn!(Pa1, super::sig::Spi1Sck, 5);
    alt_fn!(Pa1, super::sig::Usart2RtsDe, 7);
    alt_fn!(Pa1, super::sig::Uart4Rx, 8);
    alt_fn!(Pa1, super::sig::Tim15Ch1n, 14);
    alt_fn!(Pa1, super::sig::Eventout, 15);

pin!(PA2, Pa2, GPIOA, Gpioa, _PA2, GpioPin, _GPIOA, 2);
    alt_fn!(Pa2, super::sig::Tim2Ch3, 1);
    alt_fn!(Pa2, super::sig::Usart2Tx, 7);
    alt_fn!(Pa2, super::sig::Lpuart1Tx, 8);
    alt_fn!(Pa2, super::sig::QuadspiBk1Ncs, 10);
    alt_fn!(Pa2, super::sig::Comp2Out, 12);
    alt_fn!(Pa2, super::sig::Tim15Ch1, 14);
    alt_fn!(Pa2, super::sig::Eventout, 15);

pin!(PA3, Pa3, GPIOA, Gpioa, _PA3, GpioPin, _GPIOA, 3);
    alt_fn!(Pa3, super::sig::Tim2Ch4, 1);
    alt_fn!(Pa3, super::sig::Usart2Rx, 7);
    alt_fn!(Pa3, super::sig::Lpuart1Rx, 8);
    alt_fn!(Pa3, super::sig::QuadspiClk, 10);
    alt_fn!(Pa3, super::sig::Sai1MclkA, 13);
    alt_fn!(Pa3, super::sig::Tim15Ch2, 14);
    alt_fn!(Pa3, super::sig::Eventout, 15);

pin!(PA4, Pa4, GPIOA, Gpioa, _PA4, GpioPin, _GPIOA, 4);
    alt_fn!(Pa4, super::sig::Spi1Nss, 5);
    alt_fn!(Pa4, super::sig::Spi3Nss, 6);
    alt_fn!(Pa4, super::sig::Usart2Ck, 7);
    alt_fn!(Pa4, super::sig::Sai1FsB, 13);
    alt_fn!(Pa4, super::sig::Lptim2Out, 14);
    alt_fn!(Pa4, super::sig::Eventout, 15);

pin!(PA5, Pa5, GPIOA, Gpioa, _PA5, GpioPin, _GPIOA, 5);
    alt_fn!(Pa5, super::sig::Tim2Ch1, 1);
    alt_fn!(Pa5, super::sig::Tim2Etr, 2);
    alt_fn!(Pa5, super::sig::Spi1Sck, 5);
    alt_fn!(Pa5, super::sig::Dfsdm1Ckout, 6);
    alt_fn!(Pa5, super::sig::Lptim2Etr, 14);
    alt_fn!(Pa5, super::sig::Eventout, 15);

pin!(PA6, Pa6, GPIOA, Gpioa, _PA6, GpioPin, _GPIOA, 6);
    alt_fn!(Pa6, super::sig::Tim1Bkin, 1);
    alt_fn!(Pa6, super::sig::Tim3Ch1, 2);
    alt_fn!(Pa6, super::sig::Spi1Miso, 5);
    alt_fn!(Pa6, super::sig::Comp1Out, 6);
    alt_fn!(Pa6, super::sig::Usart3Cts, 7);
    alt_fn!(Pa6, super::sig::Lpuart1Cts, 8);
    alt_fn!(Pa6, super::sig::QuadspiBk1Io3, 10);
    alt_fn!(Pa6, super::sig::Tim1BkinComp2, 12);
    alt_fn!(Pa6, super::sig::Tim16Ch1, 14);
    alt_fn!(Pa6, super::sig::Eventout, 15);

pin!(PA7, Pa7, GPIOA, Gpioa, _PA7, GpioPin, _GPIOA, 7);
    alt_fn!(Pa7, super::sig::Tim1Ch1n, 1);
    alt_fn!(Pa7, super::sig::Tim3Ch2, 2);
    alt_fn!(Pa7, super::sig::I2c3Scl, 4);
    alt_fn!(Pa7, super::sig::Spi1Mosi, 5);
    alt_fn!(Pa7, super::sig::Dfsdm1Datin0, 6);
    alt_fn!(Pa7, super::sig::QuadspiBk1Io2, 10);
    alt_fn!(Pa7, super::sig::Comp2Out, 12);
    alt_fn!(Pa7, super::sig::Eventout, 15);

pin!(PA8, Pa8, GPIOA, Gpioa, _PA8, GpioPin, _GPIOA, 8);
    alt_fn!(Pa8, super::sig::Mco, 0);
    alt_fn!(Pa8, super::sig::Tim1Ch1, 1);
    alt_fn!(Pa8, super::sig::Dfsdm1Ckin1, 6);
    alt_fn!(Pa8, super::sig::Usart1Ck, 7);
    alt_fn!(Pa8, super::sig::Sai1SckA, 13);
    alt_fn!(Pa8, super::sig::Lptim2Out, 14);
    alt_fn!(Pa8, super::sig::Eventout, 15);

pin!(PA9, Pa9, GPIOA, Gpioa, _PA9, GpioPin, _GPIOA, 9);
    alt_fn!(Pa9, super::sig::Tim1Ch2, 1);
    alt_fn!(Pa9, super::sig::I2c1Scl, 4);
    alt_fn!(Pa9, super::sig::Dfsdm1Datin1, 6);
    alt_fn!(Pa9, super::sig::Usart1Tx, 7);
    alt_fn!(Pa9, super::sig::Sai1FsA, 13);
    alt_fn!(Pa9, super::sig::Tim15Bkin, 14);
    alt_fn!(Pa9, super::sig::Eventout, 15);

pin!(PA10, Pa10, GPIOA, Gpioa, _PA10, GpioPin, _GPIOA, 10);
    alt_fn!(Pa10, super::sig::Tim1Ch3, 1);
    alt_fn!(Pa10, super::sig::I2c1Sda, 4);
    alt_fn!(Pa10, super::sig::Usart1Rx, 7);
    alt_fn!(Pa10, super::sig::UsbcrsSync, 10);
    alt_fn!(Pa10, super::sig::Sai1SdA, 13);
    alt_fn!(Pa10, super::sig::Eventout, 15);

pin!(PA11, Pa11, GPIOA, Gpioa, _PA11, GpioPin, _GPIOA, 11);
    alt_fn!(Pa11, super::sig::Tim1Ch4, 1);
    alt_fn!(Pa11, super::sig::Tim1Bkin2, 2);
    alt_fn!(Pa11, super::sig::Spi1Miso, 5);
    alt_fn!(Pa11, super::sig::Comp1Out, 6);
    alt_fn!(Pa11, super::sig::Usart1Cts, 7);
    alt_fn!(Pa11, super::sig::Can1Rx, 9);
    alt_fn!(Pa11, super::sig::Usbdm, 10);
    alt_fn!(Pa11, super::sig::Tim1Bkin2Comp1, 12);
    alt_fn!(Pa11, super::sig::Eventout, 15);

pin!(PA12, Pa12, GPIOA, Gpioa, _PA12, GpioPin, _GPIOA, 12);
    alt_fn!(Pa12, super::sig::Tim1Etr, 1);
    alt_fn!(Pa12, super::sig::Spi1Mosi, 5);
    alt_fn!(Pa12, super::sig::Usart1RtsDe, 7);
    alt_fn!(Pa12, super::sig::Can1Tx, 9);
    alt_fn!(Pa12, super::sig::Usbdp, 10);
    alt_fn!(Pa12, super::sig::Eventout, 15);

pin!(PA13, Pa13, GPIOA, Gpioa, _PA13, GpioPin, _GPIOA, 13);
    alt_fn!(Pa13, super::sig::Jtms, 0);
    alt_fn!(Pa13, super::sig::Swdat, 0);
    alt_fn!(Pa13, super::sig::IrOut, 1);
    alt_fn!(Pa13, super::sig::Usbnoe, 10);
    alt_fn!(Pa13, super::sig::Sai1SdB, 13);
    alt_fn!(Pa13, super::sig::Eventout, 15);

pin!(PA14, Pa14, GPIOA, Gpioa, _PA14, GpioPin, _GPIOA, 14);
    alt_fn!(Pa14, super::sig::Jtck, 0);
    alt_fn!(Pa14, super::sig::Swclk, 0);
    alt_fn!(Pa14, super::sig::Lptim1Out, 1);
    alt_fn!(Pa14, super::sig::I2c1Smba, 4);
    alt_fn!(Pa14, super::sig::I2c4Smba, 5);
    alt_fn!(Pa14, super::sig::Sai1FsB, 13);
    alt_fn!(Pa14, super::sig::Eventout, 15);

pin!(PA15, Pa15, GPIOA, Gpioa, _PA15, GpioPin, _GPIOA, 15);
    alt_fn!(Pa15, super::sig::Jtdi, 0);
    alt_fn!(Pa15, super::sig::Tim2Ch1, 1);
    alt_fn!(Pa15, super::sig::Tim2Etr, 2);
    alt_fn!(Pa15, super::sig::Usart2Rx, 3);
    alt_fn!(Pa15, super::sig::Spi1Nss, 5);
    alt_fn!(Pa15, super::sig::Spi3Nss, 6);
    alt_fn!(Pa15, super::sig::Usart3RtsDe, 7);
    alt_fn!(Pa15, super::sig::Uart4RtsDe, 8);
    alt_fn!(Pa15, super::sig::TscG3Io1, 9);
    alt_fn!(Pa15, super::sig::Eventout, 15);

pin!(PB0, Pb0, GPIOB, Gpiob, _PB0, GpioPin, _GPIOB, 0);
    alt_fn!(Pb0, super::sig::Tim1Ch2n, 1);
    alt_fn!(Pb0, super::sig::Tim3Ch3, 2);
    alt_fn!(Pb0, super::sig::Spi1Nss, 5);
    alt_fn!(Pb0, super::sig::Dfsdm1Ckin0, 6);
    alt_fn!(Pb0, super::sig::Usart3Ck, 7);
    alt_fn!(Pb0, super::sig::QuadspiBk1Io1, 10);
    alt_fn!(Pb0, super::sig::Comp1Out, 12);
    alt_fn!(Pb0, super::sig::Sai1Extclk, 13);
    alt_fn!(Pb0, super::sig::Eventout, 15);

pin!(PB1, Pb1, GPIOB, Gpiob, _PB1, GpioPin, _GPIOB, 1);
    alt_fn!(Pb1, super::sig::Tim1Ch3n, 1);
    alt_fn!(Pb1, super::sig::Tim3Ch4, 2);
    alt_fn!(Pb1, super::sig::Dfsdm1Datin0, 6);
    alt_fn!(Pb1, super::sig::Usart3RtsDe, 7);
    alt_fn!(Pb1, super::sig::Lpuart1RtsDe, 8);
    alt_fn!(Pb1, super::sig::QuadspiBk1Io0, 10);
    alt_fn!(Pb1, super::sig::Lptim2In1, 14);
    alt_fn!(Pb1, super::sig::Eventout, 15);

pin!(PB2, Pb2, GPIOB, Gpiob, _PB2, GpioPin, _GPIOB, 2);
    alt_fn!(Pb2, super::sig::RtcOut, 0);
    alt_fn!(Pb2, super::sig::Lptim1Out, 1);
    alt_fn!(Pb2, super::sig::I2c3Smba, 4);
    alt_fn!(Pb2, super::sig::Dfsdm1Ckin0, 6);
    alt_fn!(Pb2, super::sig::Eventout, 15);

pin!(PB3, Pb3, GPIOB, Gpiob, _PB3, GpioPin, _GPIOB, 3);
    alt_fn!(Pb3, super::sig::Jtdo, 0);
    alt_fn!(Pb3, super::sig::Traceswo, 0);
    alt_fn!(Pb3, super::sig::Tim2Ch2, 1);
    alt_fn!(Pb3, super::sig::Spi1Sck, 5);
    alt_fn!(Pb3, super::sig::Spi3Sck, 6);
    alt_fn!(Pb3, super::sig::Usart1RtsDe, 7);
    alt_fn!(Pb3, super::sig::Sai1SckB, 13);
    alt_fn!(Pb3, super::sig::Eventout, 15);

pin!(PB4, Pb4, GPIOB, Gpiob, _PB4, GpioPin, _GPIOB, 4);
    alt_fn!(Pb4, super::sig::Njtrst, 0);
    alt_fn!(Pb4, super::sig::Tim3Ch1, 2);
    alt_fn!(Pb4, super::sig::I2c3Sda, 4);
    alt_fn!(Pb4, super::sig::Spi1Miso, 5);
    alt_fn!(Pb4, super::sig::Spi3Miso, 6);
    alt_fn!(Pb4, super::sig::Usart1Cts, 7);
    alt_fn!(Pb4, super::sig::TscG2Io1, 9);
    alt_fn!(Pb4, super::sig::Sai1MclkB, 13);
    alt_fn!(Pb4, super::sig::Eventout, 15);

pin!(PB5, Pb5, GPIOB, Gpiob, _PB5, GpioPin, _GPIOB, 5);
    alt_fn!(Pb5, super::sig::Lptim1In1, 1);
    alt_fn!(Pb5, super::sig::Tim3Ch2, 2);
    alt_fn!(Pb5, super::sig::Can1Rx, 3);
    alt_fn!(Pb5, super::sig::I2c1Smba, 4);
    alt_fn!(Pb5, super::sig::Spi1Mosi, 5);
    alt_fn!(Pb5, super::sig::Spi3Mosi, 6);
    alt_fn!(Pb5, super::sig::Usart1Ck, 7);
    alt_fn!(Pb5, super::sig::TscG2Io2, 9);
    alt_fn!(Pb5, super::sig::Comp2Out, 12);
    alt_fn!(Pb5, super::sig::Sai1SdB, 13);
    alt_fn!(Pb5, super::sig::Tim16Bkin, 14);
    alt_fn!(Pb5, super::sig::Eventout, 15);

pin!(PB6, Pb6, GPIOB, Gpiob, _PB6, GpioPin, _GPIOB, 6);
    alt_fn!(Pb6, super::sig::Lptim1Etr, 1);
    alt_fn!(Pb6, super::sig::I2c1Scl, 4);
    alt_fn!(Pb6, super::sig::I2c4Scl, 5);
    alt_fn!(Pb6, super::sig::Usart1Tx, 7);
    alt_fn!(Pb6, super::sig::Can1Tx, 8);
    alt_fn!(Pb6, super::sig::TscG2Io3, 9);
    alt_fn!(Pb6, super::sig::Sai1FsB, 13);
    alt_fn!(Pb6, super::sig::Tim16Ch1n, 14);
    alt_fn!(Pb6, super::sig::Eventout, 15);

pin!(PB7, Pb7, GPIOB, Gpiob, _PB7, GpioPin, _GPIOB, 7);
    alt_fn!(Pb7, super::sig::Lptim1In2, 1);
    alt_fn!(Pb7, super::sig::I2c1Sda, 4);
    alt_fn!(Pb7, super::sig::I2c4Sda, 5);
    alt_fn!(Pb7, super::sig::Usart1Rx, 7);
    alt_fn!(Pb7, super::sig::Uart4Cts, 8);
    alt_fn!(Pb7, super::sig::TscG2Io4, 9);
    alt_fn!(Pb7, super::sig::Eventout, 15);

pin!(PB8, Pb8, GPIOB, Gpiob, _PB8, GpioPin, _GPIOB, 8);
    alt_fn!(Pb8, super::sig::I2c1Scl, 4);
    alt_fn!(Pb8, super::sig::Can1Rx, 9);
    alt_fn!(Pb8, super::sig::Sdmmc1D4, 12);
    alt_fn!(Pb8, super::sig::Sai1MclkA, 13);
    alt_fn!(Pb8, super::sig::Tim16Ch1, 14);
    alt_fn!(Pb8, super::sig::Eventout, 15);

pin!(PB9, Pb9, GPIOB, Gpiob, _PB9, GpioPin, _GPIOB, 9);
    alt_fn!(Pb9, super::sig::IrOut, 1);
    alt_fn!(Pb9, super::sig::I2c1Sda, 4);
    alt_fn!(Pb9, super::sig::Spi2Nss, 5);
    alt_fn!(Pb9, super::sig::Can1Tx, 9);
    alt_fn!(Pb9, super::sig::Sdmmc1D5, 12);
    alt_fn!(Pb9, super::sig::Sai1FsA, 13);
    alt_fn!(Pb9, super::sig::Eventout, 15);

pin!(PB10, Pb10, GPIOB, Gpiob, _PB10, GpioPin, _GPIOB, 10);
    alt_fn!(Pb10, super::sig::Tim2Ch3, 1);
    alt_fn!(Pb10, super::sig::I2c4Scl, 3);
    alt_fn!(Pb10, super::sig::I2c2Scl, 4);
    alt_fn!(Pb10, super::sig::Spi2Sck, 5);
    alt_fn!(Pb10, super::sig::Usart3Tx, 7);
    alt_fn!(Pb10, super::sig::Lpuart1Rx, 8);
    alt_fn!(Pb10, super::sig::TscSync, 9);
    alt_fn!(Pb10, super::sig::QuadspiClk, 10);
    alt_fn!(Pb10, super::sig::Comp1Out, 12);
    alt_fn!(Pb10, super::sig::Sai1SckA, 13);
    alt_fn!(Pb10, super::sig::Eventout, 15);

pin!(PB11, Pb11, GPIOB, Gpiob, _PB11, GpioPin, _GPIOB, 11);
    alt_fn!(Pb11, super::sig::Tim2Ch4, 1);
    alt_fn!(Pb11, super::sig::I2c4Sda, 3);
    alt_fn!(Pb11, super::sig::I2c2Sda, 4);
    alt_fn!(Pb11, super::sig::Usart3Rx, 7);
    alt_fn!(Pb11, super::sig::Lpuart1Tx, 8);
    alt_fn!(Pb11, super::sig::QuadspiBk1Ncs, 10);
    alt_fn!(Pb11, super::sig::Comp2Out, 12);
    alt_fn!(Pb11, super::sig::Eventout, 15);

pin!(PB12, Pb12, GPIOB, Gpiob, _PB12, GpioPin, _GPIOB, 12);
    alt_fn!(Pb12, super::sig::Tim1Bkin, 1);
    alt_fn!(Pb12, super::sig::Tim1BkinComp2, 3);
    alt_fn!(Pb12, super::sig::I2c2Smba, 4);
    alt_fn!(Pb12, super::sig::Spi2Nss, 5);
    alt_fn!(Pb12, super::sig::Dfsdm1Datin1, 6);
    alt_fn!(Pb12, super::sig::Usart3Ck, 7);
    alt_fn!(Pb12, super::sig::Lpuart1RtsDe, 8);
    alt_fn!(Pb12, super::sig::TscG1Io1, 9);
    alt_fn!(Pb12, super::sig::Can1Rx, 10);
    alt_fn!(Pb12, super::sig::Sai1FsA, 13);
    alt_fn!(Pb12, super::sig::Tim15Bkin, 14);
    alt_fn!(Pb12, super::sig::Eventout, 15);

pin!(PB13, Pb13, GPIOB, Gpiob, _PB13, GpioPin, _GPIOB, 13);
    alt_fn!(Pb13, super::sig::Tim1Ch1n, 1);
    alt_fn!(Pb13, super::sig::I2c2Scl, 4);
    alt_fn!(Pb13, super::sig::Spi2Sck, 5);
    alt_fn!(Pb13, super::sig::Dfsdm1Ckin1, 6);
    alt_fn!(Pb13, super::sig::Usart3Cts, 7);
    alt_fn!(Pb13, super::sig::Lpuart1Cts, 8);
    alt_fn!(Pb13, super::sig::TscG1Io2, 9);
    alt_fn!(Pb13, super::sig::Can1Tx, 10);
    alt_fn!(Pb13, super::sig::Sai1SckA, 13);
    alt_fn!(Pb13, super::sig::Tim15Ch1n, 14);
    alt_fn!(Pb13, super::sig::Eventout, 15);

pin!(PB14, Pb14, GPIOB, Gpiob, _PB14, GpioPin, _GPIOB, 14);
    alt_fn!(Pb14, super::sig::Tim1Ch2n, 1);
    alt_fn!(Pb14, super::sig::I2c2Sda, 4);
    alt_fn!(Pb14, super::sig::Spi2Miso, 5);
    alt_fn!(Pb14, super::sig::Dfsdm1Datin2, 6);
    alt_fn!(Pb14, super::sig::Usart3RtsDe, 7);
    alt_fn!(Pb14, super::sig::TscG1Io3, 9);
    alt_fn!(Pb14, super::sig::Sai1MclkA, 13);
    alt_fn!(Pb14, super::sig::Tim15Ch1, 14);
    alt_fn!(Pb14, super::sig::Eventout, 15);

pin!(PB15, Pb15, GPIOB, Gpiob, _PB15, GpioPin, _GPIOB, 15);
    alt_fn!(Pb15, super::sig::RtcRefin, 0);
    alt_fn!(Pb15, super::sig::Tim1Ch3n, 1);
    alt_fn!(Pb15, super::sig::Spi2Mosi, 5);
    alt_fn!(Pb15, super::sig::Dfsdm1Ckin2, 6);
    alt_fn!(Pb15, super::sig::TscG1Io4, 9);
    alt_fn!(Pb15, super::sig::Sai1SdA, 13);
    alt_fn!(Pb15, super::sig::Tim15Ch2, 14);
    alt_fn!(Pb15, super::sig::Eventout, 15);

pin!(PC0, Pc0, GPIOC, Gpioc, _PC0, GpioPin, _GPIOC, 0);
    alt_fn!(Pc0, super::sig::Lptim1In1, 1);
    alt_fn!(Pc0, super::sig::I2c4Scl, 2);
    alt_fn!(Pc0, super::sig::I2c3Scl, 4);
    alt_fn!(Pc0, super::sig::Lpuart1Rx, 8);
    alt_fn!(Pc0, super::sig::Lptim2In1, 14);
    alt_fn!(Pc0, super::sig::Eventout, 15);

pin!(PC1, Pc1, GPIOC, Gpioc, _PC1, GpioPin, _GPIOC, 1);
    alt_fn!(Pc1, super::sig::Traced0, 0);
    alt_fn!(Pc1, super::sig::Lptim1Out, 1);
    alt_fn!(Pc1, super::sig::I2c4Sda, 2);
    alt_fn!(Pc1, super::sig::I2c3Sda, 4);
    alt_fn!(Pc1, super::sig::Lpuart1Tx, 8);
    alt_fn!(Pc1, super::sig::Eventout, 15);

pin!(PC2, Pc2, GPIOC, Gpioc, _PC2, GpioPin, _GPIOC, 2);
    alt_fn!(Pc2, super::sig::Lptim1In2, 1);
    alt_fn!(Pc2, super::sig::Spi2Miso, 5);
    alt_fn!(Pc2, super::sig::Dfsdm1Ckout, 6);
    alt_fn!(Pc2, super::sig::Eventout, 15);

pin!(PC3, Pc3, GPIOC, Gpioc, _PC3, GpioPin, _GPIOC, 3);
    alt_fn!(Pc3, super::sig::Lptim1Etr, 1);
    alt_fn!(Pc3, super::sig::Spi2Mosi, 5);
    alt_fn!(Pc3, super::sig::Sai1SdA, 13);
    alt_fn!(Pc3, super::sig::Lptim2Etr, 14);
    alt_fn!(Pc3, super::sig::Eventout, 15);

pin!(PC4, Pc4, GPIOC, Gpioc, _PC4, GpioPin, _GPIOC, 4);
    alt_fn!(Pc4, super::sig::Usart3Tx, 7);
    alt_fn!(Pc4, super::sig::Eventout, 15);

pin!(PC5, Pc5, GPIOC, Gpioc, _PC5, GpioPin, _GPIOC, 5);
    alt_fn!(Pc5, super::sig::Usart3Rx, 7);
    alt_fn!(Pc5, super::sig::Eventout, 15);

pin!(PC6, Pc6, GPIOC, Gpioc, _PC6, GpioPin, _GPIOC, 6);
    alt_fn!(Pc6, super::sig::Tim3Ch1, 2);
    alt_fn!(Pc6, super::sig::Dfsdm1Ckin3, 6);
    alt_fn!(Pc6, super::sig::TscG4Io1, 9);
    alt_fn!(Pc6, super::sig::Sdmmc1D6, 12);
    alt_fn!(Pc6, super::sig::Eventout, 15);

pin!(PC7, Pc7, GPIOC, Gpioc, _PC7, GpioPin, _GPIOC, 7);
    alt_fn!(Pc7, super::sig::Tim3Ch2, 2);
    alt_fn!(Pc7, super::sig::Dfsdm1Datin3, 6);
    alt_fn!(Pc7, super::sig::TscG4Io2, 9);
    alt_fn!(Pc7, super::sig::Sdmmc1D7, 12);
    alt_fn!(Pc7, super::sig::Eventout, 15);

pin!(PC8, Pc8, GPIOC, Gpioc, _PC8, GpioPin, _GPIOC, 8);
    alt_fn!(Pc8, super::sig::Tim3Ch3, 2);
    alt_fn!(Pc8, super::sig::TscG4Io3, 9);
    alt_fn!(Pc8, super::sig::Sdmmc1D0, 12);
    alt_fn!(Pc8, super::sig::Eventout, 15);

pin!(PC9, Pc9, GPIOC, Gpioc, _PC9, GpioPin, _GPIOC, 9);
    alt_fn!(Pc9, super::sig::Tim3Ch4, 2);
    alt_fn!(Pc9, super::sig::TscG4Io4, 9);
    alt_fn!(Pc9, super::sig::Usbnoe, 10);
    alt_fn!(Pc9, super::sig::Sdmmc1D1, 12);
    alt_fn!(Pc9, super::sig::Eventout, 15);

pin!(PC10, Pc10, GPIOC, Gpioc, _PC10, GpioPin, _GPIOC, 10);
    alt_fn!(Pc10, super::sig::Traced1, 0);
    alt_fn!(Pc10, super::sig::Spi3Sck, 6);
    alt_fn!(Pc10, super::sig::Usart3Tx, 7);
    alt_fn!(Pc10, super::sig::Uart4Tx, 8);
    alt_fn!(Pc10, super::sig::TscG3Io2, 9);
    alt_fn!(Pc10, super::sig::Sdmmc1D2, 12);
    alt_fn!(Pc10, super::sig::Eventout, 15);

pin!(PC11, Pc11, GPIOC, Gpioc, _PC11, GpioPin, _GPIOC, 11);
    alt_fn!(Pc11, super::sig::Spi3Miso, 6);
    alt_fn!(Pc11, super::sig::Usart3Rx, 7);
    alt_fn!(Pc11, super::sig::Uart4Rx, 8);
    alt_fn!(Pc11, super::sig::TscG3Io3, 9);
    alt_fn!(Pc11, super::sig::Sdmmc1D3, 12);
    alt_fn!(Pc11, super::sig::Eventout, 15);

pin!(PC12, Pc12, GPIOC, Gpioc, _PC12, GpioPin, _GPIOC, 12);
    alt_fn!(Pc12, super::sig::Traced3, 0);
    alt_fn!(Pc12, super::sig::Spi3Mosi, 6);
    alt_fn!(Pc12, super::sig::Usart3Ck, 7);
    alt_fn!(Pc12, super::sig::TscG3Io4, 9);
    alt_fn!(Pc12, super::sig::Sdmmc1Ck, 12);
    alt_fn!(Pc12, super::sig::Eventout, 15);

pin!(PC13, Pc13, GPIOC, Gpioc, _PC13, GpioPin, _GPIOC, 13);
    alt_fn!(Pc13, super::sig::Eventout, 15);

pin!(PC14, Pc14, GPIOC, Gpioc, _PC14, GpioPin, _GPIOC, 14);
    alt_fn!(Pc14, super::sig::Eventout, 15);

pin!(PC15, Pc15, GPIOC, Gpioc, _PC15, GpioPin, _GPIOC, 15);
    alt_fn!(Pc15, super::sig::Eventout, 15);

pin!(PD0, Pd0, GPIOD, Gpiod, _PD0, GpioPin, _GPIOD, 0);
    alt_fn!(Pd0, super::sig::Spi2Nss, 5);
    alt_fn!(Pd0, super::sig::Can1Rx, 9);
    alt_fn!(Pd0, super::sig::Eventout, 15);

pin!(PD1, Pd1, GPIOD, Gpiod, _PD1, GpioPin, _GPIOD, 1);
    alt_fn!(Pd1, super::sig::Spi2Sck, 5);
    alt_fn!(Pd1, super::sig::Can1Tx, 9);
    alt_fn!(Pd1, super::sig::Eventout, 15);

pin!(PD2, Pd2, GPIOD, Gpiod, _PD2, GpioPin, _GPIOD, 2);
    alt_fn!(Pd2, super::sig::Traced2, 0);
    alt_fn!(Pd2, super::sig::Tim3Etr, 2);
    alt_fn!(Pd2, super::sig::Usart3RtsDe, 7);
    alt_fn!(Pd2, super::sig::TscSync, 9);
    alt_fn!(Pd2, super::sig::Sdmmc1Cmd, 12);
    alt_fn!(Pd2, super::sig::Eventout, 15);

pin!(PD3, Pd3, GPIOD, Gpiod, _PD3, GpioPin, _GPIOD, 3);
    alt_fn!(Pd3, super::sig::Spi2Miso, 5);
    alt_fn!(Pd3, super::sig::Dfsdm1Datin0, 6);
    alt_fn!(Pd3, super::sig::Usart2Cts, 7);
    alt_fn!(Pd3, super::sig::QuadspiBk2Ncs, 10);
    alt_fn!(Pd3, super::sig::Eventout, 15);

pin!(PD4, Pd4, GPIOD, Gpiod, _PD4, GpioPin, _GPIOD, 4);
    alt_fn!(Pd4, super::sig::Spi2Mosi, 5);
    alt_fn!(Pd4, super::sig::Dfsdm1Ckin0, 6);
    alt_fn!(Pd4, super::sig::Usart2RtsDe, 7);
    alt_fn!(Pd4, super::sig::QuadspiBk2Io0, 10);
    alt_fn!(Pd4, super::sig::Eventout, 15);

pin!(PD5, Pd5, GPIOD, Gpiod, _PD5, GpioPin, _GPIOD, 5);
    alt_fn!(Pd5, super::sig::Usart2Tx, 7);
    alt_fn!(Pd5, super::sig::QuadspiBk2Io1, 10);
    alt_fn!(Pd5, super::sig::Eventout, 15);

pin!(PD6, Pd6, GPIOD, Gpiod, _PD6, GpioPin, _GPIOD, 6);
    alt_fn!(Pd6, super::sig::Dfsdm1Datin1, 6);
    alt_fn!(Pd6, super::sig::Usart2Rx, 7);
    alt_fn!(Pd6, super::sig::QuadspiBk2Io2, 10);
    alt_fn!(Pd6, super::sig::Sai1SdA, 13);
    alt_fn!(Pd6, super::sig::Eventout, 15);

pin!(PD7, Pd7, GPIOD, Gpiod, _PD7, GpioPin, _GPIOD, 7);
    alt_fn!(Pd7, super::sig::Dfsdm1Ckin1, 6);
    alt_fn!(Pd7, super::sig::Usart2Ck, 7);
    alt_fn!(Pd7, super::sig::QuadspiBk2Io3, 10);
    alt_fn!(Pd7, super::sig::Eventout, 15);

pin!(PD8, Pd8, GPIOD, Gpiod, _PD8, GpioPin, _GPIOD, 8);
    alt_fn!(Pd8, super::sig::Usart3Tx, 7);
    alt_fn!(Pd8, super::sig::Eventout, 15);

pin!(PD9, Pd9, GPIOD, Gpiod, _PD9, GpioPin, _GPIOD, 9);
    alt_fn!(Pd9, super::sig::Usart3Rx, 7);
    alt_fn!(Pd9, super::sig::Eventout, 15);

pin!(PD10, Pd10, GPIOD, Gpiod, _PD10, GpioPin, _GPIOD, 10);
    alt_fn!(Pd10, super::sig::Usart3Ck, 7);
    alt_fn!(Pd10, super::sig::TscG6Io1, 9);
    alt_fn!(Pd10, super::sig::Eventout, 15);

pin!(PD11, Pd11, GPIOD, Gpiod, _PD11, GpioPin, _GPIOD, 11);
    alt_fn!(Pd11, super::sig::I2c4Smba, 4);
    alt_fn!(Pd11, super::sig::Usart3Cts, 7);
    alt_fn!(Pd11, super::sig::TscG6Io2, 9);
    alt_fn!(Pd11, super::sig::Lptim2Etr, 14);
    alt_fn!(Pd11, super::sig::Eventout, 15);

pin!(PD12, Pd12, GPIOD, Gpiod, _PD12, GpioPin, _GPIOD, 12);
    alt_fn!(Pd12, super::sig::I2c4Scl, 4);
    alt_fn!(Pd12, super::sig::Usart3RtsDe, 7);
    alt_fn!(Pd12, super::sig::TscG6Io3, 9);
    alt_fn!(Pd12, super::sig::Lptim2In1, 14);
    alt_fn!(Pd12, super::sig::Eventout, 15);

pin!(PD13, Pd13, GPIOD, Gpiod, _PD13, GpioPin, _GPIOD, 13);
    alt_fn!(Pd13, super::sig::I2c4Sda, 4);
    alt_fn!(Pd13, super::sig::TscG6Io4, 9);
    alt_fn!(Pd13, super::sig::Lptim2Out, 14);
    alt_fn!(Pd13, super::sig::Eventout, 15);

pin!(PD14, Pd14, GPIOD, Gpiod, _PD14, GpioPin, _GPIOD, 14);
    alt_fn!(Pd14, super::sig::Eventout, 15);

pin!(PD15, Pd15, GPIOD, Gpiod, _PD15, GpioPin, _GPIOD, 15);
    alt_fn!(Pd15, super::sig::Eventout, 15);

pin!(PE0, Pe0, GPIOE, Gpioe, _PE0, GpioPin, _GPIOE, 0);
    alt_fn!(Pe0, super::sig::Tim16Ch1, 14);
    alt_fn!(Pe0, super::sig::Eventout, 15);

pin!(PE1, Pe1, GPIOE, Gpioe, _PE1, GpioPin, _GPIOE, 1);
    alt_fn!(Pe1, super::sig::Eventout, 15);

pin!(PE2, Pe2, GPIOE, Gpioe, _PE2, GpioPin, _GPIOE, 2);
    alt_fn!(Pe2, super::sig::Traceck, 0);
    alt_fn!(Pe2, super::sig::Tim3Etr, 2);
    alt_fn!(Pe2, super::sig::TscG7Io1, 9);
    alt_fn!(Pe2, super::sig::Sai1MclkA, 13);
    alt_fn!(Pe2, super::sig::Eventout, 15);

pin!(PE3, Pe3, GPIOE, Gpioe, _PE3, GpioPin, _GPIOE, 3);
    alt_fn!(Pe3, super::sig::Traced0, 0);
    alt_fn!(Pe3, super::sig::Tim3Ch1, 2);
    alt_fn!(Pe3, super::sig::TscG7Io2, 9);
    alt_fn!(Pe3, super::sig::Sai1SdB, 13);
    alt_fn!(Pe3, super::sig::Eventout, 15);

pin!(PE4, Pe4, GPIOE, Gpioe, _PE4, GpioPin, _GPIOE, 4);
    alt_fn!(Pe4, super::sig::Traced1, 0);
    alt_fn!(Pe4, super::sig::Tim3Ch2, 2);
    alt_fn!(Pe4, super::sig::Dfsdm1Datin3, 6);
    alt_fn!(Pe4, super::sig::TscG7Io3, 9);
    alt_fn!(Pe4, super::sig::Sai1FsA, 13);
    alt_fn!(Pe4, super::sig::Eventout, 15);

pin!(PE5, Pe5, GPIOE, Gpioe, _PE5, GpioPin, _GPIOE, 5);
    alt_fn!(Pe5, super::sig::Traced2, 0);
    alt_fn!(Pe5, super::sig::Tim3Ch3, 2);
    alt_fn!(Pe5, super::sig::Dfsdm1Ckin3, 6);
    alt_fn!(Pe5, super::sig::TscG7Io4, 9);
    alt_fn!(Pe5, super::sig::Sai1SckA, 13);
    alt_fn!(Pe5, super::sig::Eventout, 15);

pin!(PE6, Pe6, GPIOE, Gpioe, _PE6, GpioPin, _GPIOE, 6);
    alt_fn!(Pe6, super::sig::Traced3, 0);
    alt_fn!(Pe6, super::sig::Tim3Ch4, 2);
    alt_fn!(Pe6, super::sig::Sai1SdA, 13);
    alt_fn!(Pe6, super::sig::Eventout, 15);

pin!(PE7, Pe7, GPIOE, Gpioe, _PE7, GpioPin, _GPIOE, 7);
    alt_fn!(Pe7, super::sig::Tim1Etr, 1);
    alt_fn!(Pe7, super::sig::Dfsdm1Datin2, 6);
    alt_fn!(Pe7, super::sig::Sai1SdB, 13);
    alt_fn!(Pe7, super::sig::Eventout, 15);

pin!(PE8, Pe8, GPIOE, Gpioe, _PE8, GpioPin, _GPIOE, 8);
    alt_fn!(Pe8, super::sig::Tim1Ch1n, 1);
    alt_fn!(Pe8, super::sig::Dfsdm1Ckin2, 6);
    alt_fn!(Pe8, super::sig::Sai1SckB, 13);
    alt_fn!(Pe8, super::sig::Eventout, 15);

pin!(PE9, Pe9, GPIOE, Gpioe, _PE9, GpioPin, _GPIOE, 9);
    alt_fn!(Pe9, super::sig::Tim1Ch1, 1);
    alt_fn!(Pe9, super::sig::Dfsdm1Ckout, 6);
    alt_fn!(Pe9, super::sig::Sai1FsB, 13);
    alt_fn!(Pe9, super::sig::Eventout, 15);

pin!(PE10, Pe10, GPIOE, Gpioe, _PE10, GpioPin, _GPIOE, 10);
    alt_fn!(Pe10, super::sig::Tim1Ch2n, 1);
    alt_fn!(Pe10, super::sig::TscG5Io1, 9);
    alt_fn!(Pe10, super::sig::QuadspiClk, 10);
    alt_fn!(Pe10, super::sig::Sai1MclkB, 13);
    alt_fn!(Pe10, super::sig::Eventout, 15);

pin!(PE11, Pe11, GPIOE, Gpioe, _PE11, GpioPin, _GPIOE, 11);
    alt_fn!(Pe11, super::sig::Tim1Ch2, 1);
    alt_fn!(Pe11, super::sig::TscG5Io2, 9);
    alt_fn!(Pe11, super::sig::QuadspiBk1Ncs, 10);
    alt_fn!(Pe11, super::sig::Eventout, 15);

pin!(PE12, Pe12, GPIOE, Gpioe, _PE12, GpioPin, _GPIOE, 12);
    alt_fn!(Pe12, super::sig::Tim1Ch3n, 1);
    alt_fn!(Pe12, super::sig::Spi1Nss, 5);
    alt_fn!(Pe12, super::sig::TscG5Io3, 9);
    alt_fn!(Pe12, super::sig::QuadspiBk1Io0, 10);
    alt_fn!(Pe12, super::sig::Eventout, 15);

pin!(PE13, Pe13, GPIOE, Gpioe, _PE13, GpioPin, _GPIOE, 13);
    alt_fn!(Pe13, super::sig::Tim1Ch3, 1);
    alt_fn!(Pe13, super::sig::Spi1Sck, 5);
    alt_fn!(Pe13, super::sig::TscG5Io4, 9);
    alt_fn!(Pe13, super::sig::QuadspiBk1Io1, 10);
    alt_fn!(Pe13, super::sig::Eventout, 15);

pin!(PE14, Pe14, GPIOE, Gpioe, _PE14, GpioPin, _GPIOE, 14);
    alt_fn!(Pe14, super::sig::Tim1Ch4, 1);
    alt_fn!(Pe14, super::sig::Tim1Bkin2, 2);
    alt_fn!(Pe14, super::sig::Tim1Bkin2Comp2, 3);
    alt_fn!(Pe14, super::sig::Spi1Miso, 5);
    alt_fn!(Pe14, super::sig::QuadspiBk1Io2, 10);
    alt_fn!(Pe14, super::sig::Eventout, 15);

pin!(PE15, Pe15, GPIOE, Gpioe, _PE15, GpioPin, _GPIOE, 15);
    alt_fn!(Pe15, super::sig::Tim1Bkin, 1);
    alt_fn!(Pe15, super::sig::Tim1BkinComp1, 3);
    alt_fn!(Pe15, super::sig::Spi1Mosi, 5);
    alt_fn!(Pe15, super::sig::QuadspiBk1Io3, 10);
    alt_fn!(Pe15, super::sig::Eventout, 15);

pin!(PH0, Ph0, GPIOH, Gpioh, _PH0, GpioPin, _GPIOH, 0);
    alt_fn!(Ph0, super::sig::Eventout, 15);

pin!(PH1, Ph1, GPIOH, Gpioh, _PH1, GpioPin, _GPIOH, 1);
    alt_fn!(Ph1, super::sig::Eventout, 15);

pin!(PH3, Ph3, GPIOH, Gpioh, _PH3, GpioPin, _GPIOH, 3);
    alt_fn!(Ph3, super::sig::Eventout, 15);


