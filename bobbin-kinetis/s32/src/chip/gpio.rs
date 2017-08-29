#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::gpio::*;

pub trait LinkPort<T> {
   fn port(&self) -> T;
}

pub trait LinkPortPin<T> {
   fn port_pin(&self) -> T;
}

periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x400ff000);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x400ff040);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x400ff080);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x400ff0c0);
periph!( GPIOE, Gpioe, _GPIOE, GpioPeriph, 0x400ff100);

impl LinkPort<super::port::Porta > for Gpioa {
   #[inline] fn port(&self) -> super::port::Porta  { super::port::PORTA }
}

impl LinkPortPin<super::port::Pta0 > for Pa0 {
   #[inline] fn port_pin(&self) -> super::port::Pta0  { super::port::PTA0 }
}

impl LinkPortPin<super::port::Pta1 > for Pa1 {
   #[inline] fn port_pin(&self) -> super::port::Pta1  { super::port::PTA1 }
}

impl LinkPortPin<super::port::Pta2 > for Pa2 {
   #[inline] fn port_pin(&self) -> super::port::Pta2  { super::port::PTA2 }
}

impl LinkPortPin<super::port::Pta3 > for Pa3 {
   #[inline] fn port_pin(&self) -> super::port::Pta3  { super::port::PTA3 }
}

impl LinkPortPin<super::port::Pta4 > for Pa4 {
   #[inline] fn port_pin(&self) -> super::port::Pta4  { super::port::PTA4 }
}

impl LinkPortPin<super::port::Pta5 > for Pa5 {
   #[inline] fn port_pin(&self) -> super::port::Pta5  { super::port::PTA5 }
}

impl LinkPortPin<super::port::Pta6 > for Pa6 {
   #[inline] fn port_pin(&self) -> super::port::Pta6  { super::port::PTA6 }
}

impl LinkPortPin<super::port::Pta7 > for Pa7 {
   #[inline] fn port_pin(&self) -> super::port::Pta7  { super::port::PTA7 }
}

impl LinkPortPin<super::port::Pta8 > for Pa8 {
   #[inline] fn port_pin(&self) -> super::port::Pta8  { super::port::PTA8 }
}

impl LinkPortPin<super::port::Pta9 > for Pa9 {
   #[inline] fn port_pin(&self) -> super::port::Pta9  { super::port::PTA9 }
}

impl LinkPortPin<super::port::Pta10 > for Pa10 {
   #[inline] fn port_pin(&self) -> super::port::Pta10  { super::port::PTA10 }
}

impl LinkPortPin<super::port::Pta11 > for Pa11 {
   #[inline] fn port_pin(&self) -> super::port::Pta11  { super::port::PTA11 }
}

impl LinkPortPin<super::port::Pta12 > for Pa12 {
   #[inline] fn port_pin(&self) -> super::port::Pta12  { super::port::PTA12 }
}

impl LinkPortPin<super::port::Pta13 > for Pa13 {
   #[inline] fn port_pin(&self) -> super::port::Pta13  { super::port::PTA13 }
}

impl LinkPortPin<super::port::Pta14 > for Pa14 {
   #[inline] fn port_pin(&self) -> super::port::Pta14  { super::port::PTA14 }
}

impl LinkPortPin<super::port::Pta15 > for Pa15 {
   #[inline] fn port_pin(&self) -> super::port::Pta15  { super::port::PTA15 }
}

impl LinkPortPin<super::port::Pta16 > for Pa16 {
   #[inline] fn port_pin(&self) -> super::port::Pta16  { super::port::PTA16 }
}

impl LinkPortPin<super::port::Pta17 > for Pa17 {
   #[inline] fn port_pin(&self) -> super::port::Pta17  { super::port::PTA17 }
}


impl LinkPort<super::port::Portb > for Gpiob {
   #[inline] fn port(&self) -> super::port::Portb  { super::port::PORTB }
}

impl LinkPortPin<super::port::Ptb0 > for Pb0 {
   #[inline] fn port_pin(&self) -> super::port::Ptb0  { super::port::PTB0 }
}

impl LinkPortPin<super::port::Ptb1 > for Pb1 {
   #[inline] fn port_pin(&self) -> super::port::Ptb1  { super::port::PTB1 }
}

