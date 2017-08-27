#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::gpio::*;

periph!(_GPIOA, GpioPeriph, GPIOA, Gpioa, 0x50000000);
periph!(_GPIOB, GpioPeriph, GPIOB, Gpiob, 0x50000400);
periph!(_GPIOC, GpioPeriph, GPIOC, Gpioc, 0x50000800);
periph!(_GPIOH, GpioPeriph, GPIOH, Gpioh, 0x50001c00);






pin!(PA0, Pa0, GPIOA, Gpioa, 0);
   alt_fn!(Pa0, super::sig::Adc1In0, 0);
   alt_fn!(Pa0, super::sig::Lptim1In1, 1);
   alt_fn!(Pa0, super::sig::Tim2Ch1, 2);
   alt_fn!(Pa0, super::sig::Usart2Cts, 4);
   alt_fn!(Pa0, super::sig::Tim2Etr, 5);
   alt_fn!(Pa0, super::sig::Comp1Out, 7);

pin!(PA1, Pa1, GPIOA, Gpioa, 1);
   alt_fn!(Pa1, super::sig::Adc1In1, 0);
   alt_fn!(Pa1, super::sig::Eventout, 0);
   alt_fn!(Pa1, super::sig::Lptim1In2, 1);
   alt_fn!(Pa1, super::sig::Tim2Ch2, 2);
   alt_fn!(Pa1, super::sig::I2c1Smba, 3);
   alt_fn!(Pa1, super::sig::Usart2Rts, 4);
   alt_fn!(Pa1, super::sig::Tim21Etr, 5);

pin!(PA2, Pa2, GPIOA, Gpioa, 2);
   alt_fn!(Pa2, super::sig::Adc1In2, 0);
   alt_fn!(Pa2, super::sig::Tim21Ch1, 0);
   alt_fn!(Pa2, super::sig::Tim2Ch3, 2);
   alt_fn!(Pa2, super::sig::Usart2Tx, 4);
   alt_fn!(Pa2, super::sig::Lpuart1Tx, 6);
   alt_fn!(Pa2, super::sig::Comp2Out, 7);

pin!(PA3, Pa3, GPIOA, Gpioa, 3);
   alt_fn!(Pa3, super::sig::Adc1In3, 0);
   alt_fn!(Pa3, super::sig::Tim21Ch2, 0);
   alt_fn!(Pa3, super::sig::Tim2Ch4, 2);
   alt_fn!(Pa3, super::sig::Usart2Rx, 4);
   alt_fn!(Pa3, super::sig::Lpuart1Rx, 6);

pin!(PA4, Pa4, GPIOA, Gpioa, 4);
   alt_fn!(Pa4, super::sig::Adc1In4, 0);
   alt_fn!(Pa4, super::sig::Spi1Nss, 0);
   alt_fn!(Pa4, super::sig::Lptim1In1, 1);
   alt_fn!(Pa4, super::sig::Uart2Ck, 4);
   alt_fn!(Pa4, super::sig::Tim22Etr, 5);

pin!(PA5, Pa5, GPIOA, Gpioa, 5);
   alt_fn!(Pa5, super::sig::Adc1In5, 0);
   alt_fn!(Pa5, super::sig::Spi1Sck, 0);
   alt_fn!(Pa5, super::sig::Lptim1In2, 1);
   alt_fn!(Pa5, super::sig::Tim2Etr, 3);
   alt_fn!(Pa5, super::sig::Tim2Ch1, 5);

pin!(PA6, Pa6, GPIOA, Gpioa, 6);
   alt_fn!(Pa6, super::sig::Adc1In6, 0);
   alt_fn!(Pa6, super::sig::SpiMiso, 0);
   alt_fn!(Pa6, super::sig::Lptim1Etr, 1);
   alt_fn!(Pa6, super::sig::Lpuart1Cts, 4);
   alt_fn!(Pa6, super::sig::Tim22Ch1, 5);
   alt_fn!(Pa6, super::sig::Eventout, 6);
   alt_fn!(Pa6, super::sig::Comp1Out, 7);

pin!(PA7, Pa7, GPIOA, Gpioa, 7);
   alt_fn!(Pa7, super::sig::Adc1In7, 0);
   alt_fn!(Pa7, super::sig::Spi1Mosi, 0);
   alt_fn!(Pa7, super::sig::Lptim1Out, 1);
   alt_fn!(Pa7, super::sig::Usart2Cts, 4);
   alt_fn!(Pa7, super::sig::Tim22Ch2, 5);
   alt_fn!(Pa7, super::sig::Eventout, 6);
   alt_fn!(Pa7, super::sig::Comp2Out, 7);

