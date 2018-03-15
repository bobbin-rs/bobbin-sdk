#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x400ff000, 0x18);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x400ff040, 0x19);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x400ff080, 0x1a);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x400ff0c0, 0x1b);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, 0x400ff100, 0x1c);

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