impl LinkPortPin<super::port::Ptb2 > for Pb2 {
   #[inline] fn port_pin(&self) -> super::port::Ptb2  { super::port::PTB2 }
}

impl LinkPortPin<super::port::Ptb3 > for Pb3 {
   #[inline] fn port_pin(&self) -> super::port::Ptb3  { super::port::PTB3 }
}

impl LinkPortPin<super::port::Ptb4 > for Pb4 {
   #[inline] fn port_pin(&self) -> super::port::Ptb4  { super::port::PTB4 }
}

impl LinkPortPin<super::port::Ptb5 > for Pb5 {
   #[inline] fn port_pin(&self) -> super::port::Ptb5  { super::port::PTB5 }
}

impl LinkPortPin<super::port::Ptb6 > for Pb6 {
   #[inline] fn port_pin(&self) -> super::port::Ptb6  { super::port::PTB6 }
}

impl LinkPortPin<super::port::Ptb7 > for Pb7 {
   #[inline] fn port_pin(&self) -> super::port::Ptb7  { super::port::PTB7 }
}

impl LinkPortPin<super::port::Ptb8 > for Pb8 {
   #[inline] fn port_pin(&self) -> super::port::Ptb8  { super::port::PTB8 }
}

impl LinkPortPin<super::port::Ptb9 > for Pb9 {
   #[inline] fn port_pin(&self) -> super::port::Ptb9  { super::port::PTB9 }
}

impl LinkPortPin<super::port::Ptb10 > for Pb10 {
   #[inline] fn port_pin(&self) -> super::port::Ptb10  { super::port::PTB10 }
}

impl LinkPortPin<super::port::Ptb11 > for Pb11 {
   #[inline] fn port_pin(&self) -> super::port::Ptb11  { super::port::PTB11 }
}

impl LinkPortPin<super::port::Ptb12 > for Pb12 {
   #[inline] fn port_pin(&self) -> super::port::Ptb12  { super::port::PTB12 }
}

impl LinkPortPin<super::port::Ptb13 > for Pb13 {
   #[inline] fn port_pin(&self) -> super::port::Ptb13  { super::port::PTB13 }
}

impl LinkPortPin<super::port::Ptb14 > for Pb14 {
   #[inline] fn port_pin(&self) -> super::port::Ptb14  { super::port::PTB14 }
}

impl LinkPortPin<super::port::Ptb15 > for Pb15 {
   #[inline] fn port_pin(&self) -> super::port::Ptb15  { super::port::PTB15 }
}

impl LinkPortPin<super::port::Ptb16 > for Pb16 {
   #[inline] fn port_pin(&self) -> super::port::Ptb16  { super::port::PTB16 }
}

impl LinkPortPin<super::port::Ptb17 > for Pb17 {
   #[inline] fn port_pin(&self) -> super::port::Ptb17  { super::port::PTB17 }
}


impl LinkPort<super::port::Portc > for Gpioc {
   #[inline] fn port(&self) -> super::port::Portc  { super::port::PORTC }
}

impl LinkPortPin<super::port::Ptc0 > for Pc0 {
   #[inline] fn port_pin(&self) -> super::port::Ptc0  { super::port::PTC0 }
}

impl LinkPortPin<super::port::Ptc1 > for Pc1 {
   #[inline] fn port_pin(&self) -> super::port::Ptc1  { super::port::PTC1 }
}

impl LinkPortPin<super::port::Ptc2 > for Pc2 {
   #[inline] fn port_pin(&self) -> super::port::Ptc2  { super::port::PTC2 }
}

impl LinkPortPin<super::port::Ptc3 > for Pc3 {
   #[inline] fn port_pin(&self) -> super::port::Ptc3  { super::port::PTC3 }
}

impl LinkPortPin<super::port::Ptc4 > for Pc4 {
   #[inline] fn port_pin(&self) -> super::port::Ptc4  { super::port::PTC4 }
}

impl LinkPortPin<super::port::Ptc5 > for Pc5 {
   #[inline] fn port_pin(&self) -> super::port::Ptc5  { super::port::PTC5 }
}

impl LinkPortPin<super::port::Ptc6 > for Pc6 {
   #[inline] fn port_pin(&self) -> super::port::Ptc6  { super::port::PTC6 }
}

