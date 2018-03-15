#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::port::*;

periph!( PORTA, Porta, PORTA_PERIPH, PortPeriph, 0x40049000, 0x13);
periph!( PORTB, Portb, PORTB_PERIPH, PortPeriph, 0x4004a000, 0x14);
periph!( PORTC, Portc, PORTC_PERIPH, PortPeriph, 0x4004b000, 0x15);
periph!( PORTD, Portd, PORTD_PERIPH, PortPeriph, 0x4004c000, 0x16);
periph!( PORTE, Porte, PORTE_PERIPH, PortPeriph, 0x4004d000, 0x17);

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

impl LinkGpioPin<super::gpio::Pa6 > for Pta6 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa6  { super::gpio::PA6 }
}

impl LinkGpioPin<super::gpio::Pa7 > for Pta7 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa7  { super::gpio::PA7 }
}

impl LinkGpioPin<super::gpio::Pa8 > for Pta8 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa8  { super::gpio::PA8 }
}

impl LinkGpioPin<super::gpio::Pa9 > for Pta9 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa9  { super::gpio::PA9 }
}

impl LinkGpioPin<super::gpio::Pa10 > for Pta10 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa10  { super::gpio::PA10 }
}

impl LinkGpioPin<super::gpio::Pa11 > for Pta11 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa11  { super::gpio::PA11 }
}

impl LinkGpioPin<super::gpio::Pa12 > for Pta12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa12  { super::gpio::PA12 }
}

impl LinkGpioPin<super::gpio::Pa13 > for Pta13 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa13  { super::gpio::PA13 }
}

impl LinkGpioPin<super::gpio::Pa14 > for Pta14 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa14  { super::gpio::PA14 }
}

impl LinkGpioPin<super::gpio::Pa15 > for Pta15 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa15  { super::gpio::PA15 }
}

impl LinkGpioPin<super::gpio::Pa16 > for Pta16 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa16  { super::gpio::PA16 }
}

impl LinkGpioPin<super::gpio::Pa17 > for Pta17 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa17  { super::gpio::PA17 }
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

impl LinkGpioPin<super::gpio::Pb4 > for Ptb4 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb4  { super::gpio::PB4 }
}

impl LinkGpioPin<super::gpio::Pb5 > for Ptb5 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb5  { super::gpio::PB5 }
}

impl LinkGpioPin<super::gpio::Pb6 > for Ptb6 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb6  { super::gpio::PB6 }
}

impl LinkGpioPin<super::gpio::Pb7 > for Ptb7 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb7  { super::gpio::PB7 }
}

impl LinkGpioPin<super::gpio::Pb8 > for Ptb8 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb8  { super::gpio::PB8 }
}

impl LinkGpioPin<super::gpio::Pb9 > for Ptb9 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb9  { super::gpio::PB9 }
}

impl LinkGpioPin<super::gpio::Pb10 > for Ptb10 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb10  { super::gpio::PB10 }
}

impl LinkGpioPin<super::gpio::Pb11 > for Ptb11 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb11  { super::gpio::PB11 }
}

impl LinkGpioPin<super::gpio::Pb12 > for Ptb12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb12  { super::gpio::PB12 }
}

impl LinkGpioPin<super::gpio::Pb13 > for Ptb13 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb13  { super::gpio::PB13 }
}

impl LinkGpioPin<super::gpio::Pb14 > for Ptb14 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb14  { super::gpio::PB14 }
}

impl LinkGpioPin<super::gpio::Pb15 > for Ptb15 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb15  { super::gpio::PB15 }
}

impl LinkGpioPin<super::gpio::Pb16 > for Ptb16 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb16  { super::gpio::PB16 }
}

impl LinkGpioPin<super::gpio::Pb17 > for Ptb17 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb17  { super::gpio::PB17 }
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

impl LinkGpioPin<super::gpio::Pc12 > for Ptc12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc12  { super::gpio::PC12 }
}

impl LinkGpioPin<super::gpio::Pc13 > for Ptc13 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc13  { super::gpio::PC13 }
}

impl LinkGpioPin<super::gpio::Pc14 > for Ptc14 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc14  { super::gpio::PC14 }
}

impl LinkGpioPin<super::gpio::Pc15 > for Ptc15 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc15  { super::gpio::PC15 }
}

impl LinkGpioPin<super::gpio::Pc16 > for Ptc16 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc16  { super::gpio::PC16 }
}

impl LinkGpioPin<super::gpio::Pc17 > for Ptc17 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc17  { super::gpio::PC17 }
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

impl LinkGpioPin<super::gpio::Pd8 > for Ptd8 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd8  { super::gpio::PD8 }
}

impl LinkGpioPin<super::gpio::Pd9 > for Ptd9 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd9  { super::gpio::PD9 }
}

impl LinkGpioPin<super::gpio::Pd10 > for Ptd10 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd10  { super::gpio::PD10 }
}

impl LinkGpioPin<super::gpio::Pd11 > for Ptd11 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd11  { super::gpio::PD11 }
}

impl LinkGpioPin<super::gpio::Pd12 > for Ptd12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd12  { super::gpio::PD12 }
}

impl LinkGpioPin<super::gpio::Pd13 > for Ptd13 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd13  { super::gpio::PD13 }
}

impl LinkGpioPin<super::gpio::Pd14 > for Ptd14 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd14  { super::gpio::PD14 }
}

impl LinkGpioPin<super::gpio::Pd15 > for Ptd15 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd15  { super::gpio::PD15 }
}

impl LinkGpioPin<super::gpio::Pd16 > for Ptd16 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd16  { super::gpio::PD16 }
}

impl LinkGpioPin<super::gpio::Pd17 > for Ptd17 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd17  { super::gpio::PD17 }
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

impl LinkGpioPin<super::gpio::Pe2 > for Pte2 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe2  { super::gpio::PE2 }
}

impl LinkGpioPin<super::gpio::Pe3 > for Pte3 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe3  { super::gpio::PE3 }
}

impl LinkGpioPin<super::gpio::Pe4 > for Pte4 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe4  { super::gpio::PE4 }
}

impl LinkGpioPin<super::gpio::Pe5 > for Pte5 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe5  { super::gpio::PE5 }
}

impl LinkGpioPin<super::gpio::Pe6 > for Pte6 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe6  { super::gpio::PE6 }
}

impl LinkGpioPin<super::gpio::Pe7 > for Pte7 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe7  { super::gpio::PE7 }
}

impl LinkGpioPin<super::gpio::Pe8 > for Pte8 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe8  { super::gpio::PE8 }
}

impl LinkGpioPin<super::gpio::Pe9 > for Pte9 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe9  { super::gpio::PE9 }
}

impl LinkGpioPin<super::gpio::Pe10 > for Pte10 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe10  { super::gpio::PE10 }
}

impl LinkGpioPin<super::gpio::Pe11 > for Pte11 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe11  { super::gpio::PE11 }
}

impl LinkGpioPin<super::gpio::Pe12 > for Pte12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe12  { super::gpio::PE12 }
}

impl LinkGpioPin<super::gpio::Pe13 > for Pte13 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe13  { super::gpio::PE13 }
}

impl LinkGpioPin<super::gpio::Pe14 > for Pte14 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe14  { super::gpio::PE14 }
}

impl LinkGpioPin<super::gpio::Pe15 > for Pte15 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe15  { super::gpio::PE15 }
}

impl LinkGpioPin<super::gpio::Pe16 > for Pte16 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe16  { super::gpio::PE16 }
}

