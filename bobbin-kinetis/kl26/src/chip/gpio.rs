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
   fn port(&self) -> super::port::Porta  { super::port::PORTA }
}

impl LinkPortPin<super::port::Pta0 > for Pa0 {
   fn port_pin(&self) -> super::port::Pta0  { super::port::PTA0 }
}

impl LinkPortPin<super::port::Pta1 > for Pa1 {
   fn port_pin(&self) -> super::port::Pta1  { super::port::PTA1 }
}

impl LinkPortPin<super::port::Pta2 > for Pa2 {
   fn port_pin(&self) -> super::port::Pta2  { super::port::PTA2 }
}

impl LinkPortPin<super::port::Pta3 > for Pa3 {
   fn port_pin(&self) -> super::port::Pta3  { super::port::PTA3 }
}

impl LinkPortPin<super::port::Pta4 > for Pa4 {
   fn port_pin(&self) -> super::port::Pta4  { super::port::PTA4 }
}

impl LinkPortPin<super::port::Pta5 > for Pa5 {
   fn port_pin(&self) -> super::port::Pta5  { super::port::PTA5 }
}

impl LinkPortPin<super::port::Pta12 > for Pa12 {
   fn port_pin(&self) -> super::port::Pta12  { super::port::PTA12 }
}

impl LinkPortPin<super::port::Pta13 > for Pa13 {
   fn port_pin(&self) -> super::port::Pta13  { super::port::PTA13 }
}

impl LinkPortPin<super::port::Pta18 > for Pa18 {
   fn port_pin(&self) -> super::port::Pta18  { super::port::PTA18 }
}

impl LinkPortPin<super::port::Pta19 > for Pa19 {
   fn port_pin(&self) -> super::port::Pta19  { super::port::PTA19 }
}

impl LinkPortPin<super::port::Pta20 > for Pa20 {
   fn port_pin(&self) -> super::port::Pta20  { super::port::PTA20 }
}


impl LinkPort<super::port::Portb > for Gpiob {
   fn port(&self) -> super::port::Portb  { super::port::PORTB }
}

impl LinkPortPin<super::port::Ptb0 > for Pb0 {
   fn port_pin(&self) -> super::port::Ptb0  { super::port::PTB0 }
}

impl LinkPortPin<super::port::Ptb1 > for Pb1 {
   fn port_pin(&self) -> super::port::Ptb1  { super::port::PTB1 }
}

impl LinkPortPin<super::port::Ptb2 > for Pb2 {
   fn port_pin(&self) -> super::port::Ptb2  { super::port::PTB2 }
}

impl LinkPortPin<super::port::Ptb3 > for Pb3 {
   fn port_pin(&self) -> super::port::Ptb3  { super::port::PTB3 }
}

impl LinkPortPin<super::port::Ptb16 > for Pb16 {
   fn port_pin(&self) -> super::port::Ptb16  { super::port::PTB16 }
}

impl LinkPortPin<super::port::Ptb17 > for Pb17 {
   fn port_pin(&self) -> super::port::Ptb17  { super::port::PTB17 }
}

impl LinkPortPin<super::port::Ptb18 > for Pb18 {
   fn port_pin(&self) -> super::port::Ptb18  { super::port::PTB18 }
}

impl LinkPortPin<super::port::Ptb19 > for Pb19 {
   fn port_pin(&self) -> super::port::Ptb19  { super::port::PTB19 }
}


impl LinkPort<super::port::Portc > for Gpioc {
   fn port(&self) -> super::port::Portc  { super::port::PORTC }
}

impl LinkPortPin<super::port::Ptc0 > for Pc0 {
   fn port_pin(&self) -> super::port::Ptc0  { super::port::PTC0 }
}

impl LinkPortPin<super::port::Ptc1 > for Pc1 {
   fn port_pin(&self) -> super::port::Ptc1  { super::port::PTC1 }
}

impl LinkPortPin<super::port::Ptc2 > for Pc2 {
   fn port_pin(&self) -> super::port::Ptc2  { super::port::PTC2 }
}

impl LinkPortPin<super::port::Ptc3 > for Pc3 {
   fn port_pin(&self) -> super::port::Ptc3  { super::port::PTC3 }
}

impl LinkPortPin<super::port::Ptc4 > for Pc4 {
   fn port_pin(&self) -> super::port::Ptc4  { super::port::PTC4 }
}

impl LinkPortPin<super::port::Ptc5 > for Pc5 {
   fn port_pin(&self) -> super::port::Ptc5  { super::port::PTC5 }
}

