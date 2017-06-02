pub use kinetis_common::chip::port::*;

pub trait GpioLink {
   fn gpio(&self) -> &::core::ops::Deref<Target=super::gpio::GpioImpl>;
}

pub const PORTA: Porta = Porta {};
pub const PORTA_REF: &Porta = &PORTA;
pub const PORTA_IMPL: PortImpl = PortImpl(0x40049000);
pub const PORTA_IMPL_REF: &PortImpl = &PORTA_IMPL;

pub struct Porta {}
impl ::core::ops::Deref for Porta {
   type Target = PortImpl;
   #[inline]
   fn deref(&self) -> &PortImpl { PORTA_IMPL_REF }
}

impl GpioLink for Porta {
   fn gpio(&self) -> &::core::ops::Deref<Target=super::gpio::GpioImpl> { super::gpio::GPIOA_REF }
}


pub const PORTB: Portb = Portb {};
pub const PORTB_REF: &Portb = &PORTB;
pub const PORTB_IMPL: PortImpl = PortImpl(0x4004a000);
pub const PORTB_IMPL_REF: &PortImpl = &PORTB_IMPL;

pub struct Portb {}
impl ::core::ops::Deref for Portb {
   type Target = PortImpl;
   #[inline]
   fn deref(&self) -> &PortImpl { PORTB_IMPL_REF }
}

impl GpioLink for Portb {
   fn gpio(&self) -> &::core::ops::Deref<Target=super::gpio::GpioImpl> { super::gpio::GPIOB_REF }
}


pub const PORTC: Portc = Portc {};
pub const PORTC_REF: &Portc = &PORTC;
pub const PORTC_IMPL: PortImpl = PortImpl(0x4004b000);
pub const PORTC_IMPL_REF: &PortImpl = &PORTC_IMPL;

pub struct Portc {}
impl ::core::ops::Deref for Portc {
   type Target = PortImpl;
   #[inline]
   fn deref(&self) -> &PortImpl { PORTC_IMPL_REF }
}

impl GpioLink for Portc {
   fn gpio(&self) -> &::core::ops::Deref<Target=super::gpio::GpioImpl> { super::gpio::GPIOC_REF }
}


pub const PORTD: Portd = Portd {};
pub const PORTD_REF: &Portd = &PORTD;
pub const PORTD_IMPL: PortImpl = PortImpl(0x4004c000);
pub const PORTD_IMPL_REF: &PortImpl = &PORTD_IMPL;

pub struct Portd {}
impl ::core::ops::Deref for Portd {
   type Target = PortImpl;
   #[inline]
   fn deref(&self) -> &PortImpl { PORTD_IMPL_REF }
}

impl GpioLink for Portd {
   fn gpio(&self) -> &::core::ops::Deref<Target=super::gpio::GpioImpl> { super::gpio::GPIOD_REF }
}


pub const PORTE: Porte = Porte {};
pub const PORTE_REF: &Porte = &PORTE;
pub const PORTE_IMPL: PortImpl = PortImpl(0x4004d000);
pub const PORTE_IMPL_REF: &PortImpl = &PORTE_IMPL;

pub struct Porte {}
impl ::core::ops::Deref for Porte {
   type Target = PortImpl;
   #[inline]
   fn deref(&self) -> &PortImpl { PORTE_IMPL_REF }
}

impl GpioLink for Porte {
   fn gpio(&self) -> &::core::ops::Deref<Target=super::gpio::GpioImpl> { super::gpio::GPIOE_REF }
}



pub const PTA0: Pta0 = Pta0 {}; 
pub const PTA0_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 0 };
pub const PTA0_IMPL_REF: &PinImpl = &PTA0_IMPL;

impl ::core::ops::Deref for Pta0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta0 {}

impl Pin<Porta> for Pta0 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se0> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In0> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta0> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch1> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Scls> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD2> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm2QdPha> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Lpuart0Cts> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut3> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA1: Pta1 = Pta1 {}; 
pub const PTA1_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 1 };
pub const PTA1_IMPL_REF: &PinImpl = &PTA1_IMPL;

impl ::core::ops::Deref for Pta1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta1 {}

impl Pin<Porta> for Pta1 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Adc0Se1> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In1> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta1> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch1> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Sdas> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD3> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPha> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Lpuart0Rts> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut0> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA2: Pta2 = Pta2 {}; 
pub const PTA2_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 2 };
pub const PTA2_IMPL_REF: &PinImpl = &PTA2_IMPL;

impl ::core::ops::Deref for Pta2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta2 {}

impl Pin<Porta> for Pta2 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Adc1Se0> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta2> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch0> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Sda> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::EwmOutB> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD4> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Lpuart0Rx> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA3: Pta3 = Pta3 {}; 
pub const PTA3_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 3 };
pub const PTA3_IMPL_REF: &PinImpl = &PTA3_IMPL;

impl ::core::ops::Deref for Pta3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta3 {}

