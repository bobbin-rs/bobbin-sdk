#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x400ff000, 0x15);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x400ff040, 0x16);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x400ff080, 0x17);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x400ff0c0, 0x18);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, 0x400ff100, 0x19);

pub trait LinkPort<T> {
    fn port(&self) -> T;
}

pub trait LinkPortPin<T> {
    fn port_pin(&self) -> T;
}

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

impl LinkPortPin<super::port::Pta12 > for Pa12 {
    #[inline] fn port_pin(&self) -> super::port::Pta12  { super::port::PTA12 }
}

impl LinkPortPin<super::port::Pta13 > for Pa13 {
    #[inline] fn port_pin(&self) -> super::port::Pta13  { super::port::PTA13 }
}

impl LinkPortPin<super::port::Pta18 > for Pa18 {
    #[inline] fn port_pin(&self) -> super::port::Pta18  { super::port::PTA18 }
}

impl LinkPortPin<super::port::Pta19 > for Pa19 {
    #[inline] fn port_pin(&self) -> super::port::Pta19  { super::port::PTA19 }
}

impl LinkPortPin<super::port::Pta20 > for Pa20 {
    #[inline] fn port_pin(&self) -> super::port::Pta20  { super::port::PTA20 }
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

impl LinkPortPin<super::port::Ptb16 > for Pb16 {
    #[inline] fn port_pin(&self) -> super::port::Ptb16  { super::port::PTB16 }
}

impl LinkPortPin<super::port::Ptb17 > for Pb17 {
    #[inline] fn port_pin(&self) -> super::port::Ptb17  { super::port::PTB17 }
}

impl LinkPortPin<super::port::Ptb18 > for Pb18 {
    #[inline] fn port_pin(&self) -> super::port::Ptb18  { super::port::PTB18 }
}

impl LinkPortPin<super::port::Ptb19 > for Pb19 {
    #[inline] fn port_pin(&self) -> super::port::Ptb19  { super::port::PTB19 }
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

impl LinkPort<super::port::Porte > for Gpioe {
    #[inline] fn port(&self) -> super::port::Porte  { super::port::PORTE }
}

impl LinkPortPin<super::port::Pte0 > for Pe0 {
    #[inline] fn port_pin(&self) -> super::port::Pte0  { super::port::PTE0 }
}

impl LinkPortPin<super::port::Pte1 > for Pe1 {
    #[inline] fn port_pin(&self) -> super::port::Pte1  { super::port::PTE1 }
}

impl LinkPortPin<super::port::Pte20 > for Pe20 {
    #[inline] fn port_pin(&self) -> super::port::Pte20  { super::port::PTE20 }
}

impl LinkPortPin<super::port::Pte21 > for Pe21 {
    #[inline] fn port_pin(&self) -> super::port::Pte21  { super::port::PTE21 }
}

impl LinkPortPin<super::port::Pte22 > for Pe22 {
    #[inline] fn port_pin(&self) -> super::port::Pte22  { super::port::PTE22 }
}

impl LinkPortPin<super::port::Pte23 > for Pe23 {
    #[inline] fn port_pin(&self) -> super::port::Pte23  { super::port::PTE23 }
}

impl LinkPortPin<super::port::Pte24 > for Pe24 {
    #[inline] fn port_pin(&self) -> super::port::Pte24  { super::port::PTE24 }
}

impl LinkPortPin<super::port::Pte25 > for Pe25 {
    #[inline] fn port_pin(&self) -> super::port::Pte25  { super::port::PTE25 }
}

impl LinkPortPin<super::port::Pte29 > for Pe29 {
    #[inline] fn port_pin(&self) -> super::port::Pte29  { super::port::PTE29 }
}

impl LinkPortPin<super::port::Pte30 > for Pe30 {
    #[inline] fn port_pin(&self) -> super::port::Pte30  { super::port::PTE30 }
}

impl LinkPortPin<super::port::Pte31 > for Pe31 {
    #[inline] fn port_pin(&self) -> super::port::Pte31  { super::port::PTE31 }
}