pin!(PA8, Pa8, GPIOA, Gpioa, 8);
   alt_fn!(Pa8, super::sig::Adc1In8, 0);
   alt_fn!(Pa8, super::sig::Mco, 0);
   alt_fn!(Pa8, super::sig::Lptim1In1, 2);
   alt_fn!(Pa8, super::sig::Eventout, 3);
   alt_fn!(Pa8, super::sig::Usart2Ck, 4);
   alt_fn!(Pa8, super::sig::Tim2Ch1, 5);

pin!(PA9, Pa9, GPIOA, Gpioa, 9);
   alt_fn!(Pa9, super::sig::Adc1In9, 0);
   alt_fn!(Pa9, super::sig::Mco, 0);
   alt_fn!(Pa9, super::sig::I2c1Scl, 1);
   alt_fn!(Pa9, super::sig::Usart2Tx, 4);
   alt_fn!(Pa9, super::sig::Tim22Ch1, 5);

pin!(PA10, Pa10, GPIOA, Gpioa, 10);
   alt_fn!(Pa10, super::sig::I2c1Sda, 1);
   alt_fn!(Pa10, super::sig::Usart2Rx, 4);
   alt_fn!(Pa10, super::sig::Tim22Ch2, 5);

pin!(PA11, Pa11, GPIOA, Gpioa, 11);
   alt_fn!(Pa11, super::sig::Spi1Mio, 0);
   alt_fn!(Pa11, super::sig::Eventout, 2);
   alt_fn!(Pa11, super::sig::Usart2Cts, 4);
   alt_fn!(Pa11, super::sig::Tim21Ch2, 5);
   alt_fn!(Pa11, super::sig::Comp1Out, 7);

pin!(PA12, Pa12, GPIOA, Gpioa, 12);
   alt_fn!(Pa12, super::sig::Spi1Mosi, 0);
   alt_fn!(Pa12, super::sig::Eventout, 2);
   alt_fn!(Pa12, super::sig::Usart2Rts, 4);
   alt_fn!(Pa12, super::sig::Comp2Out, 7);

pin!(PA13, Pa13, GPIOA, Gpioa, 13);
   alt_fn!(Pa13, super::sig::Swdio, 0);
   alt_fn!(Pa13, super::sig::Lptim1Etr, 1);
   alt_fn!(Pa13, super::sig::Lpuart1Rx, 6);

pin!(PA14, Pa14, GPIOA, Gpioa, 14);
   alt_fn!(Pa14, super::sig::Swclk, 0);
   alt_fn!(Pa14, super::sig::Lptim1Out, 1);
   alt_fn!(Pa14, super::sig::I2c1Smba, 3);
   alt_fn!(Pa14, super::sig::Usart2Tx, 4);
   alt_fn!(Pa14, super::sig::Lpuart1Tx, 6);

pin!(PA15, Pa15, GPIOA, Gpioa, 15);
   alt_fn!(Pa15, super::sig::Spi1Nss, 0);
   alt_fn!(Pa15, super::sig::Tim2Etr, 2);
   alt_fn!(Pa15, super::sig::Eventout, 3);
   alt_fn!(Pa15, super::sig::Usart2Rx, 4);
   alt_fn!(Pa15, super::sig::Tim2Ch1, 5);

pin!(PB0, Pb0, GPIOB, Gpiob, 0);
   alt_fn!(Pb0, super::sig::Eventout, 0);
   alt_fn!(Pb0, super::sig::Spi1Miso, 1);
   alt_fn!(Pb0, super::sig::Usart2Rts, 4);
   alt_fn!(Pb0, super::sig::Tim2Ch3, 5);

pin!(PB1, Pb1, GPIOB, Gpiob, 1);
   alt_fn!(Pb1, super::sig::Usart2Ck, 0);
   alt_fn!(Pb1, super::sig::Spi1Mosi, 1);
   alt_fn!(Pb1, super::sig::Lpuart1Rts, 4);
   alt_fn!(Pb1, super::sig::Tim2Ch4, 5);

pin!(PB2, Pb2, GPIOB, Gpiob, 2);
   alt_fn!(Pb2, super::sig::Lptim1Out, 2);