impl Pin<Porta> for Pta3 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Adc1Se1> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta3> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch1> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Scl> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::EwmIn> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD5> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Lpuart0Tx> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA4: Pta4 = Pta4 {}; 
pub const PTA4_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 4 };
pub const PTA4_IMPL_REF: &PinImpl = &PTA4_IMPL;

impl ::core::ops::Deref for Pta4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta4 {}

impl Pin<Porta> for Pta4 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Pta4> for Pta4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Cmp0Out> for Pta4 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::EwmOutB> for Pta4 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::JtagTms> for Pta4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::SwdDio> for Pta4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA5: Pta5 = Pta5 {}; 
pub const PTA5_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 5 };
pub const PTA5_IMPL_REF: &PinImpl = &PTA5_IMPL;

impl ::core::ops::Deref for Pta5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta5 {}

impl Pin<Porta> for Pta5 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Pta5> for Pta5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tclk1> for Pta5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::ResetB> for Pta5 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA6: Pta6 = Pta6 {}; 
pub const PTA6_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 6 };
pub const PTA6_IMPL_REF: &PinImpl = &PTA6_IMPL;

impl ::core::ops::Deref for Pta6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta6 {}

impl Pin<Porta> for Pta6 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Adc0Se2> for Pta6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta6> for Pta6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt1> for Pta6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Pcs1> for Pta6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart1Cts> for Pta6 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA7: Pta7 = Pta7 {}; 
pub const PTA7_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 7 };
pub const PTA7_IMPL_REF: &PinImpl = &PTA7_IMPL;

impl ::core::ops::Deref for Pta7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta7 {}

impl Pin<Porta> for Pta7 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Adc0Se3> for Pta7 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta7> for Pta7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt2> for Pta7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::RtcClkin> for Pta7 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lpuart1Rts> for Pta7 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA8: Pta8 = Pta8 {}; 
pub const PTA8_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 8 };
pub const PTA8_IMPL_REF: &PinImpl = &PTA8_IMPL;

impl ::core::ops::Deref for Pta8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta8 {}

impl Pin<Porta> for Pta8 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Pta8> for Pta8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart2Rx> for Pta8 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sout> for Pta8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD6> for Pta8 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Flt3> for Pta8 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTA9: Pta9 = Pta9 {}; 
pub const PTA9_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 9 };
pub const PTA9_IMPL_REF: &PinImpl = &PTA9_IMPL;

impl ::core::ops::Deref for Pta9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta9 {}

impl Pin<Porta> for Pta9 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Pta9> for Pta9 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart2Tx> for Pta9 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Pcs0> for Pta9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD7> for Pta9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Flt2> for Pta9 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm1Flt3> for Pta9 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA10: Pta10 = Pta10 {}; 
pub const PTA10_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 10 };
pub const PTA10_IMPL_REF: &PinImpl = &PTA10_IMPL;

impl ::core::ops::Deref for Pta10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta10 {}

impl Pin<Porta> for Pta10 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Pta10> for Pta10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch4> for Pta10 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FxioD0> for Pta10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::JtagTdo> for Pta10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::NoetmTraceSwo> for Pta10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA11: Pta11 = Pta11 {}; 
pub const PTA11_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 11 };
pub const PTA11_IMPL_REF: &PinImpl = &PTA11_IMPL;

impl ::core::ops::Deref for Pta11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta11 {}

impl Pin<Porta> for Pta11 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Pta11> for Pta11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch5> for Pta11 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FxioD1> for Pta11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp0Rrt> for Pta11 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTA12: Pta12 = Pta12 {}; 
pub const PTA12_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 12 };
pub const PTA12_IMPL_REF: &PinImpl = &PTA12_IMPL;

impl ::core::ops::Deref for Pta12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta12 {}

impl Pin<Porta> for Pta12 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Pta12> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch6> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can1Rx> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA13: Pta13 = Pta13 {}; 
pub const PTA13_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 13 };
pub const PTA13_IMPL_REF: &PinImpl = &PTA13_IMPL;

impl ::core::ops::Deref for Pta13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta13 {}

impl Pin<Porta> for Pta13 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Pta13> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch7> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can1Tx> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2QdPha> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA14: Pta14 = Pta14 {}; 
pub const PTA14_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 14 };
pub const PTA14_IMPL_REF: &PinImpl = &PTA14_IMPL;

impl ::core::ops::Deref for Pta14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta14 {}

impl Pin<Porta> for Pta14 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Pta14> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt0> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt1> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::EwmIn> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1Flt0> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA15: Pta15 = Pta15 {}; 
pub const PTA15_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 15 };
pub const PTA15_IMPL_REF: &PinImpl = &PTA15_IMPL;

impl ::core::ops::Deref for Pta15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta15 {}

