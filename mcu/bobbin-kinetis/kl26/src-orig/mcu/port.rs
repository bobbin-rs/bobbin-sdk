#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::port::*;

periph!( PORTA, Porta, PORTA_PERIPH, PortPeriph, 0x40049000, 0x10);
periph!( PORTB, Portb, PORTB_PERIPH, PortPeriph, 0x4004a000, 0x11);
periph!( PORTC, Portc, PORTC_PERIPH, PortPeriph, 0x4004b000, 0x12);
periph!( PORTD, Portd, PORTD_PERIPH, PortPeriph, 0x4004c000, 0x13);
periph!( PORTE, Porte, PORTE_PERIPH, PortPeriph, 0x4004d000, 0x14);

pub trait LinkGpio<T> {
    fn gpio(&self) -> T;
}

pub trait LinkGpioPin<T> {
    fn gpio_pin(&self) -> T;
}

impl LinkGpio<super::gpio::Gpioa > for Porta {
    #[inline] fn gpio(&self) -> super::gpio::Gpioa  { super::gpio::GPIOA }
}

impl LinkGpioPin<super::gpio::Pa0 > for Pta0 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa0  { super::gpio::PA0 }
}

impl LinkGpioPin<super::gpio::Pa1 > for Pta1 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa1  { super::gpio::PA1 }
}

impl LinkGpioPin<super::gpio::Pa2 > for Pta2 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa2  { super::gpio::PA2 }
}

impl LinkGpioPin<super::gpio::Pa3 > for Pta3 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa3  { super::gpio::PA3 }
}

impl LinkGpioPin<super::gpio::Pa4 > for Pta4 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa4  { super::gpio::PA4 }
}

impl LinkGpioPin<super::gpio::Pa5 > for Pta5 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa5  { super::gpio::PA5 }
}

impl LinkGpioPin<super::gpio::Pa12 > for Pta12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa12  { super::gpio::PA12 }
}

impl LinkGpioPin<super::gpio::Pa13 > for Pta13 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa13  { super::gpio::PA13 }
}

impl LinkGpioPin<super::gpio::Pa18 > for Pta18 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa18  { super::gpio::PA18 }
}

impl LinkGpioPin<super::gpio::Pa19 > for Pta19 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa19  { super::gpio::PA19 }
}

impl LinkGpioPin<super::gpio::Pa20 > for Pta20 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa20  { super::gpio::PA20 }
}

impl LinkGpio<super::gpio::Gpiob > for Portb {
    #[inline] fn gpio(&self) -> super::gpio::Gpiob  { super::gpio::GPIOB }
}

impl LinkGpioPin<super::gpio::Pb0 > for Ptb0 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb0  { super::gpio::PB0 }
}

impl LinkGpioPin<super::gpio::Pb1 > for Ptb1 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb1  { super::gpio::PB1 }
}

impl LinkGpioPin<super::gpio::Pb2 > for Ptb2 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb2  { super::gpio::PB2 }
}

impl LinkGpioPin<super::gpio::Pb3 > for Ptb3 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb3  { super::gpio::PB3 }
}

impl LinkGpioPin<super::gpio::Pb16 > for Ptb16 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb16  { super::gpio::PB16 }
}

impl LinkGpioPin<super::gpio::Pb17 > for Ptb17 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb17  { super::gpio::PB17 }
}

impl LinkGpioPin<super::gpio::Pb18 > for Ptb18 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb18  { super::gpio::PB18 }
}

impl LinkGpioPin<super::gpio::Pb19 > for Ptb19 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb19  { super::gpio::PB19 }
}

impl LinkGpio<super::gpio::Gpioc > for Portc {
    #[inline] fn gpio(&self) -> super::gpio::Gpioc  { super::gpio::GPIOC }
}

impl LinkGpioPin<super::gpio::Pc0 > for Ptc0 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc0  { super::gpio::PC0 }
}

impl LinkGpioPin<super::gpio::Pc1 > for Ptc1 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc1  { super::gpio::PC1 }
}

