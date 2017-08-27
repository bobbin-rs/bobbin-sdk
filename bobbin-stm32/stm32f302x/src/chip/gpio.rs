#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::gpio::*;

periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x48000000);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x48000400);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x48000800);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x48000c00);
periph!( GPIOF, Gpiof, _GPIOF, GpioPeriph, 0x48001400);







pin!(PA0, Pa0, GPIOA, Gpioa, _PA0, GpioPin, _GPIOA, 0);
   alt_fn!(Pa0, super::sig::Adc1In1, 0);
   alt_fn!(Pa0, super::sig::Tim2Ch1, 1);
   alt_fn!(Pa0, super::sig::Tim2Etr, 1);
   alt_fn!(Pa0, super::sig::TscG1Io1, 3);
   alt_fn!(Pa0, super::sig::Usart2Cts, 7);
   alt_fn!(Pa0, super::sig::Eventout, 15);

pin!(PA1, Pa1, GPIOA, Gpioa, _PA1, GpioPin, _GPIOA, 1);
   alt_fn!(Pa1, super::sig::Adc1In2, 0);
   alt_fn!(Pa1, super::sig::RtcRefin, 0);
   alt_fn!(Pa1, super::sig::Tim2Ch2, 1);
   alt_fn!(Pa1, super::sig::TscG1Io2, 3);
   alt_fn!(Pa1, super::sig::Usart2RtsDe, 7);
   alt_fn!(Pa1, super::sig::Tim15Ch1n, 9);
   alt_fn!(Pa1, super::sig::Eventout, 15);

pin!(PA2, Pa2, GPIOA, Gpioa, _PA2, GpioPin, _GPIOA, 2);
   alt_fn!(Pa2, super::sig::Adc1In3, 0);
   alt_fn!(Pa2, super::sig::Tim2Ch3, 1);
   alt_fn!(Pa2, super::sig::TscG1Io3, 3);
   alt_fn!(Pa2, super::sig::Usart2Tx, 7);
   alt_fn!(Pa2, super::sig::Comp2Out, 8);
   alt_fn!(Pa2, super::sig::Tim15Ch1, 9);
   alt_fn!(Pa2, super::sig::Eventout, 15);

pin!(PA3, Pa3, GPIOA, Gpioa, _PA3, GpioPin, _GPIOA, 3);
   alt_fn!(Pa3, super::sig::Adc1In4, 0);
   alt_fn!(Pa3, super::sig::Tim2Ch4, 1);
   alt_fn!(Pa3, super::sig::TscG1Io4, 3);
   alt_fn!(Pa3, super::sig::Usart2Rx, 7);
   alt_fn!(Pa3, super::sig::Tim15Ch2, 9);
   alt_fn!(Pa3, super::sig::Eventout, 15);

pin!(PA4, Pa4, GPIOA, Gpioa, _PA4, GpioPin, _GPIOA, 4);
   alt_fn!(Pa4, super::sig::Adc1In5, 0);
   alt_fn!(Pa4, super::sig::TscG2Io1, 3);
   alt_fn!(Pa4, super::sig::Spi3Nss, 6);
   alt_fn!(Pa4, super::sig::I2s3Ws, 6);
   alt_fn!(Pa4, super::sig::Usart2Ck, 7);
   alt_fn!(Pa4, super::sig::Eventout, 15);

pin!(PA5, Pa5, GPIOA, Gpioa, _PA5, GpioPin, _GPIOA, 5);
   alt_fn!(Pa5, super::sig::Tim2Ch1, 1);
   alt_fn!(Pa5, super::sig::Tim2Etr, 1);
   alt_fn!(Pa5, super::sig::TscG2Io2, 3);
   alt_fn!(Pa5, super::sig::Eventout, 15);

pin!(PA6, Pa6, GPIOA, Gpioa, _PA6, GpioPin, _GPIOA, 6);
   alt_fn!(Pa6, super::sig::Adc1In10, 0);
   alt_fn!(Pa6, super::sig::Tim16Ch1, 1);
   alt_fn!(Pa6, super::sig::TscG2Io3, 3);
   alt_fn!(Pa6, super::sig::Tim1Bkin, 6);
   alt_fn!(Pa6, super::sig::Eventout, 15);