impl Pin<Porta> for Pta15 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Adc1Se12> for Pta15 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta15> for Pta15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch2> for Pta15 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Pcs3> for Pta15 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpspi2Pcs3> for Pta15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTA16: Pta16 = Pta16 {}; 
pub const PTA16_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 16 };
pub const PTA16_IMPL_REF: &PinImpl = &PTA16_IMPL;

impl ::core::ops::Deref for Pta16 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA16_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta16 {}

impl Pin<Porta> for Pta16 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 16 }
}

impl AltFn<super::sig::Adc1Se13> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta16> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch3> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Pcs2> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTA17: Pta17 = Pta17 {}; 
pub const PTA17_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 17 };
pub const PTA17_IMPL_REF: &PinImpl = &PTA17_IMPL;

impl ::core::ops::Deref for Pta17 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA17_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta17 {}

impl Pin<Porta> for Pta17 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 17 }
}

impl AltFn<super::sig::Pta17> for Pta17 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch6> for Pta17 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt0> for Pta17 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::EwmOutB> for Pta17 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTB0: Ptb0 = Ptb0 {}; 
pub const PTB0_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 0 };
pub const PTB0_IMPL_REF: &PinImpl = &PTB0_IMPL;

impl ::core::ops::Deref for Ptb0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb0 {}

impl Pin<Portb> for Ptb0 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se4> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se14> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb0> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart0Rx> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Pcs0> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lptmr0Alt3> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Can0Rx> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTB1: Ptb1 = Ptb1 {}; 
pub const PTB1_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 1 };
pub const PTB1_IMPL_REF: &PinImpl = &PTB1_IMPL;

impl ::core::ops::Deref for Ptb1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb1 {}

impl Pin<Portb> for Ptb1 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Adc0Se5> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se15> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb1> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart0Tx> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sout> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tclk0> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Can0Tx> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTB2: Ptb2 = Ptb2 {}; 
pub const PTB2_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 2 };
pub const PTB2_IMPL_REF: &PinImpl = &PTB2_IMPL;

impl ::core::ops::Deref for Ptb2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb2 {}

impl Pin<Portb> for Ptb2 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Adc0Se6> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb2> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch0> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sck> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TrgmuxIn3> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB3: Ptb3 = Ptb3 {}; 
pub const PTB3_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 3 };
pub const PTB3_IMPL_REF: &PinImpl = &PTB3_IMPL;

impl ::core::ops::Deref for Ptb3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb3 {}

impl Pin<Portb> for Ptb3 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Adc0Se7> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb3> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch1> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sin> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm1QdPha> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TrgmuxIn2> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB4: Ptb4 = Ptb4 {}; 
pub const PTB4_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 4 };
pub const PTB4_IMPL_REF: &PinImpl = &PTB4_IMPL;

impl ::core::ops::Deref for Ptb4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb4 {}

impl Pin<Portb> for Ptb4 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ptb4> for Ptb4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch4> for Ptb4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sout> for Ptb4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TrgmuxIn1> for Ptb4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB5: Ptb5 = Ptb5 {}; 
pub const PTB5_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 5 };
pub const PTB5_IMPL_REF: &PinImpl = &PTB5_IMPL;

impl ::core::ops::Deref for Ptb5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb5 {}

impl Pin<Portb> for Ptb5 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ptb5> for Ptb5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch5> for Ptb5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Pcs1> for Ptb5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpspi0Pcs0> for Ptb5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Clkout> for Ptb5 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::TrgmuxIn0> for Ptb5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB6: Ptb6 = Ptb6 {}; 
pub const PTB6_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 6 };
pub const PTB6_IMPL_REF: &PinImpl = &PTB6_IMPL;

impl ::core::ops::Deref for Ptb6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb6 {}

impl Pin<Portb> for Ptb6 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Xtal> for Ptb6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb6> for Ptb6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpi2c0Sda> for Ptb6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PTB7: Ptb7 = Ptb7 {}; 
pub const PTB7_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 7 };
pub const PTB7_IMPL_REF: &PinImpl = &PTB7_IMPL;

impl ::core::ops::Deref for Ptb7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb7 {}

impl Pin<Portb> for Ptb7 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Extal> for Ptb7 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb7> for Ptb7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpi2c0Scl> for Ptb7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PTB8: Ptb8 = Ptb8 {}; 
pub const PTB8_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 8 };
pub const PTB8_IMPL_REF: &PinImpl = &PTB8_IMPL;

impl ::core::ops::Deref for Ptb8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb8 {}

impl Pin<Portb> for Ptb8 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Ptb8> for Ptb8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch0> for Ptb8 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PTB9: Ptb9 = Ptb9 {}; 
pub const PTB9_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 9 };
pub const PTB9_IMPL_REF: &PinImpl = &PTB9_IMPL;

impl ::core::ops::Deref for Ptb9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb9 {}