impl LinkPortPin<super::port::Ptc6 > for Pc6 {
   fn port_pin(&self) -> super::port::Ptc6  { super::port::PTC6 }
}

impl LinkPortPin<super::port::Ptc7 > for Pc7 {
   fn port_pin(&self) -> super::port::Ptc7  { super::port::PTC7 }
}

impl LinkPortPin<super::port::Ptc8 > for Pc8 {
   fn port_pin(&self) -> super::port::Ptc8  { super::port::PTC8 }
}

impl LinkPortPin<super::port::Ptc9 > for Pc9 {
   fn port_pin(&self) -> super::port::Ptc9  { super::port::PTC9 }
}

impl LinkPortPin<super::port::Ptc10 > for Pc10 {
   fn port_pin(&self) -> super::port::Ptc10  { super::port::PTC10 }
}

impl LinkPortPin<super::port::Ptc11 > for Pc11 {
   fn port_pin(&self) -> super::port::Ptc11  { super::port::PTC11 }
}


impl LinkPort<super::port::Portd > for Gpiod {
   fn port(&self) -> super::port::Portd  { super::port::PORTD }
}

impl LinkPortPin<super::port::Ptd0 > for Pd0 {
   fn port_pin(&self) -> super::port::Ptd0  { super::port::PTD0 }
}

impl LinkPortPin<super::port::Ptd1 > for Pd1 {
   fn port_pin(&self) -> super::port::Ptd1  { super::port::PTD1 }
}

impl LinkPortPin<super::port::Ptd2 > for Pd2 {
   fn port_pin(&self) -> super::port::Ptd2  { super::port::PTD2 }
}

impl LinkPortPin<super::port::Ptd3 > for Pd3 {
   fn port_pin(&self) -> super::port::Ptd3  { super::port::PTD3 }
}

impl LinkPortPin<super::port::Ptd4 > for Pd4 {
   fn port_pin(&self) -> super::port::Ptd4  { super::port::PTD4 }
}

impl LinkPortPin<super::port::Ptd5 > for Pd5 {
   fn port_pin(&self) -> super::port::Ptd5  { super::port::PTD5 }
}

impl LinkPortPin<super::port::Ptd6 > for Pd6 {
   fn port_pin(&self) -> super::port::Ptd6  { super::port::PTD6 }
}

impl LinkPortPin<super::port::Ptd7 > for Pd7 {
   fn port_pin(&self) -> super::port::Ptd7  { super::port::PTD7 }
}


impl LinkPort<super::port::Porte > for Gpioe {
   fn port(&self) -> super::port::Porte  { super::port::PORTE }
}

impl LinkPortPin<super::port::Pte0 > for Pe0 {
   fn port_pin(&self) -> super::port::Pte0  { super::port::PTE0 }
}

impl LinkPortPin<super::port::Pte1 > for Pe1 {
   fn port_pin(&self) -> super::port::Pte1  { super::port::PTE1 }
}

impl LinkPortPin<super::port::Pte20 > for Pe20 {
   fn port_pin(&self) -> super::port::Pte20  { super::port::PTE20 }
}

impl LinkPortPin<super::port::Pte21 > for Pe21 {
   fn port_pin(&self) -> super::port::Pte21  { super::port::PTE21 }
}

impl LinkPortPin<super::port::Pte22 > for Pe22 {
   fn port_pin(&self) -> super::port::Pte22  { super::port::PTE22 }
}

impl LinkPortPin<super::port::Pte23 > for Pe23 {
   fn port_pin(&self) -> super::port::Pte23  { super::port::PTE23 }
}

impl LinkPortPin<super::port::Pte24 > for Pe24 {
   fn port_pin(&self) -> super::port::Pte24  { super::port::PTE24 }
}

impl LinkPortPin<super::port::Pte25 > for Pe25 {
   fn port_pin(&self) -> super::port::Pte25  { super::port::PTE25 }
}

impl LinkPortPin<super::port::Pte29 > for Pe29 {
   fn port_pin(&self) -> super::port::Pte29  { super::port::PTE29 }
}

impl LinkPortPin<super::port::Pte30 > for Pe30 {
   fn port_pin(&self) -> super::port::Pte30  { super::port::PTE30 }
}

impl LinkPortPin<super::port::Pte31 > for Pe31 {
   fn port_pin(&self) -> super::port::Pte31  { super::port::PTE31 }
}



