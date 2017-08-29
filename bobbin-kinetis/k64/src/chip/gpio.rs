#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::gpio::*;

pub trait LinkPort<T> {
   fn port(&self) -> T;
}

periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x400ff000);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x400ff040);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x400ff080);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x400ff0c0);
periph!( GPIOE, Gpioe, _GPIOE, GpioPeriph, 0x400ff100);

impl LinkPort<super::port::Porta > for Gpioa {
   fn port(&self) -> super::port::Porta  { super::port::PORTA }
}


impl LinkPort<super::port::Portb > for Gpiob {
   fn port(&self) -> super::port::Portb  { super::port::PORTB }
}


impl LinkPort<super::port::Portc > for Gpioc {
   fn port(&self) -> super::port::Portc  { super::port::PORTC }
}


impl LinkPort<super::port::Portd > for Gpiod {
   fn port(&self) -> super::port::Portd  { super::port::PORTD }
}


impl LinkPort<super::port::Porte > for Gpioe {
   fn port(&self) -> super::port::Porte  { super::port::PORTE }
}



pin!(PA0, Pa0, GPIOA, Gpioa, _PA0, GpioPin, _GPIOA, 0);

pin!(PA1, Pa1, GPIOA, Gpioa, _PA1, GpioPin, _GPIOA, 1);

pin!(PA2, Pa2, GPIOA, Gpioa, _PA2, GpioPin, _GPIOA, 2);

pin!(PA3, Pa3, GPIOA, Gpioa, _PA3, GpioPin, _GPIOA, 3);

pin!(PA4, Pa4, GPIOA, Gpioa, _PA4, GpioPin, _GPIOA, 4);

pin!(PA5, Pa5, GPIOA, Gpioa, _PA5, GpioPin, _GPIOA, 5);

pin!(PA6, Pa6, GPIOA, Gpioa, _PA6, GpioPin, _GPIOA, 6);

pin!(PA7, Pa7, GPIOA, Gpioa, _PA7, GpioPin, _GPIOA, 7);

pin!(PA8, Pa8, GPIOA, Gpioa, _PA8, GpioPin, _GPIOA, 8);

pin!(PA9, Pa9, GPIOA, Gpioa, _PA9, GpioPin, _GPIOA, 9);

pin!(PA10, Pa10, GPIOA, Gpioa, _PA10, GpioPin, _GPIOA, 10);

pin!(PA11, Pa11, GPIOA, Gpioa, _PA11, GpioPin, _GPIOA, 11);

pin!(PA12, Pa12, GPIOA, Gpioa, _PA12, GpioPin, _GPIOA, 12);

pin!(PA13, Pa13, GPIOA, Gpioa, _PA13, GpioPin, _GPIOA, 13);

pin!(PA14, Pa14, GPIOA, Gpioa, _PA14, GpioPin, _GPIOA, 14);

pin!(PA15, Pa15, GPIOA, Gpioa, _PA15, GpioPin, _GPIOA, 15);

pin!(PA16, Pa16, GPIOA, Gpioa, _PA16, GpioPin, _GPIOA, 16);

pin!(PA17, Pa17, GPIOA, Gpioa, _PA17, GpioPin, _GPIOA, 17);

pin!(PA18, Pa18, GPIOA, Gpioa, _PA18, GpioPin, _GPIOA, 18);

pin!(PA19, Pa19, GPIOA, Gpioa, _PA19, GpioPin, _GPIOA, 19);

pin!(PA20, Pa20, GPIOA, Gpioa, _PA20, GpioPin, _GPIOA, 20);

pin!(PA21, Pa21, GPIOA, Gpioa, _PA21, GpioPin, _GPIOA, 21);

pin!(PA22, Pa22, GPIOA, Gpioa, _PA22, GpioPin, _GPIOA, 22);

pin!(PA23, Pa23, GPIOA, Gpioa, _PA23, GpioPin, _GPIOA, 23);

pin!(PA24, Pa24, GPIOA, Gpioa, _PA24, GpioPin, _GPIOA, 24);

pin!(PA25, Pa25, GPIOA, Gpioa, _PA25, GpioPin, _GPIOA, 25);

pin!(PA26, Pa26, GPIOA, Gpioa, _PA26, GpioPin, _GPIOA, 26);

pin!(PA27, Pa27, GPIOA, Gpioa, _PA27, GpioPin, _GPIOA, 27);

pin!(PA28, Pa28, GPIOA, Gpioa, _PA28, GpioPin, _GPIOA, 28);

pin!(PA29, Pa29, GPIOA, Gpioa, _PA29, GpioPin, _GPIOA, 29);

pin!(PB0, Pb0, GPIOB, Gpiob, _PB0, GpioPin, _GPIOB, 0);

pin!(PB1, Pb1, GPIOB, Gpiob, _PB1, GpioPin, _GPIOB, 1);

pin!(PB2, Pb2, GPIOB, Gpiob, _PB2, GpioPin, _GPIOB, 2);

pin!(PB3, Pb3, GPIOB, Gpiob, _PB3, GpioPin, _GPIOB, 3);