impl Pin<Portb> for Ptb9 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Ptb9> for Ptb9 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch1> for Ptb9 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Scls> for Ptb9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB10: Ptb10 = Ptb10 {}; 
pub const PTB10_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 10 };
pub const PTB10_IMPL_REF: &PinImpl = &PTB10_IMPL;

impl ::core::ops::Deref for Ptb10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb10 {}

impl Pin<Portb> for Ptb10 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Ptb10> for Ptb10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch2> for Ptb10 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Sdas> for Ptb10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB11: Ptb11 = Ptb11 {}; 
pub const PTB11_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 11 };
pub const PTB11_IMPL_REF: &PinImpl = &PTB11_IMPL;

impl ::core::ops::Deref for Ptb11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb11 {}

impl Pin<Portb> for Ptb11 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Ptb11> for Ptb11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch3> for Ptb11 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Hreq> for Ptb11 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB12: Ptb12 = Ptb12 {}; 
pub const PTB12_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 12 };
pub const PTB12_IMPL_REF: &PinImpl = &PTB12_IMPL;

impl ::core::ops::Deref for Ptb12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb12 {}

impl Pin<Portb> for Ptb12 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Adc1Se7> for Ptb12 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb12> for Ptb12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch0> for Ptb12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt2> for Ptb12 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can2Rx> for Ptb12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTB13: Ptb13 = Ptb13 {}; 
pub const PTB13_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 13 };
pub const PTB13_IMPL_REF: &PinImpl = &PTB13_IMPL;

impl ::core::ops::Deref for Ptb13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb13 {}

impl Pin<Portb> for Ptb13 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Adc1Se8> for Ptb13 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se8> for Ptb13 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb13> for Ptb13 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch1> for Ptb13 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt1> for Ptb13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can2Tx> for Ptb13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTB14: Ptb14 = Ptb14 {}; 
pub const PTB14_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 14 };
pub const PTB14_IMPL_REF: &PinImpl = &PTB14_IMPL;

impl ::core::ops::Deref for Ptb14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb14 {}

impl Pin<Portb> for Ptb14 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Adc1Se9> for Ptb14 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se9> for Ptb14 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb14> for Ptb14 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch2> for Ptb14 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sck> for Ptb14 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB15: Ptb15 = Ptb15 {}; 
pub const PTB15_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 15 };
pub const PTB15_IMPL_REF: &PinImpl = &PTB15_IMPL;

impl ::core::ops::Deref for Ptb15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb15 {}

impl Pin<Portb> for Ptb15 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Adc1Se14> for Ptb15 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb15> for Ptb15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch3> for Ptb15 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sin> for Ptb15 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB16: Ptb16 = Ptb16 {}; 
pub const PTB16_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 16 };
pub const PTB16_IMPL_REF: &PinImpl = &PTB16_IMPL;

impl ::core::ops::Deref for Ptb16 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB16_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb16 {}

impl Pin<Portb> for Ptb16 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 16 }
}

impl AltFn<super::sig::Adc1Se15> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb16> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch4> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sout> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB17: Ptb17 = Ptb17 {}; 
pub const PTB17_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 17 };
pub const PTB17_IMPL_REF: &PinImpl = &PTB17_IMPL;

impl ::core::ops::Deref for Ptb17 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB17_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb17 {}

impl Pin<Portb> for Ptb17 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 17 }
}

impl AltFn<super::sig::Ptb17> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch5> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Pcs3> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTC0: Ptc0 = Ptc0 {}; 
pub const PTC0_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 0 };
pub const PTC0_IMPL_REF: &PinImpl = &PTC0_IMPL;

impl ::core::ops::Deref for Ptc0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc0 {}

impl Pin<Portc> for Ptc0 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se8> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc0> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch0> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sin> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm1Ch6> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC1: Ptc1 = Ptc1 {}; 
pub const PTC1_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 1 };
pub const PTC1_IMPL_REF: &PinImpl = &PTC1_IMPL;

impl ::core::ops::Deref for Ptc1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc1 {}

impl Pin<Portc> for Ptc1 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Adc0Se9> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc1> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch1> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sout> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm1Ch7> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC2: Ptc2 = Ptc2 {}; 
pub const PTC2_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 2 };
pub const PTC2_IMPL_REF: &PinImpl = &PTC2_IMPL;

impl ::core::ops::Deref for Ptc2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc2 {}

impl Pin<Portc> for Ptc2 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Adc0Se10> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In5> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc2> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch2> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can0Rx> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart0Rx> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC3: Ptc3 = Ptc3 {}; 
pub const PTC3_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 3 };
pub const PTC3_IMPL_REF: &PinImpl = &PTC3_IMPL;

impl ::core::ops::Deref for Ptc3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc3 {}

impl Pin<Portc> for Ptc3 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Adc0Se11> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In4> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc3> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch3> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can0Tx> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart0Tx> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC4: Ptc4 = Ptc4 {}; 
pub const PTC4_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 4 };
pub const PTC4_IMPL_REF: &PinImpl = &PTC4_IMPL;