pin!(PA7, Pa7, GPIOA, Gpioa, _PA7, GpioPin, _GPIOA, 7);
   alt_fn!(Pa7, super::sig::Adc1In15, 0);
   alt_fn!(Pa7, super::sig::Tim17Ch1, 1);
   alt_fn!(Pa7, super::sig::TscG2Io4, 3);
   alt_fn!(Pa7, super::sig::Tim1Ch1n, 6);
   alt_fn!(Pa7, super::sig::Eventout, 15);

pin!(PA8, Pa8, GPIOA, Gpioa, _PA8, GpioPin, _GPIOA, 8);
   alt_fn!(Pa8, super::sig::Mco, 0);
   alt_fn!(Pa8, super::sig::I2c3Scl, 3);
   alt_fn!(Pa8, super::sig::I2c2Smbal, 4);
   alt_fn!(Pa8, super::sig::I2s2Mck, 5);
   alt_fn!(Pa8, super::sig::Tim1Ch1, 6);
   alt_fn!(Pa8, super::sig::Usart1Ck, 7);
   alt_fn!(Pa8, super::sig::Eventout, 15);

pin!(PA9, Pa9, GPIOA, Gpioa, _PA9, GpioPin, _GPIOA, 9);
   alt_fn!(Pa9, super::sig::I2c3Smbal, 2);
   alt_fn!(Pa9, super::sig::TscG4Io1, 3);
   alt_fn!(Pa9, super::sig::I2c2Scl, 4);
   alt_fn!(Pa9, super::sig::I2s3Mck, 5);
   alt_fn!(Pa9, super::sig::Tim1Ch2, 6);
   alt_fn!(Pa9, super::sig::Usart1Tx, 7);
   alt_fn!(Pa9, super::sig::Tim15Bkin, 9);
   alt_fn!(Pa9, super::sig::Tim2Ch3, 10);
   alt_fn!(Pa9, super::sig::Eventout, 15);

pin!(PA10, Pa10, GPIOA, Gpioa, _PA10, GpioPin, _GPIOA, 10);
   alt_fn!(Pa10, super::sig::Tim17Bkin, 1);
   alt_fn!(Pa10, super::sig::TscG4Io2, 3);
   alt_fn!(Pa10, super::sig::I2c2Sda, 4);
   alt_fn!(Pa10, super::sig::Spi2Miso, 5);
   alt_fn!(Pa10, super::sig::I2s2extSd, 5);
   alt_fn!(Pa10, super::sig::Tim1Ch3, 6);
   alt_fn!(Pa10, super::sig::Usart1Rx, 7);
   alt_fn!(Pa10, super::sig::Comp6Out, 8);
   alt_fn!(Pa10, super::sig::Tim2Ch4, 10);
   alt_fn!(Pa10, super::sig::Eventout, 15);

pin!(PA11, Pa11, GPIOA, Gpioa, _PA11, GpioPin, _GPIOA, 11);
   alt_fn!(Pa11, super::sig::Spi2Mosi, 5);
   alt_fn!(Pa11, super::sig::I2s2Sd, 5);
   alt_fn!(Pa11, super::sig::Tim1Ch1n, 6);
   alt_fn!(Pa11, super::sig::Usart1Cts, 7);
   alt_fn!(Pa11, super::sig::CanRx, 9);
   alt_fn!(Pa11, super::sig::Tim1Ch4, 11);
   alt_fn!(Pa11, super::sig::Tim1Bkin2, 12);
   alt_fn!(Pa11, super::sig::Eventout, 15);

pin!(PA12, Pa12, GPIOA, Gpioa, _PA12, GpioPin, _GPIOA, 12);
   alt_fn!(Pa12, super::sig::Tim16Ch1, 1);
   alt_fn!(Pa12, super::sig::I2sckin, 5);
   alt_fn!(Pa12, super::sig::Tim1Ch2n, 6);
   alt_fn!(Pa12, super::sig::Usart1RtsDe, 7);
   alt_fn!(Pa12, super::sig::Comp2Out, 8);
   alt_fn!(Pa12, super::sig::CanTx, 9);
   alt_fn!(Pa12, super::sig::Tim1Etr, 11);
   alt_fn!(Pa12, super::sig::Eventout, 15);