pin!(PB4, Pb4, GPIOB, Gpiob, _PB4, GpioPin, _GPIOB, 4);

pin!(PB5, Pb5, GPIOB, Gpiob, _PB5, GpioPin, _GPIOB, 5);

pin!(PB6, Pb6, GPIOB, Gpiob, _PB6, GpioPin, _GPIOB, 6);

pin!(PB7, Pb7, GPIOB, Gpiob, _PB7, GpioPin, _GPIOB, 7);

pin!(PB8, Pb8, GPIOB, Gpiob, _PB8, GpioPin, _GPIOB, 8);

pin!(PB9, Pb9, GPIOB, Gpiob, _PB9, GpioPin, _GPIOB, 9);

pin!(PB10, Pb10, GPIOB, Gpiob, _PB10, GpioPin, _GPIOB, 10);

pin!(PB11, Pb11, GPIOB, Gpiob, _PB11, GpioPin, _GPIOB, 11);

pin!(PB12, Pb12, GPIOB, Gpiob, _PB12, GpioPin, _GPIOB, 12);

pin!(PB13, Pb13, GPIOB, Gpiob, _PB13, GpioPin, _GPIOB, 13);

pin!(PB14, Pb14, GPIOB, Gpiob, _PB14, GpioPin, _GPIOB, 14);

pin!(PB15, Pb15, GPIOB, Gpiob, _PB15, GpioPin, _GPIOB, 15);

pin!(PB16, Pb16, GPIOB, Gpiob, _PB16, GpioPin, _GPIOB, 16);

pin!(PB17, Pb17, GPIOB, Gpiob, _PB17, GpioPin, _GPIOB, 17);

pin!(PB18, Pb18, GPIOB, Gpiob, _PB18, GpioPin, _GPIOB, 18);

pin!(PB19, Pb19, GPIOB, Gpiob, _PB19, GpioPin, _GPIOB, 19);

pin!(PB20, Pb20, GPIOB, Gpiob, _PB20, GpioPin, _GPIOB, 20);

pin!(PB21, Pb21, GPIOB, Gpiob, _PB21, GpioPin, _GPIOB, 21);

pin!(PB22, Pb22, GPIOB, Gpiob, _PB22, GpioPin, _GPIOB, 22);

pin!(PB23, Pb23, GPIOB, Gpiob, _PB23, GpioPin, _GPIOB, 23);

pin!(PC0, Pc0, GPIOC, Gpioc, _PC0, GpioPin, _GPIOC, 0);

pin!(PC1, Pc1, GPIOC, Gpioc, _PC1, GpioPin, _GPIOC, 1);

pin!(PC2, Pc2, GPIOC, Gpioc, _PC2, GpioPin, _GPIOC, 2);

pin!(PC3, Pc3, GPIOC, Gpioc, _PC3, GpioPin, _GPIOC, 3);

pin!(PC4, Pc4, GPIOC, Gpioc, _PC4, GpioPin, _GPIOC, 4);

pin!(PC5, Pc5, GPIOC, Gpioc, _PC5, GpioPin, _GPIOC, 5);

pin!(PC6, Pc6, GPIOC, Gpioc, _PC6, GpioPin, _GPIOC, 6);

pin!(PC7, Pc7, GPIOC, Gpioc, _PC7, GpioPin, _GPIOC, 7);

pin!(PC8, Pc8, GPIOC, Gpioc, _PC8, GpioPin, _GPIOC, 8);

pin!(PC9, Pc9, GPIOC, Gpioc, _PC9, GpioPin, _GPIOC, 9);

pin!(PC10, Pc10, GPIOC, Gpioc, _PC10, GpioPin, _GPIOC, 10);

pin!(PC11, Pc11, GPIOC, Gpioc, _PC11, GpioPin, _GPIOC, 11);

pin!(PC12, Pc12, GPIOC, Gpioc, _PC12, GpioPin, _GPIOC, 12);

pin!(PC13, Pc13, GPIOC, Gpioc, _PC13, GpioPin, _GPIOC, 13);

pin!(PC14, Pc14, GPIOC, Gpioc, _PC14, GpioPin, _GPIOC, 14);

pin!(PC15, Pc15, GPIOC, Gpioc, _PC15, GpioPin, _GPIOC, 15);

pin!(PC16, Pc16, GPIOC, Gpioc, _PC16, GpioPin, _GPIOC, 16);

pin!(PC17, Pc17, GPIOC, Gpioc, _PC17, GpioPin, _GPIOC, 17);

pin!(PC18, Pc18, GPIOC, Gpioc, _PC18, GpioPin, _GPIOC, 18);

pin!(PC19, Pc19, GPIOC, Gpioc, _PC19, GpioPin, _GPIOC, 19);

pin!(PD0, Pd0, GPIOD, Gpiod, _PD0, GpioPin, _GPIOD, 0);

pin!(PD1, Pd1, GPIOD, Gpiod, _PD1, GpioPin, _GPIOD, 1);