impl ::core::ops::Deref for Ptc4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc4 {}

impl Pin<Portc> for Ptc4 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp0In2> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc4> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch0> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::RtcClkout> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::EwmIn> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::JtagTclk> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::SwdClk> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTC5: Ptc5 = Ptc5 {}; 
pub const PTC5_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 5 };
pub const PTC5_IMPL_REF: &PinImpl = &PTC5_IMPL;

impl ::core::ops::Deref for Ptc5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc5 {}

impl Pin<Portc> for Ptc5 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ptc5> for Ptc5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch0> for Ptc5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::RtcClkout> for Ptc5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Ptc5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::JtagTdi> for Ptc5 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTC6: Ptc6 = Ptc6 {}; 
pub const PTC6_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 6 };
pub const PTC6_IMPL_REF: &PinImpl = &PTC6_IMPL;

impl ::core::ops::Deref for Ptc6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc6 {}

impl Pin<Portc> for Ptc6 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Adc1Se4> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc6> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Rx> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can1Rx> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch2> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC7: Ptc7 = Ptc7 {}; 
pub const PTC7_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 7 };
pub const PTC7_IMPL_REF: &PinImpl = &PTC7_IMPL;

impl ::core::ops::Deref for Ptc7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc7 {}

impl Pin<Portc> for Ptc7 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Adc1Se5> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc7> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Tx> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can1Tx> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch3> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPha> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC8: Ptc8 = Ptc8 {}; 
pub const PTC8_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 8 };
pub const PTC8_IMPL_REF: &PinImpl = &PTC8_IMPL;

impl ::core::ops::Deref for Ptc8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc8 {}

impl Pin<Portc> for Ptc8 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Ptc8> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Rx> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Flt0> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart0Cts> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC9: Ptc9 = Ptc9 {}; 
pub const PTC9_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 9 };
pub const PTC9_IMPL_REF: &PinImpl = &PTC9_IMPL;

impl ::core::ops::Deref for Ptc9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc9 {}

impl Pin<Portc> for Ptc9 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Ptc9> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Tx> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Flt1> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart0Rts> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC10: Ptc10 = Ptc10 {}; 
pub const PTC10_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 10 };
pub const PTC10_IMPL_REF: &PinImpl = &PTC10_IMPL;

impl ::core::ops::Deref for Ptc10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc10 {}

impl Pin<Portc> for Ptc10 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Ptc10> for Ptc10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch4> for Ptc10 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TrgmuxIn11> for Ptc10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC11: Ptc11 = Ptc11 {}; 
pub const PTC11_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 11 };
pub const PTC11_IMPL_REF: &PinImpl = &PTC11_IMPL;

impl ::core::ops::Deref for Ptc11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc11 {}

impl Pin<Portc> for Ptc11 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Ptc11> for Ptc11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch5> for Ptc11 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TrgmuxIn10> for Ptc11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC12: Ptc12 = Ptc12 {}; 
pub const PTC12_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 12 };
pub const PTC12_IMPL_REF: &PinImpl = &PTC12_IMPL;

impl ::core::ops::Deref for Ptc12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc12 {}

impl Pin<Portc> for Ptc12 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Ptc12> for Ptc12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch6> for Ptc12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Ch6> for Ptc12 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart2Cts> for Ptc12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC13: Ptc13 = Ptc13 {}; 
pub const PTC13_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 13 };
pub const PTC13_IMPL_REF: &PinImpl = &PTC13_IMPL;

impl ::core::ops::Deref for Ptc13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc13 {}

impl Pin<Portc> for Ptc13 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Ptc13> for Ptc13 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch7> for Ptc13 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Ch7> for Ptc13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart2Rts> for Ptc13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC14: Ptc14 = Ptc14 {}; 
pub const PTC14_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 14 };
pub const PTC14_IMPL_REF: &PinImpl = &PTC14_IMPL;

impl ::core::ops::Deref for Ptc14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc14 {}

impl Pin<Portc> for Ptc14 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Adc0Se12> for Ptc14 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc14> for Ptc14 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch2> for Ptc14 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Pcs0> for Ptc14 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TrgmuxIn9> for Ptc14 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC15: Ptc15 = Ptc15 {}; 
pub const PTC15_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 15 };
pub const PTC15_IMPL_REF: &PinImpl = &PTC15_IMPL;

impl ::core::ops::Deref for Ptc15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc15 {}

impl Pin<Portc> for Ptc15 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Adc0Se13> for Ptc15 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc15> for Ptc15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch3> for Ptc15 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sck> for Ptc15 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TrgmuxIn8> for Ptc15 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC16: Ptc16 = Ptc16 {}; 
pub const PTC16_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 16 };
pub const PTC16_IMPL_REF: &PinImpl = &PTC16_IMPL;

impl ::core::ops::Deref for Ptc16 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC16_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc16 {}

impl Pin<Portc> for Ptc16 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 16 }
}

