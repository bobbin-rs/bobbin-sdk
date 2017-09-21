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
    alt_fn!(Pa0, super::sig::Adc1In5, 0);
    alt_fn!(Pa0, super::sig::Tim2Ch1, 1);
    alt_fn!(Pa0, super::sig::Usart2Cts, 7);
    alt_fn!(Pa0, super::sig::Comp1Out, 12);
    alt_fn!(Pa0, super::sig::Sai1Extclk, 13);
    alt_fn!(Pa0, super::sig::Tim2Etr, 14);
    alt_fn!(Pa0, super::sig::Eventout, 15);

pin!(PA1, Pa1, GPIOA, Gpioa, _PA1, GpioPin, _GPIOA, 1);
    alt_fn!(Pa1, super::sig::Adc1In6, 0);
    alt_fn!(Pa1, super::sig::Tim2Ch2, 1);
    alt_fn!(Pa1, super::sig::I2c1Smba, 4);
    alt_fn!(Pa1, super::sig::Spi1Sck, 5);
    alt_fn!(Pa1, super::sig::Usart2RtsDe, 7);
    alt_fn!(Pa1, super::sig::Tim15Ch1n, 14);
    alt_fn!(Pa1, super::sig::Eventout, 15);

pin!(PA2, Pa2, GPIOA, Gpioa, _PA2, GpioPin, _GPIOA, 2);
    alt_fn!(Pa2, super::sig::Adc1In7, 0);
    alt_fn!(Pa2, super::sig::Tim2Ch3, 1);
    alt_fn!(Pa2, super::sig::Usart2Tx, 7);
    alt_fn!(Pa2, super::sig::Lpuart1Tx, 8);
    alt_fn!(Pa2, super::sig::QuadspiBk1Ncs, 10);
    alt_fn!(Pa2, super::sig::Comp2Out, 12);
    alt_fn!(Pa2, super::sig::Tim15Ch1, 14);
    alt_fn!(Pa2, super::sig::Eventout, 15);

pin!(PA3, Pa3, GPIOA, Gpioa, _PA3, GpioPin, _GPIOA, 3);
    alt_fn!(Pa3, super::sig::Adc1In8, 0);
    alt_fn!(Pa3, super::sig::Tim2Ch4, 1);
    alt_fn!(Pa3, super::sig::Usart2Rx, 7);
    alt_fn!(Pa3, super::sig::Lpuart1Rx, 8);
    alt_fn!(Pa3, super::sig::QuadspiClk, 10);
    alt_fn!(Pa3, super::sig::Sai1MclkA, 13);
    alt_fn!(Pa3, super::sig::Tim15Ch2, 14);
    alt_fn!(Pa3, super::sig::Eventout, 15);

pin!(PA4, Pa4, GPIOA, Gpioa, _PA4, GpioPin, _GPIOA, 4);
    alt_fn!(Pa4, super::sig::Adc1In9, 0);
    alt_fn!(Pa4, super::sig::Spi1Nss, 5);
    alt_fn!(Pa4, super::sig::Spi3Nss, 6);
    alt_fn!(Pa4, super::sig::Usart2Ck, 7);
    alt_fn!(Pa4, super::sig::Sai1FsB, 13);
    alt_fn!(Pa4, super::sig::Lptim2Out, 14);
    alt_fn!(Pa4, super::sig::Eventout, 15);

pin!(PA5, Pa5, GPIOA, Gpioa, _PA5, GpioPin, _GPIOA, 5);
    alt_fn!(Pa5, super::sig::Adc1In10, 0);
    alt_fn!(Pa5, super::sig::Tim2Ch1, 1);
    alt_fn!(Pa5, super::sig::Tim2Etr, 2);
    alt_fn!(Pa5, super::sig::Spi1Sck, 5);
    alt_fn!(Pa5, super::sig::Lptim2Etr, 14);
    alt_fn!(Pa5, super::sig::Eventout, 15);