impl LinkGpioPin<super::gpio::Pc2 > for Ptc2 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc2  { super::gpio::PC2 }
}

impl LinkGpioPin<super::gpio::Pc3 > for Ptc3 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc3  { super::gpio::PC3 }
}

impl LinkGpioPin<super::gpio::Pc4 > for Ptc4 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc4  { super::gpio::PC4 }
}

impl LinkGpioPin<super::gpio::Pc5 > for Ptc5 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc5  { super::gpio::PC5 }
}

impl LinkGpioPin<super::gpio::Pc6 > for Ptc6 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc6  { super::gpio::PC6 }
}

impl LinkGpioPin<super::gpio::Pc7 > for Ptc7 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc7  { super::gpio::PC7 }
}

impl LinkGpioPin<super::gpio::Pc8 > for Ptc8 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc8  { super::gpio::PC8 }
}

impl LinkGpioPin<super::gpio::Pc9 > for Ptc9 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc9  { super::gpio::PC9 }
}

impl LinkGpioPin<super::gpio::Pc10 > for Ptc10 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc10  { super::gpio::PC10 }
}

impl LinkGpioPin<super::gpio::Pc11 > for Ptc11 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc11  { super::gpio::PC11 }
}

impl LinkGpio<super::gpio::Gpiod > for Portd {
    #[inline] fn gpio(&self) -> super::gpio::Gpiod  { super::gpio::GPIOD }
}

impl LinkGpioPin<super::gpio::Pd0 > for Ptd0 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd0  { super::gpio::PD0 }
}

impl LinkGpioPin<super::gpio::Pd1 > for Ptd1 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd1  { super::gpio::PD1 }
}

impl LinkGpioPin<super::gpio::Pd2 > for Ptd2 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd2  { super::gpio::PD2 }
}

impl LinkGpioPin<super::gpio::Pd3 > for Ptd3 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd3  { super::gpio::PD3 }
}

impl LinkGpioPin<super::gpio::Pd4 > for Ptd4 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd4  { super::gpio::PD4 }
}

impl LinkGpioPin<super::gpio::Pd5 > for Ptd5 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd5  { super::gpio::PD5 }
}

impl LinkGpioPin<super::gpio::Pd6 > for Ptd6 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd6  { super::gpio::PD6 }
}

impl LinkGpioPin<super::gpio::Pd7 > for Ptd7 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd7  { super::gpio::PD7 }
}

impl LinkGpio<super::gpio::Gpioe > for Porte {
    #[inline] fn gpio(&self) -> super::gpio::Gpioe  { super::gpio::GPIOE }
}

impl LinkGpioPin<super::gpio::Pe0 > for Pte0 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe0  { super::gpio::PE0 }
}

impl LinkGpioPin<super::gpio::Pe1 > for Pte1 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe1  { super::gpio::PE1 }
}

impl LinkGpioPin<super::gpio::Pe20 > for Pte20 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe20  { super::gpio::PE20 }
}

impl LinkGpioPin<super::gpio::Pe21 > for Pte21 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe21  { super::gpio::PE21 }
}

impl LinkGpioPin<super::gpio::Pe22 > for Pte22 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe22  { super::gpio::PE22 }
}

impl LinkGpioPin<super::gpio::Pe23 > for Pte23 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe23  { super::gpio::PE23 }
}

impl LinkGpioPin<super::gpio::Pe24 > for Pte24 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe24  { super::gpio::PE24 }
}

impl LinkGpioPin<super::gpio::Pe25 > for Pte25 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe25  { super::gpio::PE25 }
}

impl LinkGpioPin<super::gpio::Pe29 > for Pte29 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe29  { super::gpio::PE29 }
}

impl LinkGpioPin<super::gpio::Pe30 > for Pte30 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe30  { super::gpio::PE30 }
}

impl LinkGpioPin<super::gpio::Pe31 > for Pte31 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe31  { super::gpio::PE31 }
}