impl AltFn<super::sig::Adc0Se14> for Ptc16 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc16> for Ptc16 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Flt2> for Ptc16 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can2Rx> for Ptc16 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTC17: Ptc17 = Ptc17 {}; 
pub const PTC17_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 17 };
pub const PTC17_IMPL_REF: &PinImpl = &PTC17_IMPL;

impl ::core::ops::Deref for Ptc17 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC17_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc17 {}

impl Pin<Portc> for Ptc17 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 17 }
}

impl AltFn<super::sig::Adc0Se15> for Ptc17 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc17> for Ptc17 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Flt3> for Ptc17 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can2Tx> for Ptc17 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTD0: Ptd0 = Ptd0 {}; 
pub const PTD0_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 0 };
pub const PTD0_IMPL_REF: &PinImpl = &PTD0_IMPL;

impl ::core::ops::Deref for Ptd0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd0 {}

impl Pin<Portd> for Ptd0 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd0> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch2> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sck> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch0> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD0> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut1> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD1: Ptd1 = Ptd1 {}; 
pub const PTD1_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 1 };
pub const PTD1_IMPL_REF: &PinImpl = &PTD1_IMPL;

impl ::core::ops::Deref for Ptd1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd1 {}

impl Pin<Portd> for Ptd1 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ptd1> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch3> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sin> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch1> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD1> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut2> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD2: Ptd2 = Ptd2 {}; 
pub const PTD2_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 2 };
pub const PTD2_IMPL_REF: &PinImpl = &PTD2_IMPL;

impl ::core::ops::Deref for Ptd2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd2 {}

impl Pin<Portd> for Ptd2 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Adc1Se2> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd2> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch4> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sout> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD4> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD6> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::TrgmuxIn5> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD3: Ptd3 = Ptd3 {}; 
pub const PTD3_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 3 };
pub const PTD3_IMPL_REF: &PinImpl = &PTD3_IMPL;

impl ::core::ops::Deref for Ptd3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd3 {}

impl Pin<Portd> for Ptd3 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Adc1Se3> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd3> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch5> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Pcs0> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD5> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD7> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::TrgmuxIn4> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::NmiB> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD4: Ptd4 = Ptd4 {}; 
pub const PTD4_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 4 };
pub const PTD4_IMPL_REF: &PinImpl = &PTD4_IMPL;

impl ::core::ops::Deref for Ptd4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd4 {}

impl Pin<Portd> for Ptd4 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Adc1Se6> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd4> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt3> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt3> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTD5: Ptd5 = Ptd5 {}; 
pub const PTD5_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 5 };
pub const PTD5_IMPL_REF: &PinImpl = &PTD5_IMPL;

impl ::core::ops::Deref for Ptd5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd5 {}

impl Pin<Portd> for Ptd5 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ptd5> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch3> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lptmr0Alt2> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Flt1> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TrgmuxIn7> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD6: Ptd6 = Ptd6 {}; 
pub const PTD6_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 6 };
pub const PTD6_IMPL_REF: &PinImpl = &PTD6_IMPL;

impl ::core::ops::Deref for Ptd6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd6 {}

impl Pin<Portd> for Ptd6 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Cmp0In7> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd6> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart2Rx> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Flt2> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTD7: Ptd7 = Ptd7 {}; 
pub const PTD7_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 7 };
pub const PTD7_IMPL_REF: &PinImpl = &PTD7_IMPL;

impl ::core::ops::Deref for Ptd7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd7 {}

impl Pin<Portd> for Ptd7 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Cmp0In6> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd7> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart2Tx> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Flt3> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTD8: Ptd8 = Ptd8 {}; 
pub const PTD8_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 8 };
pub const PTD8_IMPL_REF: &PinImpl = &PTD8_IMPL;

impl ::core::ops::Deref for Ptd8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd8 {}

impl Pin<Portd> for Ptd8 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Ptd8> for Ptd8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Flt2> for Ptd8 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD1> for Ptd8 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm1Ch4> for Ptd8 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD9: Ptd9 = Ptd9 {}; 
pub const PTD9_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 9 };
pub const PTD9_IMPL_REF: &PinImpl = &PTD9_IMPL;

impl ::core::ops::Deref for Ptd9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd9 {}

impl Pin<Portd> for Ptd9 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Ptd9> for Ptd9 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FxioD0> for Ptd9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Flt3> for Ptd9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1Ch5> for Ptd9 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD10: Ptd10 = Ptd10 {}; 
pub const PTD10_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 10 };
pub const PTD10_IMPL_REF: &PinImpl = &PTD10_IMPL;

impl ::core::ops::Deref for Ptd10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd10 {}

impl Pin<Portd> for Ptd10 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Ptd10> for Ptd10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch0> for Ptd10 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Ptd10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTD11: Ptd11 = Ptd11 {}; 
pub const PTD11_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 11 };
pub const PTD11_IMPL_REF: &PinImpl = &PTD11_IMPL;