pin!(PA6, Pa6, GPIOA, Gpioa, _PA6, GpioPin, _GPIOA, 6);
    alt_fn!(Pa6, super::sig::Adc1In11, 0);
    alt_fn!(Pa6, super::sig::Tim1Bkin, 1);
    alt_fn!(Pa6, super::sig::Spi1Miso, 5);
    alt_fn!(Pa6, super::sig::Comp1Out, 6);
    alt_fn!(Pa6, super::sig::Usart3Cts, 7);
    alt_fn!(Pa6, super::sig::Lpuart1Cts, 8);
    alt_fn!(Pa6, super::sig::QuadspiBk1Io3, 10);
    alt_fn!(Pa6, super::sig::Tim1BkinComp2, 12);
    alt_fn!(Pa6, super::sig::Tim16Ch1, 14);
    alt_fn!(Pa6, super::sig::Eventout, 15);

pin!(PA7, Pa7, GPIOA, Gpioa, _PA7, GpioPin, _GPIOA, 7);
    alt_fn!(Pa7, super::sig::Adc1In12, 0);
    alt_fn!(Pa7, super::sig::Tim1Ch1n, 1);
    alt_fn!(Pa7, super::sig::I2c3Scl, 4);
    alt_fn!(Pa7, super::sig::Spi1Mosi, 5);
    alt_fn!(Pa7, super::sig::QuadspiBk1Io2, 10);
    alt_fn!(Pa7, super::sig::Comp2Out, 12);
    alt_fn!(Pa7, super::sig::Eventout, 15);

pin!(PA8, Pa8, GPIOA, Gpioa, _PA8, GpioPin, _GPIOA, 8);
    alt_fn!(Pa8, super::sig::Mco, 0);
    alt_fn!(Pa8, super::sig::Tim1Ch1, 1);
    alt_fn!(Pa8, super::sig::Usart1Ck, 7);
    alt_fn!(Pa8, super::sig::Swpmi1Io, 12);
    alt_fn!(Pa8, super::sig::Sai1SckA, 13);
    alt_fn!(Pa8, super::sig::Lptim2Out, 14);
    alt_fn!(Pa8, super::sig::Eventout, 15);

pin!(PA9, Pa9, GPIOA, Gpioa, _PA9, GpioPin, _GPIOA, 9);
    alt_fn!(Pa9, super::sig::Tim1Ch2, 1);
    alt_fn!(Pa9, super::sig::I2c1Scl, 4);
    alt_fn!(Pa9, super::sig::Usart1Tx, 7);
    alt_fn!(Pa9, super::sig::Sai1FsA, 13);
    alt_fn!(Pa9, super::sig::Tim15Bkin, 14);
    alt_fn!(Pa9, super::sig::Eventout, 15);

pin!(PA10, Pa10, GPIOA, Gpioa, _PA10, GpioPin, _GPIOA, 10);
    alt_fn!(Pa10, super::sig::Tim1Ch3, 1);
    alt_fn!(Pa10, super::sig::I2c1Sda, 4);
    alt_fn!(Pa10, super::sig::Usart1Rx, 7);
    alt_fn!(Pa10, super::sig::UsbCrsSync, 10);
    alt_fn!(Pa10, super::sig::Sai1SdA, 13);
    alt_fn!(Pa10, super::sig::Eventout, 15);

pin!(PA11, Pa11, GPIOA, Gpioa, _PA11, GpioPin, _GPIOA, 11);
    alt_fn!(Pa11, super::sig::Tim1Ch4, 1);
    alt_fn!(Pa11, super::sig::Tim1Bkin2, 2);
    alt_fn!(Pa11, super::sig::Spi1Miso, 5);
    alt_fn!(Pa11, super::sig::Comp1Out, 6);
    alt_fn!(Pa11, super::sig::Usart1Cts, 7);
    alt_fn!(Pa11, super::sig::Can1Rx, 9);
    alt_fn!(Pa11, super::sig::UsbDm, 10);
    alt_fn!(Pa11, super::sig::Tim1Bkin2Comp1, 12);
    alt_fn!(Pa11, super::sig::Eventout, 15);