pin!(PA13, Pa13, GPIOA, Gpioa, _PA13, GpioPin, _GPIOA, 13);
   alt_fn!(Pa13, super::sig::Swdat, 0);
   alt_fn!(Pa13, super::sig::Jtms, 0);
   alt_fn!(Pa13, super::sig::Tim16Ch1n, 1);
   alt_fn!(Pa13, super::sig::TscG4Io3, 3);
   alt_fn!(Pa13, super::sig::IrOut, 5);
   alt_fn!(Pa13, super::sig::Usart3Cts, 7);
   alt_fn!(Pa13, super::sig::Eventout, 15);

pin!(PA14, Pa14, GPIOA, Gpioa, _PA14, GpioPin, _GPIOA, 14);
   alt_fn!(Pa14, super::sig::Swclk, 0);
   alt_fn!(Pa14, super::sig::Jtck, 0);
   alt_fn!(Pa14, super::sig::TscG4Io4, 3);
   alt_fn!(Pa14, super::sig::I2c1Sda, 4);
   alt_fn!(Pa14, super::sig::Tim1Bkin, 6);
   alt_fn!(Pa14, super::sig::Usart2Tx, 7);
   alt_fn!(Pa14, super::sig::Eventout, 15);

pin!(PA15, Pa15, GPIOA, Gpioa, _PA15, GpioPin, _GPIOA, 15);
   alt_fn!(Pa15, super::sig::Jtdi, 0);
   alt_fn!(Pa15, super::sig::Tim2Ch1, 1);
   alt_fn!(Pa15, super::sig::Tim2Etr, 1);
   alt_fn!(Pa15, super::sig::TscSync, 3);
   alt_fn!(Pa15, super::sig::I2c1Scl, 4);
   alt_fn!(Pa15, super::sig::Spi3Nss, 6);
   alt_fn!(Pa15, super::sig::I2s3Ws, 6);
   alt_fn!(Pa15, super::sig::Usart2Rx, 7);
   alt_fn!(Pa15, super::sig::Tim1Bkin, 9);
   alt_fn!(Pa15, super::sig::Eventout, 15);

pin!(PB0, Pb0, GPIOB, Gpiob, _PB0, GpioPin, _GPIOB, 0);
   alt_fn!(Pb0, super::sig::Adc1In11, 0);
   alt_fn!(Pb0, super::sig::TscG3Io2, 3);
   alt_fn!(Pb0, super::sig::Tim1Ch2n, 6);
   alt_fn!(Pb0, super::sig::Eventout, 15);

pin!(PB1, Pb1, GPIOB, Gpiob, _PB1, GpioPin, _GPIOB, 1);
   alt_fn!(Pb1, super::sig::Adc1In12, 0);
   alt_fn!(Pb1, super::sig::TscG3Io3, 3);
   alt_fn!(Pb1, super::sig::Tim1Ch3n, 6);
   alt_fn!(Pb1, super::sig::Comp4Out, 8);
   alt_fn!(Pb1, super::sig::Eventout, 15);

pin!(PB2, Pb2, GPIOB, Gpiob, _PB2, GpioPin, _GPIOB, 2);
   alt_fn!(Pb2, super::sig::TscG3Io4, 3);
   alt_fn!(Pb2, super::sig::Eventout, 15);

pin!(PB3, Pb3, GPIOB, Gpiob, _PB3, GpioPin, _GPIOB, 3);
   alt_fn!(Pb3, super::sig::Jtdo, 0);
   alt_fn!(Pb3, super::sig::Traceswo, 0);
   alt_fn!(Pb3, super::sig::Tim2Ch2, 1);
   alt_fn!(Pb3, super::sig::TscG5Io1, 3);
   alt_fn!(Pb3, super::sig::Spi3Sck, 6);
   alt_fn!(Pb3, super::sig::I2s3Ck, 6);
   alt_fn!(Pb3, super::sig::Usart2Tx, 7);
   alt_fn!(Pb3, super::sig::Eventout, 15);