impl ::core::ops::Deref for Ptd11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd11 {}

impl Pin<Portd> for Ptd11 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Ptd11> for Ptd11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch1> for Ptd11 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2QdPha> for Ptd11 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart2Cts> for Ptd11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD12: Ptd12 = Ptd12 {}; 
pub const PTD12_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 12 };
pub const PTD12_IMPL_REF: &PinImpl = &PTD12_IMPL;

impl ::core::ops::Deref for Ptd12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd12 {}

impl Pin<Portd> for Ptd12 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Ptd12> for Ptd12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch2> for Ptd12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart2Rts> for Ptd12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD13: Ptd13 = Ptd13 {}; 
pub const PTD13_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 13 };
pub const PTD13_IMPL_REF: &PinImpl = &PTD13_IMPL;

impl ::core::ops::Deref for Ptd13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd13 {}

impl Pin<Portd> for Ptd13 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Ptd13> for Ptd13 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch4> for Ptd13 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart1Rx> for Ptd13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::RtcClkout> for Ptd13 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD14: Ptd14 = Ptd14 {}; 
pub const PTD14_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 14 };
pub const PTD14_IMPL_REF: &PinImpl = &PTD14_IMPL;

impl ::core::ops::Deref for Ptd14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd14 {}

impl Pin<Portd> for Ptd14 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Ptd14> for Ptd14 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch5> for Ptd14 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart1Tx> for Ptd14 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Clkout> for Ptd14 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD15: Ptd15 = Ptd15 {}; 
pub const PTD15_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 15 };
pub const PTD15_IMPL_REF: &PinImpl = &PTD15_IMPL;

impl ::core::ops::Deref for Ptd15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd15 {}

impl Pin<Portd> for Ptd15 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Ptd15> for Ptd15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch0> for Ptd15 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sck> for Ptd15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTD16: Ptd16 = Ptd16 {}; 
pub const PTD16_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 16 };
pub const PTD16_IMPL_REF: &PinImpl = &PTD16_IMPL;

impl ::core::ops::Deref for Ptd16 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD16_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd16 {}

impl Pin<Portd> for Ptd16 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 16 }
}

impl AltFn<super::sig::Ptd16> for Ptd16 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch1> for Ptd16 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sin> for Ptd16 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp0Rrt> for Ptd16 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTD17: Ptd17 = Ptd17 {}; 
pub const PTD17_IMPL: PinImpl = PinImpl { port: PORTD_IMPL, index: 17 };
pub const PTD17_IMPL_REF: &PinImpl = &PTD17_IMPL;

impl ::core::ops::Deref for Ptd17 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTD17_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd17 {}

impl Pin<Portd> for Ptd17 {
   #[inline]
   fn port(&self) -> Portd { PORTD }
   #[inline]
   fn index(&self) -> usize { 17 }
}

impl AltFn<super::sig::Ptd17> for Ptd17 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt2> for Ptd17 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart2Rx> for Ptd17 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTE0: Pte0 = Pte0 {}; 
pub const PTE0_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 0 };
pub const PTE0_IMPL_REF: &PinImpl = &PTE0_IMPL;

impl ::core::ops::Deref for Pte0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte0 {}

impl Pin<Porte> for Pte0 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte0> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi0Sck> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tclk1> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpspi1Sout> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm1Flt2> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE1: Pte1 = Pte1 {}; 
pub const PTE1_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 1 };
pub const PTE1_IMPL_REF: &PinImpl = &PTE1_IMPL;

impl ::core::ops::Deref for Pte1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte1 {}

impl Pin<Porte> for Pte1 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Pte1> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi0Sin> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Hreq> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpspi1Pcs0> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm1Flt1> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE2: Pte2 = Pte2 {}; 
pub const PTE2_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 2 };
pub const PTE2_IMPL_REF: &PinImpl = &PTE2_IMPL;

impl ::core::ops::Deref for Pte2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte2 {}

impl Pin<Porte> for Pte2 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Adc1Se10> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte2> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi0Sout> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lptmr0Alt3> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch6> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lpuart1Cts> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE3: Pte3 = Pte3 {}; 
pub const PTE3_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 3 };
pub const PTE3_IMPL_REF: &PinImpl = &PTE3_IMPL;

impl ::core::ops::Deref for Pte3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte3 {}

impl Pin<Porte> for Pte3 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Pte3> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt0> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart2Rts> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Flt0> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TrgmuxIn6> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Cmp0Out> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE4: Pte4 = Pte4 {}; 
pub const PTE4_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 4 };
pub const PTE4_IMPL_REF: &PinImpl = &PTE4_IMPL;

impl ::core::ops::Deref for Pte4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte4 {}

impl Pin<Porte> for Pte4 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Pte4> for Pte4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Pte4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch2> for Pte4 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Can0Rx> for Pte4 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FxioD6> for Pte4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::EwmOutB> for Pte4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE5: Pte5 = Pte5 {}; 
pub const PTE5_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 5 };
pub const PTE5_IMPL_REF: &PinImpl = &PTE5_IMPL;