pin!(PA0, Pa0, GPIOA, Gpioa, _PA0, GpioPin, _GPIOA, 0);

pin!(PA1, Pa1, GPIOA, Gpioa, _PA1, GpioPin, _GPIOA, 1);

pin!(PA2, Pa2, GPIOA, Gpioa, _PA2, GpioPin, _GPIOA, 2);

pin!(PA3, Pa3, GPIOA, Gpioa, _PA3, GpioPin, _GPIOA, 3);

pin!(PA4, Pa4, GPIOA, Gpioa, _PA4, GpioPin, _GPIOA, 4);

pin!(PA5, Pa5, GPIOA, Gpioa, _PA5, GpioPin, _GPIOA, 5);

pin!(PA12, Pa12, GPIOA, Gpioa, _PA12, GpioPin, _GPIOA, 12);

pin!(PA13, Pa13, GPIOA, Gpioa, _PA13, GpioPin, _GPIOA, 13);

pin!(PA18, Pa18, GPIOA, Gpioa, _PA18, GpioPin, _GPIOA, 18);

pin!(PA19, Pa19, GPIOA, Gpioa, _PA19, GpioPin, _GPIOA, 19);

pin!(PA20, Pa20, GPIOA, Gpioa, _PA20, GpioPin, _GPIOA, 20);

pin!(PB0, Pb0, GPIOB, Gpiob, _PB0, GpioPin, _GPIOB, 0);

pin!(PB1, Pb1, GPIOB, Gpiob, _PB1, GpioPin, _GPIOB, 1);

pin!(PB2, Pb2, GPIOB, Gpiob, _PB2, GpioPin, _GPIOB, 2);

pin!(PB3, Pb3, GPIOB, Gpiob, _PB3, GpioPin, _GPIOB, 3);

pin!(PB16, Pb16, GPIOB, Gpiob, _PB16, GpioPin, _GPIOB, 16);

pin!(PB17, Pb17, GPIOB, Gpiob, _PB17, GpioPin, _GPIOB, 17);

pin!(PB18, Pb18, GPIOB, Gpiob, _PB18, GpioPin, _GPIOB, 18);

pin!(PB19, Pb19, GPIOB, Gpiob, _PB19, GpioPin, _GPIOB, 19);

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

pin!(PD0, Pd0, GPIOD, Gpiod, _PD0, GpioPin, _GPIOD, 0);

pin!(PD1, Pd1, GPIOD, Gpiod, _PD1, GpioPin, _GPIOD, 1);

pin!(PD2, Pd2, GPIOD, Gpiod, _PD2, GpioPin, _GPIOD, 2);

pin!(PD3, Pd3, GPIOD, Gpiod, _PD3, GpioPin, _GPIOD, 3);

pin!(PD4, Pd4, GPIOD, Gpiod, _PD4, GpioPin, _GPIOD, 4);

pin!(PD5, Pd5, GPIOD, Gpiod, _PD5, GpioPin, _GPIOD, 5);

pin!(PD6, Pd6, GPIOD, Gpiod, _PD6, GpioPin, _GPIOD, 6);

pin!(PD7, Pd7, GPIOD, Gpiod, _PD7, GpioPin, _GPIOD, 7);

pin!(PE0, Pe0, GPIOE, Gpioe, _PE0, GpioPin, _GPIOE, 0);

pin!(PE1, Pe1, GPIOE, Gpioe, _PE1, GpioPin, _GPIOE, 1);

pin!(PE20, Pe20, GPIOE, Gpioe, _PE20, GpioPin, _GPIOE, 20);

pin!(PE21, Pe21, GPIOE, Gpioe, _PE21, GpioPin, _GPIOE, 21);

pin!(PE22, Pe22, GPIOE, Gpioe, _PE22, GpioPin, _GPIOE, 22);

pin!(PE23, Pe23, GPIOE, Gpioe, _PE23, GpioPin, _GPIOE, 23);

pin!(PE24, Pe24, GPIOE, Gpioe, _PE24, GpioPin, _GPIOE, 24);

pin!(PE25, Pe25, GPIOE, Gpioe, _PE25, GpioPin, _GPIOE, 25);

pin!(PE29, Pe29, GPIOE, Gpioe, _PE29, GpioPin, _GPIOE, 29);

pin!(PE30, Pe30, GPIOE, Gpioe, _PE30, GpioPin, _GPIOE, 30);

pin!(PE31, Pe31, GPIOE, Gpioe, _PE31, GpioPin, _GPIOE, 31);