pin!(PA12, Pa12, GPIOA, Gpioa, _PA12, GpioPin, _GPIOA, 12);
    alt_fn!(Pa12, super::sig::Tim1Etr, 1);
    alt_fn!(Pa12, super::sig::Spi1Mosi, 5);
    alt_fn!(Pa12, super::sig::Usart1RtsDe, 7);
    alt_fn!(Pa12, super::sig::Can1Tx, 9);
    alt_fn!(Pa12, super::sig::UsbDp, 10);
    alt_fn!(Pa12, super::sig::Eventout, 15);

pin!(PA13, Pa13, GPIOA, Gpioa, _PA13, GpioPin, _GPIOA, 13);
    alt_fn!(Pa13, super::sig::Jtms, 0);
    alt_fn!(Pa13, super::sig::Swdio, 0);
    alt_fn!(Pa13, super::sig::IrOut, 1);
    alt_fn!(Pa13, super::sig::UsbNoe, 10);
    alt_fn!(Pa13, super::sig::Swpmi1Tx, 12);
    alt_fn!(Pa13, super::sig::Sai1SdB, 13);
    alt_fn!(Pa13, super::sig::Eventout, 15);

pin!(PA14, Pa14, GPIOA, Gpioa, _PA14, GpioPin, _GPIOA, 14);
    alt_fn!(Pa14, super::sig::Jtck, 0);
    alt_fn!(Pa14, super::sig::Swclk, 0);
    alt_fn!(Pa14, super::sig::Lptim1Out, 1);
    alt_fn!(Pa14, super::sig::I2c1Smba, 4);
    alt_fn!(Pa14, super::sig::Swpmi1Rx, 12);
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
    alt_fn!(Pa15, super::sig::TscG3Io1, 9);
    alt_fn!(Pa15, super::sig::Swpmi1Suspend, 12);
    alt_fn!(Pa15, super::sig::Eventout, 15);

pin!(PB0, Pb0, GPIOB, Gpiob, _PB0, GpioPin, _GPIOB, 0);
    alt_fn!(Pb0, super::sig::Adc1In15, 0);
    alt_fn!(Pb0, super::sig::Tim1Ch2n, 1);
    alt_fn!(Pb0, super::sig::Spi1Nss, 5);
    alt_fn!(Pb0, super::sig::Usart3Ck, 7);
    alt_fn!(Pb0, super::sig::Lpuart1, 8);
    alt_fn!(Pb0, super::sig::Can1, 9);
    alt_fn!(Pb0, super::sig::Tsc, 9);
    alt_fn!(Pb0, super::sig::Usb, 10);
    alt_fn!(Pb0, super::sig::Quadspi, 10);
    alt_fn!(Pb0, super::sig::Comp1, 12);
    alt_fn!(Pb0, super::sig::Comp2, 12);
    alt_fn!(Pb0, super::sig::Swpmi1, 12);
    alt_fn!(Pb0, super::sig::Sai1, 13);
    alt_fn!(Pb0, super::sig::Tim2, 14);
    alt_fn!(Pb0, super::sig::Tim15, 14);
    alt_fn!(Pb0, super::sig::Tim16, 14);
    alt_fn!(Pb0, super::sig::Lptim2, 14);
    alt_fn!(Pb0, super::sig::Eventout, 15);

pin!(PB1, Pb1, GPIOB, Gpiob, _PB1, GpioPin, _GPIOB, 1);
    alt_fn!(Pb1, super::sig::Adc1In16, 0);
    alt_fn!(Pb1, super::sig::Tim1Ch3n, 1);
    alt_fn!(Pb1, super::sig::Usart3RtsDe, 7);
    alt_fn!(Pb1, super::sig::QuadspiBk1Io1, 10);
    alt_fn!(Pb1, super::sig::Comp1Out, 12);
    alt_fn!(Pb1, super::sig::Sai1Extclk, 13);
    alt_fn!(Pb1, super::sig::Eventout, 15);

pin!(PB3, Pb3, GPIOB, Gpiob, _PB3, GpioPin, _GPIOB, 3);
    alt_fn!(Pb3, super::sig::Jtdo, 0);
    alt_fn!(Pb3, super::sig::Traceswo, 0);
    alt_fn!(Pb3, super::sig::Tim2Ch2, 1);
    alt_fn!(Pb3, super::sig::Spi1Sck, 5);
    alt_fn!(Pb3, super::sig::Spi3Sck, 6);
    alt_fn!(Pb3, super::sig::Usart1RtsDe, 7);
    alt_fn!(Pb3, super::sig::Lpuart1RtsDe, 8);
    alt_fn!(Pb3, super::sig::QuadspiBk1Io0, 10);
    alt_fn!(Pb3, super::sig::Lptim2In1, 14);
    alt_fn!(Pb3, super::sig::Eventout, 15);