impl LinkPortPin<super::port::Ptc7 > for Pc7 {
   #[inline] fn port_pin(&self) -> super::port::Ptc7  { super::port::PTC7 }
}

impl LinkPortPin<super::port::Ptc8 > for Pc8 {
   #[inline] fn port_pin(&self) -> super::port::Ptc8  { super::port::PTC8 }
}

impl LinkPortPin<super::port::Ptc9 > for Pc9 {
   #[inline] fn port_pin(&self) -> super::port::Ptc9  { super::port::PTC9 }
}

impl LinkPortPin<super::port::Ptc10 > for Pc10 {
   #[inline] fn port_pin(&self) -> super::port::Ptc10  { super::port::PTC10 }
}

impl LinkPortPin<super::port::Ptc11 > for Pc11 {
   #[inline] fn port_pin(&self) -> super::port::Ptc11  { super::port::PTC11 }
}

impl LinkPortPin<super::port::Ptc12 > for Pc12 {
   #[inline] fn port_pin(&self) -> super::port::Ptc12  { super::port::PTC12 }
}

impl LinkPortPin<super::port::Ptc13 > for Pc13 {
   #[inline] fn port_pin(&self) -> super::port::Ptc13  { super::port::PTC13 }
}

impl LinkPortPin<super::port::Ptc14 > for Pc14 {
   #[inline] fn port_pin(&self) -> super::port::Ptc14  { super::port::PTC14 }
}

impl LinkPortPin<super::port::Ptc15 > for Pc15 {
   #[inline] fn port_pin(&self) -> super::port::Ptc15  { super::port::PTC15 }
}

impl LinkPortPin<super::port::Ptc16 > for Pc16 {
   #[inline] fn port_pin(&self) -> super::port::Ptc16  { super::port::PTC16 }
}

impl LinkPortPin<super::port::Ptc17 > for Pc17 {
   #[inline] fn port_pin(&self) -> super::port::Ptc17  { super::port::PTC17 }
}


impl LinkPort<super::port::Portd > for Gpiod {
   #[inline] fn port(&self) -> super::port::Portd  { super::port::PORTD }
}

impl LinkPortPin<super::port::Ptd0 > for Pd0 {
   #[inline] fn port_pin(&self) -> super::port::Ptd0  { super::port::PTD0 }
}

impl LinkPortPin<super::port::Ptd1 > for Pd1 {
   #[inline] fn port_pin(&self) -> super::port::Ptd1  { super::port::PTD1 }
}

impl LinkPortPin<super::port::Ptd2 > for Pd2 {
   #[inline] fn port_pin(&self) -> super::port::Ptd2  { super::port::PTD2 }
}

impl LinkPortPin<super::port::Ptd3 > for Pd3 {
   #[inline] fn port_pin(&self) -> super::port::Ptd3  { super::port::PTD3 }
}

impl LinkPortPin<super::port::Ptd4 > for Pd4 {
   #[inline] fn port_pin(&self) -> super::port::Ptd4  { super::port::PTD4 }
}

impl LinkPortPin<super::port::Ptd5 > for Pd5 {
   #[inline] fn port_pin(&self) -> super::port::Ptd5  { super::port::PTD5 }
}

impl LinkPortPin<super::port::Ptd6 > for Pd6 {
   #[inline] fn port_pin(&self) -> super::port::Ptd6  { super::port::PTD6 }
}

impl LinkPortPin<super::port::Ptd7 > for Pd7 {
   #[inline] fn port_pin(&self) -> super::port::Ptd7  { super::port::PTD7 }
}

impl LinkPortPin<super::port::Ptd8 > for Pd8 {
   #[inline] fn port_pin(&self) -> super::port::Ptd8  { super::port::PTD8 }
}

impl LinkPortPin<super::port::Ptd9 > for Pd9 {
   #[inline] fn port_pin(&self) -> super::port::Ptd9  { super::port::PTD9 }
}

impl LinkPortPin<super::port::Ptd10 > for Pd10 {
   #[inline] fn port_pin(&self) -> super::port::Ptd10  { super::port::PTD10 }
}

impl LinkPortPin<super::port::Ptd11 > for Pd11 {
   #[inline] fn port_pin(&self) -> super::port::Ptd11  { super::port::PTD11 }
}

impl LinkPortPin<super::port::Ptd12 > for Pd12 {
   #[inline] fn port_pin(&self) -> super::port::Ptd12  { super::port::PTD12 }
}