pin!(PB4, Pb4, GPIOB, Gpiob, _PB4, GpioPin, _GPIOB, 4);
   alt_fn!(Pb4, super::sig::Jtrst, 0);
   alt_fn!(Pb4, super::sig::Tim16Ch1, 1);
   alt_fn!(Pb4, super::sig::TscG5Io2, 3);
   alt_fn!(Pb4, super::sig::Spi3Miso, 6);
   alt_fn!(Pb4, super::sig::I2s3Sd, 6);
   alt_fn!(Pb4, super::sig::Usart2Rx, 7);
   alt_fn!(Pb4, super::sig::Tim17Bkin, 10);
   alt_fn!(Pb4, super::sig::Eventout, 15);

pin!(PB5, Pb5, GPIOB, Gpiob, _PB5, GpioPin, _GPIOB, 5);
   alt_fn!(Pb5, super::sig::Tim16Bkin, 1);
   alt_fn!(Pb5, super::sig::I2c1Smbal, 4);
   alt_fn!(Pb5, super::sig::Spi3Mosi, 6);
   alt_fn!(Pb5, super::sig::I2s3extSd, 6);
   alt_fn!(Pb5, super::sig::Usart2Ck, 7);
   alt_fn!(Pb5, super::sig::I2c3Sda, 8);
   alt_fn!(Pb5, super::sig::Tim17Ch1, 10);
   alt_fn!(Pb5, super::sig::Eventout, 15);

pin!(PB6, Pb6, GPIOB, Gpiob, _PB6, GpioPin, _GPIOB, 6);
   alt_fn!(Pb6, super::sig::Tim16Ch1n, 1);
   alt_fn!(Pb6, super::sig::TscG5Io3, 3);
   alt_fn!(Pb6, super::sig::I2c1Scl, 4);
   alt_fn!(Pb6, super::sig::Usart1Tx, 7);
   alt_fn!(Pb6, super::sig::Eventout, 15);

pin!(PB7, Pb7, GPIOB, Gpiob, _PB7, GpioPin, _GPIOB, 7);
   alt_fn!(Pb7, super::sig::Tim17Ch1n, 1);
   alt_fn!(Pb7, super::sig::TscG5Io4, 3);
   alt_fn!(Pb7, super::sig::I2c1Sda, 4);
   alt_fn!(Pb7, super::sig::Usart1Rx, 7);
   alt_fn!(Pb7, super::sig::Eventout, 15);

pin!(PB8, Pb8, GPIOB, Gpiob, _PB8, GpioPin, _GPIOB, 8);
   alt_fn!(Pb8, super::sig::Tim16Ch1, 1);
   alt_fn!(Pb8, super::sig::TscSync, 3);
   alt_fn!(Pb8, super::sig::I2c1Scl, 4);
   alt_fn!(Pb8, super::sig::Usart3Rx, 7);
   alt_fn!(Pb8, super::sig::CanRx, 9);
   alt_fn!(Pb8, super::sig::Tim1Bkin, 12);
   alt_fn!(Pb8, super::sig::Eventout, 15);

pin!(PB9, Pb9, GPIOB, Gpiob, _PB9, GpioPin, _GPIOB, 9);
   alt_fn!(Pb9, super::sig::Tim17Ch1, 1);
   alt_fn!(Pb9, super::sig::I2c1Sda, 4);
   alt_fn!(Pb9, super::sig::IrOut, 6);
   alt_fn!(Pb9, super::sig::Usart3Tx, 7);
   alt_fn!(Pb9, super::sig::Comp2Out, 8);
   alt_fn!(Pb9, super::sig::CanTx, 9);
   alt_fn!(Pb9, super::sig::Eventout, 15);

pin!(PB10, Pb10, GPIOB, Gpiob, _PB10, GpioPin, _GPIOB, 10);
   alt_fn!(Pb10, super::sig::Tim2Ch3, 1);
   alt_fn!(Pb10, super::sig::TscSync, 3);
   alt_fn!(Pb10, super::sig::Usart3Tx, 7);
   alt_fn!(Pb10, super::sig::Eventout, 15);

pin!(PB11, Pb11, GPIOB, Gpiob, _PB11, GpioPin, _GPIOB, 11);
   alt_fn!(Pb11, super::sig::Adc1In11, 0);
   alt_fn!(Pb11, super::sig::Tim2Ch4, 1);
   alt_fn!(Pb11, super::sig::TscG6Io1, 3);
   alt_fn!(Pb11, super::sig::Usart3Rx, 7);
   alt_fn!(Pb11, super::sig::Eventout, 15);