pin!(PB4, Pb4, GPIOB, Gpiob, _PB4, GpioPin, _GPIOB, 4);
    alt_fn!(Pb4, super::sig::Njtrst, 0);
    alt_fn!(Pb4, super::sig::I2c3Sda, 4);
    alt_fn!(Pb4, super::sig::Spi1Miso, 5);
    alt_fn!(Pb4, super::sig::Spi3Miso, 6);
    alt_fn!(Pb4, super::sig::Usart1Cts, 7);
    alt_fn!(Pb4, super::sig::Sai1SckB, 13);
    alt_fn!(Pb4, super::sig::Eventout, 15);

pin!(PB5, Pb5, GPIOB, Gpiob, _PB5, GpioPin, _GPIOB, 5);
    alt_fn!(Pb5, super::sig::Lptim1In1, 1);
    alt_fn!(Pb5, super::sig::I2c1Smba, 4);
    alt_fn!(Pb5, super::sig::Spi1Mosi, 5);
    alt_fn!(Pb5, super::sig::Spi3Mosi, 6);
    alt_fn!(Pb5, super::sig::Usart1Ck, 7);
    alt_fn!(Pb5, super::sig::TscG2Io1, 9);
    alt_fn!(Pb5, super::sig::Sai1MclkB, 13);
    alt_fn!(Pb5, super::sig::Eventout, 15);

pin!(PB6, Pb6, GPIOB, Gpiob, _PB6, GpioPin, _GPIOB, 6);
    alt_fn!(Pb6, super::sig::Lptim1Etr, 1);
    alt_fn!(Pb6, super::sig::I2c1Scl, 4);
    alt_fn!(Pb6, super::sig::Usart1Tx, 7);
    alt_fn!(Pb6, super::sig::TscG2Io2, 9);
    alt_fn!(Pb6, super::sig::Comp2Out, 12);
    alt_fn!(Pb6, super::sig::Sai1SdB, 13);
    alt_fn!(Pb6, super::sig::Tim16Bkin, 14);
    alt_fn!(Pb6, super::sig::Eventout, 15);

pin!(PB7, Pb7, GPIOB, Gpiob, _PB7, GpioPin, _GPIOB, 7);
    alt_fn!(Pb7, super::sig::Lptim1In2, 1);
    alt_fn!(Pb7, super::sig::I2c1Sda, 4);
    alt_fn!(Pb7, super::sig::Usart1Rx, 7);
    alt_fn!(Pb7, super::sig::TscG2Io3, 9);
    alt_fn!(Pb7, super::sig::Sai1FsB, 13);
    alt_fn!(Pb7, super::sig::Tim16Ch1n, 14);
    alt_fn!(Pb7, super::sig::Eventout, 15);

pin!(PC0, Pc0, GPIOC, Gpioc, _PC0, GpioPin, _GPIOC, 0);
    alt_fn!(Pc0, super::sig::Lptim1In1, 0);
    alt_fn!(Pc0, super::sig::Eventout, 2);
    alt_fn!(Pc0, super::sig::Lpuart1Rx, 6);

pin!(PC14, Pc14, GPIOC, Gpioc, _PC14, GpioPin, _GPIOC, 14);
    alt_fn!(Pc14, super::sig::TscG2Io4, 9);
    alt_fn!(Pc14, super::sig::Eventout, 15);

pin!(PC15, Pc15, GPIOC, Gpioc, _PC15, GpioPin, _GPIOC, 15);
    alt_fn!(Pc15, super::sig::Eventout, 15);

pin!(PH3, Ph3, GPIOH, Gpioh, _PH3, GpioPin, _GPIOH, 3);
    alt_fn!(Ph3, super::sig::Eventout, 15);