pin!(PB3, Pb3, GPIOB, Gpiob, 3);
   alt_fn!(Pb3, super::sig::Spi1Sck, 0);
   alt_fn!(Pb3, super::sig::Tim2Ch2, 2);
   alt_fn!(Pb3, super::sig::Eventout, 4);

pin!(PB4, Pb4, GPIOB, Gpiob, 4);
   alt_fn!(Pb4, super::sig::Spi1Miso, 0);
   alt_fn!(Pb4, super::sig::Eventout, 2);
   alt_fn!(Pb4, super::sig::Tim22Ch1, 4);

pin!(PB5, Pb5, GPIOB, Gpiob, 5);
   alt_fn!(Pb5, super::sig::Spi1Mosi, 0);
   alt_fn!(Pb5, super::sig::Lptim1In1, 2);
   alt_fn!(Pb5, super::sig::I2c1Smba, 3);
   alt_fn!(Pb5, super::sig::Tim22Ch2, 4);

pin!(PB6, Pb6, GPIOB, Gpiob, 6);
   alt_fn!(Pb6, super::sig::Usart2Tx, 0);
   alt_fn!(Pb6, super::sig::I2c1Scl, 1);
   alt_fn!(Pb6, super::sig::Lptim1Etr, 2);
   alt_fn!(Pb6, super::sig::Tim21Ch1, 5);

pin!(PB7, Pb7, GPIOB, Gpiob, 7);
   alt_fn!(Pb7, super::sig::Usart2Rx, 0);
   alt_fn!(Pb7, super::sig::I2c1Sda, 1);
   alt_fn!(Pb7, super::sig::Lptim1In2, 2);

pin!(PB8, Pb8, GPIOB, Gpiob, 8);
   alt_fn!(Pb8, super::sig::I2c1Scl, 4);

pin!(PB9, Pb9, GPIOB, Gpiob, 9);
   alt_fn!(Pb9, super::sig::Eventout, 2);
   alt_fn!(Pb9, super::sig::I2c1Sda, 4);

pin!(PB10, Pb10, GPIOB, Gpiob, 10);
   alt_fn!(Pb10, super::sig::Tim2Ch3, 2);
   alt_fn!(Pb10, super::sig::Lpuart1Tx, 6);

pin!(PB11, Pb11, GPIOB, Gpiob, 11);
   alt_fn!(Pb11, super::sig::Eventout, 0);
   alt_fn!(Pb11, super::sig::Tim2Ch4, 2);
   alt_fn!(Pb11, super::sig::Lpuart1Rx, 6);

pin!(PB12, Pb12, GPIOB, Gpiob, 12);
   alt_fn!(Pb12, super::sig::Spi1Nss, 0);
   alt_fn!(Pb12, super::sig::Eventout, 6);

pin!(PB13, Pb13, GPIOB, Gpiob, 13);
   alt_fn!(Pb13, super::sig::SpiSck, 0);
   alt_fn!(Pb13, super::sig::Mco, 1);
   alt_fn!(Pb13, super::sig::Tim21Ch1, 5);
   alt_fn!(Pb13, super::sig::Lpuart1Cts, 6);

pin!(PB14, Pb14, GPIOB, Gpiob, 14);
   alt_fn!(Pb14, super::sig::Spi1Miso, 0);
   alt_fn!(Pb14, super::sig::RtcOut, 2);
   alt_fn!(Pb14, super::sig::Tim21Ch2, 5);
   alt_fn!(Pb14, super::sig::Lpuart1Rts, 6);

pin!(PB15, Pb15, GPIOB, Gpiob, 15);
   alt_fn!(Pb15, super::sig::Spi1Mosi, 0);
   alt_fn!(Pb15, super::sig::RtcRefin, 2);

pin!(PC0, Pc0, GPIOC, Gpioc, 0);
   alt_fn!(Pc0, super::sig::Lptim1In1, 0);
   alt_fn!(Pc0, super::sig::Eventout, 2);
   alt_fn!(Pc0, super::sig::Lpuart1Rx, 6);

pin!(PC14, Pc14, GPIOC, Gpioc, 14);

pin!(PC15, Pc15, GPIOC, Gpioc, 15);

pin!(PH0, Ph0, GPIOH, Gpioh, 0);

pin!(PH1, Ph1, GPIOH, Gpioh, 1);