pin!(PB12, Pb12, GPIOB, Gpiob, _PB12, GpioPin, _GPIOB, 12);
   alt_fn!(Pb12, super::sig::TscG6Io2, 3);
   alt_fn!(Pb12, super::sig::I2c2Smbal, 4);
   alt_fn!(Pb12, super::sig::Spi2Nss, 5);
   alt_fn!(Pb12, super::sig::I2s2Ws, 5);
   alt_fn!(Pb12, super::sig::Tim1Bkin, 6);
   alt_fn!(Pb12, super::sig::Usart3Ck, 7);
   alt_fn!(Pb12, super::sig::Eventout, 15);

pin!(PB13, Pb13, GPIOB, Gpiob, _PB13, GpioPin, _GPIOB, 13);
   alt_fn!(Pb13, super::sig::Adc1In13, 0);
   alt_fn!(Pb13, super::sig::TscG6Io3, 3);
   alt_fn!(Pb13, super::sig::Spi2Sck, 5);
   alt_fn!(Pb13, super::sig::I2s2Ck, 5);
   alt_fn!(Pb13, super::sig::Tim1Ch1n, 6);
   alt_fn!(Pb13, super::sig::Usart3Cts, 7);
   alt_fn!(Pb13, super::sig::Eventout, 15);

pin!(PB14, Pb14, GPIOB, Gpiob, _PB14, GpioPin, _GPIOB, 14);
   alt_fn!(Pb14, super::sig::Tim15Ch1, 1);
   alt_fn!(Pb14, super::sig::TscG6Io4, 3);
   alt_fn!(Pb14, super::sig::Spi2Miso, 5);
   alt_fn!(Pb14, super::sig::I2s2extSd, 5);
   alt_fn!(Pb14, super::sig::Tim1Ch2n, 6);
   alt_fn!(Pb14, super::sig::Usart3RtsDe, 7);
   alt_fn!(Pb14, super::sig::Eventout, 15);

pin!(PB15, Pb15, GPIOB, Gpiob, _PB15, GpioPin, _GPIOB, 15);
   alt_fn!(Pb15, super::sig::RtcRefin, 0);
   alt_fn!(Pb15, super::sig::Tim15Ch2, 1);
   alt_fn!(Pb15, super::sig::Tim15Ch1n, 2);
   alt_fn!(Pb15, super::sig::Tim1Ch3n, 4);
   alt_fn!(Pb15, super::sig::Spi2Mosi, 5);
   alt_fn!(Pb15, super::sig::I2s2Sd, 5);
   alt_fn!(Pb15, super::sig::Eventout, 15);

pin!(PC0, Pc0, GPIOC, Gpioc, _PC0, GpioPin, _GPIOC, 0);
   alt_fn!(Pc0, super::sig::Eventout, 1);
   alt_fn!(Pc0, super::sig::Tim1Ch1, 2);

pin!(PC1, Pc1, GPIOC, Gpioc, _PC1, GpioPin, _GPIOC, 1);
   alt_fn!(Pc1, super::sig::Eventout, 1);
   alt_fn!(Pc1, super::sig::Tim1Ch2, 2);

pin!(PC2, Pc2, GPIOC, Gpioc, _PC2, GpioPin, _GPIOC, 2);
   alt_fn!(Pc2, super::sig::Eventout, 1);
   alt_fn!(Pc2, super::sig::Tim1Ch3, 2);

pin!(PC3, Pc3, GPIOC, Gpioc, _PC3, GpioPin, _GPIOC, 3);
   alt_fn!(Pc3, super::sig::Eventout, 1);
   alt_fn!(Pc3, super::sig::Tim1Ch4, 2);
   alt_fn!(Pc3, super::sig::Tim1Bkin2, 6);

pin!(PC4, Pc4, GPIOC, Gpioc, _PC4, GpioPin, _GPIOC, 4);
   alt_fn!(Pc4, super::sig::Eventout, 1);
   alt_fn!(Pc4, super::sig::Tim1Etr, 2);
   alt_fn!(Pc4, super::sig::Usart1Tx, 7);