impl LinkPortPin<super::port::Ptd13 > for Pd13 {
   #[inline] fn port_pin(&self) -> super::port::Ptd13  { super::port::PTD13 }
}

impl LinkPortPin<super::port::Ptd14 > for Pd14 {
   #[inline] fn port_pin(&self) -> super::port::Ptd14  { super::port::PTD14 }
}

impl LinkPortPin<super::port::Ptd15 > for Pd15 {
   #[inline] fn port_pin(&self) -> super::port::Ptd15  { super::port::PTD15 }
}

impl LinkPortPin<super::port::Ptd16 > for Pd16 {
   #[inline] fn port_pin(&self) -> super::port::Ptd16  { super::port::PTD16 }
}

impl LinkPortPin<super::port::Ptd17 > for Pd17 {
   #[inline] fn port_pin(&self) -> super::port::Ptd17  { super::port::PTD17 }
}


impl LinkPort<super::port::Porte > for Gpioe {
   #[inline] fn port(&self) -> super::port::Porte  { super::port::PORTE }
}

impl LinkPortPin<super::port::Pte0 > for Pe0 {
   #[inline] fn port_pin(&self) -> super::port::Pte0  { super::port::PTE0 }
}

impl LinkPortPin<super::port::Pte1 > for Pe1 {
   #[inline] fn port_pin(&self) -> super::port::Pte1  { super::port::PTE1 }
}

impl LinkPortPin<super::port::Pte2 > for Pe2 {
   #[inline] fn port_pin(&self) -> super::port::Pte2  { super::port::PTE2 }
}

impl LinkPortPin<super::port::Pte3 > for Pe3 {
   #[inline] fn port_pin(&self) -> super::port::Pte3  { super::port::PTE3 }
}

impl LinkPortPin<super::port::Pte4 > for Pe4 {
   #[inline] fn port_pin(&self) -> super::port::Pte4  { super::port::PTE4 }
}

impl LinkPortPin<super::port::Pte5 > for Pe5 {
   #[inline] fn port_pin(&self) -> super::port::Pte5  { super::port::PTE5 }
}

impl LinkPortPin<super::port::Pte6 > for Pe6 {
   #[inline] fn port_pin(&self) -> super::port::Pte6  { super::port::PTE6 }
}

impl LinkPortPin<super::port::Pte7 > for Pe7 {
   #[inline] fn port_pin(&self) -> super::port::Pte7  { super::port::PTE7 }
}

impl LinkPortPin<super::port::Pte8 > for Pe8 {
   #[inline] fn port_pin(&self) -> super::port::Pte8  { super::port::PTE8 }
}

impl LinkPortPin<super::port::Pte9 > for Pe9 {
   #[inline] fn port_pin(&self) -> super::port::Pte9  { super::port::PTE9 }
}

impl LinkPortPin<super::port::Pte10 > for Pe10 {
   #[inline] fn port_pin(&self) -> super::port::Pte10  { super::port::PTE10 }
}

impl LinkPortPin<super::port::Pte11 > for Pe11 {
   #[inline] fn port_pin(&self) -> super::port::Pte11  { super::port::PTE11 }
}

impl LinkPortPin<super::port::Pte12 > for Pe12 {
   #[inline] fn port_pin(&self) -> super::port::Pte12  { super::port::PTE12 }
}

impl LinkPortPin<super::port::Pte13 > for Pe13 {
   #[inline] fn port_pin(&self) -> super::port::Pte13  { super::port::PTE13 }
}

impl LinkPortPin<super::port::Pte14 > for Pe14 {
   #[inline] fn port_pin(&self) -> super::port::Pte14  { super::port::PTE14 }
}

impl LinkPortPin<super::port::Pte15 > for Pe15 {
   #[inline] fn port_pin(&self) -> super::port::Pte15  { super::port::PTE15 }
}

impl LinkPortPin<super::port::Pte16 > for Pe16 {
   #[inline] fn port_pin(&self) -> super::port::Pte16  { super::port::PTE16 }
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

pin!(PD16, Pd16, GPIOD, Gpiod, _PD16, GpioPin, _GPIOD, 16);

pin!(PD17, Pd17, GPIOD, Gpiod, _PD17, GpioPin, _GPIOD, 17);

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