impl ::core::ops::Deref for Pte5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte5 {}

impl Pin<Porte> for Pte5 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Pte5> for Pte5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tclk2> for Pte5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2QdPha> for Pte5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch3> for Pte5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Can0Tx> for Pte5 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FxioD7> for Pte5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::EwmIn> for Pte5 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE6: Pte6 = Pte6 {}; 
pub const PTE6_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 6 };
pub const PTE6_IMPL_REF: &PinImpl = &PTE6_IMPL;

impl ::core::ops::Deref for Pte6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte6 {}

impl Pin<Porte> for Pte6 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Adc1Se11> for Pte6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte6> for Pte6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi0Pcs2> for Pte6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Ch7> for Pte6 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lpuart1Rts> for Pte6 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE7: Pte7 = Pte7 {}; 
pub const PTE7_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 7 };
pub const PTE7_IMPL_REF: &PinImpl = &PTE7_IMPL;

impl ::core::ops::Deref for Pte7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte7 {}

impl Pin<Porte> for Pte7 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Pte7> for Pte7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch7> for Pte7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt0> for Pte7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTE8: Pte8 = Pte8 {}; 
pub const PTE8_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 8 };
pub const PTE8_IMPL_REF: &PinImpl = &PTE8_IMPL;

impl ::core::ops::Deref for Pte8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte8 {}

impl Pin<Porte> for Pte8 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Cmp0In3> for Pte8 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte8> for Pte8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch6> for Pte8 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PTE9: Pte9 = Pte9 {}; 
pub const PTE9_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 9 };
pub const PTE9_IMPL_REF: &PinImpl = &PTE9_IMPL;

impl ::core::ops::Deref for Pte9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte9 {}

impl Pin<Porte> for Pte9 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Pte9> for Pte9 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch7> for Pte9 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart2Cts> for Pte9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTE10: Pte10 = Pte10 {}; 
pub const PTE10_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 10 };
pub const PTE10_IMPL_REF: &PinImpl = &PTE10_IMPL;

impl ::core::ops::Deref for Pte10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte10 {}

impl Pin<Porte> for Pte10 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Pte10> for Pte10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Clkout> for Pte10 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Pcs1> for Pte10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch4> for Pte10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD4> for Pte10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut4> for Pte10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE11: Pte11 = Pte11 {}; 
pub const PTE11_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 11 };
pub const PTE11_IMPL_REF: &PinImpl = &PTE11_IMPL;

impl ::core::ops::Deref for Pte11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte11 {}

impl Pin<Porte> for Pte11 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Pte11> for Pte11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi2Pcs0> for Pte11 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lptmr0Alt1> for Pte11 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch5> for Pte11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD5> for Pte11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut5> for Pte11 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE12: Pte12 = Pte12 {}; 
pub const PTE12_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 12 };
pub const PTE12_IMPL_REF: &PinImpl = &PTE12_IMPL;

impl ::core::ops::Deref for Pte12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte12 {}

impl Pin<Porte> for Pte12 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Pte12> for Pte12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt3> for Pte12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart2Tx> for Pte12 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTE13: Pte13 = Pte13 {}; 
pub const PTE13_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 13 };
pub const PTE13_IMPL_REF: &PinImpl = &PTE13_IMPL;

impl ::core::ops::Deref for Pte13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte13 {}

impl Pin<Porte> for Pte13 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Pte13> for Pte13 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi2Pcs2> for Pte13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Flt0> for Pte13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE14: Pte14 = Pte14 {}; 
pub const PTE14_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 14 };
pub const PTE14_IMPL_REF: &PinImpl = &PTE14_IMPL;

impl ::core::ops::Deref for Pte14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte14 {}

impl Pin<Porte> for Pte14 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Pte14> for Pte14 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt1> for Pte14 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Flt1> for Pte14 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE15: Pte15 = Pte15 {}; 
pub const PTE15_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 15 };
pub const PTE15_IMPL_REF: &PinImpl = &PTE15_IMPL;

impl ::core::ops::Deref for Pte15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte15 {}

impl Pin<Porte> for Pte15 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Pte15> for Pte15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Cts> for Pte15 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sck> for Pte15 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch6> for Pte15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD2> for Pte15 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut6> for Pte15 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE16: Pte16 = Pte16 {}; 
pub const PTE16_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 16 };
pub const PTE16_IMPL_REF: &PinImpl = &PTE16_IMPL;

impl ::core::ops::Deref for Pte16 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE16_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte16 {}

impl Pin<Porte> for Pte16 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 16 }
}

impl AltFn<super::sig::Pte16> for Pte16 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Rts> for Pte16 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sin> for Pte16 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch7> for Pte16 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD3> for Pte16 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut7> for Pte16 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