pin!(PD2, Pd2, GPIOD, Gpiod, _PD2, GpioPin, _GPIOD, 2);

pin!(PD3, Pd3, GPIOD, Gpiod, _PD3, GpioPin, _GPIOD, 3);

pin!(PD4, Pd4, GPIOD, Gpiod, _PD4, GpioPin, _GPIOD, 4);

pin!(PD5, Pd5, GPIOD, Gpiod, _PD5, GpioPin, _GPIOD, 5);

pin!(PD6, Pd6, GPIOD, Gpiod, _PD6, GpioPin, _GPIOD, 6);

pin!(PD7, Pd7, GPIOD, Gpiod, _PD7, GpioPin, _GPIOD, 7);

pin!(PD8, Pd8, GPIOD, Gpiod, _PD8, GpioPin, _GPIOD, 8);

pin!(PD9, Pd9, GPIOD, Gpiod, _PD9, GpioPin, _GPIOD, 9);

pin!(PD10, Pd10, GPIOD, Gpiod, _PD10, GpioPin, _GPIOD, 10);

pin!(PD11, Pd11, GPIOD, Gpiod, _PD11, GpioPin, _GPIOD, 11);

pin!(PD12, Pd12, GPIOD, Gpiod, _PD12, GpioPin, _GPIOD, 12);

pin!(PD13, Pd13, GPIOD, Gpiod, _PD13, GpioPin, _GPIOD, 13);

pin!(PD14, Pd14, GPIOD, Gpiod, _PD14, GpioPin, _GPIOD, 14);

pin!(PD15, Pd15, GPIOD, Gpiod, _PD15, GpioPin, _GPIOD, 15);

pin!(PE0, Pe0, GPIOE, Gpioe, _PE0, GpioPin, _GPIOE, 0);

pin!(PE1, Pe1, GPIOE, Gpioe, _PE1, GpioPin, _GPIOE, 1);

pin!(PE2, Pe2, GPIOE, Gpioe, _PE2, GpioPin, _GPIOE, 2);

pin!(PE3, Pe3, GPIOE, Gpioe, _PE3, GpioPin, _GPIOE, 3);

pin!(PE4, Pe4, GPIOE, Gpioe, _PE4, GpioPin, _GPIOE, 4);

pin!(PE5, Pe5, GPIOE, Gpioe, _PE5, GpioPin, _GPIOE, 5);

pin!(PE6, Pe6, GPIOE, Gpioe, _PE6, GpioPin, _GPIOE, 6);

pin!(PE7, Pe7, GPIOE, Gpioe, _PE7, GpioPin, _GPIOE, 7);

pin!(PE8, Pe8, GPIOE, Gpioe, _PE8, GpioPin, _GPIOE, 8);

pin!(PE9, Pe9, GPIOE, Gpioe, _PE9, GpioPin, _GPIOE, 9);

pin!(PE10, Pe10, GPIOE, Gpioe, _PE10, GpioPin, _GPIOE, 10);

pin!(PE11, Pe11, GPIOE, Gpioe, _PE11, GpioPin, _GPIOE, 11);

pin!(PE12, Pe12, GPIOE, Gpioe, _PE12, GpioPin, _GPIOE, 12);

pin!(PE13, Pe13, GPIOE, Gpioe, _PE13, GpioPin, _GPIOE, 13);

pin!(PE14, Pe14, GPIOE, Gpioe, _PE14, GpioPin, _GPIOE, 14);

pin!(PE15, Pe15, GPIOE, Gpioe, _PE15, GpioPin, _GPIOE, 15);

pin!(PE16, Pe16, GPIOE, Gpioe, _PE16, GpioPin, _GPIOE, 16);

pin!(PE17, Pe17, GPIOE, Gpioe, _PE17, GpioPin, _GPIOE, 17);

pin!(PE18, Pe18, GPIOE, Gpioe, _PE18, GpioPin, _GPIOE, 18);

pin!(PE19, Pe19, GPIOE, Gpioe, _PE19, GpioPin, _GPIOE, 19);

pin!(PE20, Pe20, GPIOE, Gpioe, _PE20, GpioPin, _GPIOE, 20);

pin!(PE21, Pe21, GPIOE, Gpioe, _PE21, GpioPin, _GPIOE, 21);

pin!(PE22, Pe22, GPIOE, Gpioe, _PE22, GpioPin, _GPIOE, 22);

pin!(PE23, Pe23, GPIOE, Gpioe, _PE23, GpioPin, _GPIOE, 23);

pin!(PE24, Pe24, GPIOE, Gpioe, _PE24, GpioPin, _GPIOE, 24);

pin!(PE25, Pe25, GPIOE, Gpioe, _PE25, GpioPin, _GPIOE, 25);

pin!(PE26, Pe26, GPIOE, Gpioe, _PE26, GpioPin, _GPIOE, 26);

pin!(PE27, Pe27, GPIOE, Gpioe, _PE27, GpioPin, _GPIOE, 27);

pin!(PE28, Pe28, GPIOE, Gpioe, _PE28, GpioPin, _GPIOE, 28);