pin!(PC5, Pc5, GPIOC, Gpioc, _PC5, GpioPin, _GPIOC, 5);
   alt_fn!(Pc5, super::sig::Eventout, 1);
   alt_fn!(Pc5, super::sig::Tim15Bkin, 2);
   alt_fn!(Pc5, super::sig::TscG3Io1, 3);
   alt_fn!(Pc5, super::sig::Usart1Rx, 7);

pin!(PC6, Pc6, GPIOC, Gpioc, _PC6, GpioPin, _GPIOC, 6);
   alt_fn!(Pc6, super::sig::Eventout, 1);
   alt_fn!(Pc6, super::sig::I2s2Mck, 6);
   alt_fn!(Pc6, super::sig::Comp6Out, 7);

pin!(PC7, Pc7, GPIOC, Gpioc, _PC7, GpioPin, _GPIOC, 7);
   alt_fn!(Pc7, super::sig::Eventout, 1);
   alt_fn!(Pc7, super::sig::I2s3Mck, 6);

pin!(PC8, Pc8, GPIOC, Gpioc, _PC8, GpioPin, _GPIOC, 8);
   alt_fn!(Pc8, super::sig::Eventout, 1);

pin!(PC9, Pc9, GPIOC, Gpioc, _PC9, GpioPin, _GPIOC, 9);
   alt_fn!(Pc9, super::sig::Eventout, 1);
   alt_fn!(Pc9, super::sig::I2c3Sda, 3);
   alt_fn!(Pc9, super::sig::I2sckin, 5);

pin!(PC10, Pc10, GPIOC, Gpioc, _PC10, GpioPin, _GPIOC, 10);
   alt_fn!(Pc10, super::sig::Eventout, 1);
   alt_fn!(Pc10, super::sig::Spi3Sck, 6);
   alt_fn!(Pc10, super::sig::I2s3Ck, 6);
   alt_fn!(Pc10, super::sig::Usart3Tx, 7);

pin!(PC11, Pc11, GPIOC, Gpioc, _PC11, GpioPin, _GPIOC, 11);
   alt_fn!(Pc11, super::sig::Eventout, 1);
   alt_fn!(Pc11, super::sig::Spi3Miso, 6);
   alt_fn!(Pc11, super::sig::I2s3extSd, 6);
   alt_fn!(Pc11, super::sig::Usart3Rx, 7);

pin!(PC12, Pc12, GPIOC, Gpioc, _PC12, GpioPin, _GPIOC, 12);
   alt_fn!(Pc12, super::sig::Eventout, 1);
   alt_fn!(Pc12, super::sig::Spi3Mosi, 6);
   alt_fn!(Pc12, super::sig::I2s3Sd, 6);
   alt_fn!(Pc12, super::sig::Usart3Ck, 7);

pin!(PC13, Pc13, GPIOC, Gpioc, _PC13, GpioPin, _GPIOC, 13);
   alt_fn!(Pc13, super::sig::Tim1Ch1n, 4);

pin!(PC14, Pc14, GPIOC, Gpioc, _PC14, GpioPin, _GPIOC, 14);

pin!(PC15, Pc15, GPIOC, Gpioc, _PC15, GpioPin, _GPIOC, 15);

pin!(PD2, Pd2, GPIOD, Gpiod, _PD2, GpioPin, _GPIOD, 2);
   alt_fn!(Pd2, super::sig::Eventout, 1);

pin!(PF0, Pf0, GPIOF, Gpiof, _PF0, GpioPin, _GPIOF, 0);
   alt_fn!(Pf0, super::sig::I2c2Sda, 4);
   alt_fn!(Pf0, super::sig::Spi2Nss, 5);
   alt_fn!(Pf0, super::sig::I2s2Ws, 5);
   alt_fn!(Pf0, super::sig::Tim1Ch3n, 6);

pin!(PF1, Pf1, GPIOF, Gpiof, _PF1, GpioPin, _GPIOF, 1);
   alt_fn!(Pf1, super::sig::I2c2Scl, 4);
   alt_fn!(Pf1, super::sig::Spi2Sck, 5);
   alt_fn!(Pf1, super::sig::I2s2Ck, 5);

