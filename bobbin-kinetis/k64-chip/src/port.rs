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

impl AltFn<super::sig::Pta0> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0CtsB> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0ColB> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm0Ch5> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::JtagTclk> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::SwdClk> for Pta0 {
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

impl AltFn<super::sig::Pta1> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0Rx> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm0Ch6> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::JtagTdi> for Pta1 {
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

impl AltFn<super::sig::Pta2> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0Tx> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm0Ch7> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::JtagTdo> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::TraceSwo> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Pta3> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0RtsB> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm0Ch0> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::JtagTms> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::SwdDio> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Ftm0Ch1> for Pta4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::NmiB> for Pta4 {
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

impl AltFn<super::sig::UsbClkin> for Pta5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm0Ch2> for Pta5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Rxer> for Pta5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Rxer> for Pta5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp2Out> for Pta5 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0TxBclk> for Pta5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::JtagTrstB> for Pta5 {
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

impl AltFn<super::sig::Pta6> for Pta6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch3> for Pta6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Clkout> for Pta6 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::TraceClkout> for Pta6 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Adc0Se10> for Pta7 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta7> for Pta7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch4> for Pta7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TraceD3> for Pta7 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Adc0Se11> for Pta8 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta8> for Pta8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch0> for Pta8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm1QdPha> for Pta8 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TraceD2> for Pta8 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Ftm1Ch1> for Pta9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Mii0Rxd3> for Pta9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Pta9 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TraceD1> for Pta9 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Ftm2Ch0> for Pta10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Mii0Rxd2> for Pta10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm2QdPha> for Pta10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TraceD0> for Pta10 {
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

impl AltFn<super::sig::Ftm2Ch1> for Pta11 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Mii0Rxclk> for Pta11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2c2Sda> for Pta11 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Pta11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Cmp2In0> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta12> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Can0Tx> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch0> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Rxd1> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Rxd1> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2c2Scl> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0Txd0> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ftm1QdPha> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Cmp2In1> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta13> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Can0Rx> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch1> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Rxd0> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Rxd0> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2c2Sda> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0TxFs> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Spi0Pcs0> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Tx> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0CrsDv> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Rxdv> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2c2Scl> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0RxBclk> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s0Txd1> for Pta14 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Pta15> for Pta15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sck> for Pta15 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Rx> for Pta15 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Txen> for Pta15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Txen> for Pta15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s0Rxd0> for Pta15 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Pta16> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sout> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0CtsB> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart0ColB> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Txd0> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Txd0> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s0RxFs> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s0Rxd1> for Pta16 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Adc1Se17> for Pta17 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta17> for Pta17 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sin> for Pta17 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0RtsB> for Pta17 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Txd1> for Pta17 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Txd1> for Pta17 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s0Mclk> for Pta17 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA18: Pta18 = Pta18 {}; 
pub const PTA18_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 18 };
pub const PTA18_IMPL_REF: &PinImpl = &PTA18_IMPL;

impl ::core::ops::Deref for Pta18 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA18_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta18 {}

impl Pin<Porta> for Pta18 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 18 }
}

impl AltFn<super::sig::Extal0> for Pta18 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta18> for Pta18 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt2> for Pta18 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FtmClkin0> for Pta18 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTA19: Pta19 = Pta19 {}; 
pub const PTA19_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 19 };
pub const PTA19_IMPL_REF: &PinImpl = &PTA19_IMPL;

impl ::core::ops::Deref for Pta19 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA19_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta19 {}

impl Pin<Porta> for Pta19 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 19 }
}

impl AltFn<super::sig::Xtal0> for Pta19 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta19> for Pta19 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Flt0> for Pta19 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FtmClkin1> for Pta19 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lptmr0Alt1> for Pta19 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA24: Pta24 = Pta24 {}; 
pub const PTA24_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 24 };
pub const PTA24_IMPL_REF: &PinImpl = &PTA24_IMPL;

impl ::core::ops::Deref for Pta24 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA24_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta24 {}

impl Pin<Porta> for Pta24 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 24 }
}

impl AltFn<super::sig::Pta24> for Pta24 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Txd2> for Pta24 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA29> for Pta24 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA25: Pta25 = Pta25 {}; 
pub const PTA25_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 25 };
pub const PTA25_IMPL_REF: &PinImpl = &PTA25_IMPL;

impl ::core::ops::Deref for Pta25 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA25_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta25 {}

impl Pin<Porta> for Pta25 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 25 }
}

impl AltFn<super::sig::Pta25> for Pta25 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Txclk> for Pta25 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA28> for Pta25 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA26: Pta26 = Pta26 {}; 
pub const PTA26_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 26 };
pub const PTA26_IMPL_REF: &PinImpl = &PTA26_IMPL;

impl ::core::ops::Deref for Pta26 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA26_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta26 {}

impl Pin<Porta> for Pta26 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 26 }
}

impl AltFn<super::sig::Pta26> for Pta26 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Txd3> for Pta26 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA27> for Pta26 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA27: Pta27 = Pta27 {}; 
pub const PTA27_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 27 };
pub const PTA27_IMPL_REF: &PinImpl = &PTA27_IMPL;

impl ::core::ops::Deref for Pta27 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA27_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta27 {}

impl Pin<Porta> for Pta27 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 27 }
}

impl AltFn<super::sig::Pta27> for Pta27 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Crs> for Pta27 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA26> for Pta27 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA28: Pta28 = Pta28 {}; 
pub const PTA28_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 28 };
pub const PTA28_IMPL_REF: &PinImpl = &PTA28_IMPL;

impl ::core::ops::Deref for Pta28 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA28_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta28 {}

impl Pin<Porta> for Pta28 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 28 }
}

impl AltFn<super::sig::Pta28> for Pta28 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Txer> for Pta28 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA25> for Pta28 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA29: Pta29 = Pta29 {}; 
pub const PTA29_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 29 };
pub const PTA29_IMPL_REF: &PinImpl = &PTA29_IMPL;

impl ::core::ops::Deref for Pta29 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA29_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta29 {}

impl Pin<Porta> for Pta29 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 29 }
}

impl AltFn<super::sig::Pta29> for Pta29 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Col> for Pta29 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA24> for Pta29 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Adc0Se8> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se8> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb0> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Scl> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch0> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Mdio> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Mdio> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPha> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Adc0Se9> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se9> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb1> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Sda> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch1> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Mdc> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Mdc> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Adc0Se12> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb2> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Scl> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0RtsB> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr0> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm0Flt3> for Ptb2 {
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

impl AltFn<super::sig::Adc0Se13> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb3> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Sda> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0CtsB> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart0ColB> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr1> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm0Flt0> for Ptb3 {
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

impl AltFn<super::sig::Adc1Se10> for Ptb4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb4> for Ptb4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Enet01588Tmr2> for Ptb4 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1Flt0> for Ptb4 {
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

impl AltFn<super::sig::Adc1Se11> for Ptb5 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb5> for Ptb5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Enet01588Tmr3> for Ptb5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm2Flt0> for Ptb5 {
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

impl AltFn<super::sig::Adc1Se12> for Ptb6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb6> for Ptb6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FbAd23> for Ptb6 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Adc1Se13> for Ptb7 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb7> for Ptb7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FbAd22> for Ptb7 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Uart3RtsB> for Ptb8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd21> for Ptb8 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Spi1Pcs1> for Ptb9 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3CtsB> for Ptb9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd20> for Ptb9 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Adc1Se14> for Ptb10 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb10> for Ptb10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Pcs0> for Ptb10 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3Rx> for Ptb10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd19> for Ptb10 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm0Flt1> for Ptb10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Adc1Se15> for Ptb11 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb11> for Ptb11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sck> for Ptb11 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3Tx> for Ptb11 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd18> for Ptb11 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm0Flt2> for Ptb11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Ptb12> for Ptb12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3RtsB> for Ptb12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch0> for Ptb12 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch4> for Ptb12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPha> for Ptb12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Ptb13> for Ptb13 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3CtsB> for Ptb13 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch1> for Ptb13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch5> for Ptb13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Ptb13 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Ptb16> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sout> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Rx> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FtmClkin0> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd17> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmIn> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Spi1Sin> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Tx> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FtmClkin1> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd16> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmOutB> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB18: Ptb18 = Ptb18 {}; 
pub const PTB18_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 18 };
pub const PTB18_IMPL_REF: &PinImpl = &PTB18_IMPL;

impl ::core::ops::Deref for Ptb18 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB18_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb18 {}

impl Pin<Portb> for Ptb18 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 18 }
}

impl AltFn<super::sig::Ptb18> for Ptb18 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Can0Tx> for Ptb18 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Ch0> for Ptb18 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxBclk> for Ptb18 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd15> for Ptb18 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm2QdPha> for Ptb18 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB19: Ptb19 = Ptb19 {}; 
pub const PTB19_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 19 };
pub const PTB19_IMPL_REF: &PinImpl = &PTB19_IMPL;

impl ::core::ops::Deref for Ptb19 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB19_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb19 {}

impl Pin<Portb> for Ptb19 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 19 }
}

impl AltFn<super::sig::Ptb19> for Ptb19 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Can0Rx> for Ptb19 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Ch1> for Ptb19 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxFs> for Ptb19 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbOeB> for Ptb19 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Ptb19 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB20: Ptb20 = Ptb20 {}; 
pub const PTB20_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 20 };
pub const PTB20_IMPL_REF: &PinImpl = &PTB20_IMPL;

impl ::core::ops::Deref for Ptb20 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB20_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb20 {}

impl Pin<Portb> for Ptb20 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 20 }
}

impl AltFn<super::sig::Ptb20> for Ptb20 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Pcs0> for Ptb20 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FbAd31> for Ptb20 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Cmp0Out> for Ptb20 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB21: Ptb21 = Ptb21 {}; 
pub const PTB21_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 21 };
pub const PTB21_IMPL_REF: &PinImpl = &PTB21_IMPL;

impl ::core::ops::Deref for Ptb21 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB21_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb21 {}

impl Pin<Portb> for Ptb21 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 21 }
}

impl AltFn<super::sig::Ptb21> for Ptb21 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sck> for Ptb21 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FbAd30> for Ptb21 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Cmp1Out> for Ptb21 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB22: Ptb22 = Ptb22 {}; 
pub const PTB22_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 22 };
pub const PTB22_IMPL_REF: &PinImpl = &PTB22_IMPL;

impl ::core::ops::Deref for Ptb22 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB22_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb22 {}

impl Pin<Portb> for Ptb22 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 22 }
}

impl AltFn<super::sig::Ptb22> for Ptb22 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sout> for Ptb22 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FbAd29> for Ptb22 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Cmp2Out> for Ptb22 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB23: Ptb23 = Ptb23 {}; 
pub const PTB23_IMPL: PinImpl = PinImpl { port: PORTB_IMPL, index: 23 };
pub const PTB23_IMPL_REF: &PinImpl = &PTB23_IMPL;

impl ::core::ops::Deref for Ptb23 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTB23_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb23 {}

impl Pin<Portb> for Ptb23 {
   #[inline]
   fn port(&self) -> Portb { PORTB }
   #[inline]
   fn index(&self) -> usize { 23 }
}

impl AltFn<super::sig::Ptb23> for Ptb23 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sin> for Ptb23 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Spi0Pcs5> for Ptb23 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd28> for Ptb23 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Adc0Se14> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc0> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs4> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Pdb0Extrg> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::UsbSofOut> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd14> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0Txd1> for Ptc0 {
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

impl AltFn<super::sig::Adc0Se15> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc1> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs3> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1RtsB> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch0> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd13> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0Txd0> for Ptc1 {
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

impl AltFn<super::sig::Adc0Se4b> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp1In0> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc2> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs2> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1CtsB> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch1> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd12> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0TxFs> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Cmp1In1> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc3> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs1> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Rx> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch2> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Clkout> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0TxBclk> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Ptc4> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs0> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Tx> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch3> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd11> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Cmp1Out> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Spi0Sck> for Ptc5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lptmr0Alt2> for Ptc5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Rxd0> for Ptc5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd10> for Ptc5 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Cmp0Out> for Ptc5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ftm0Ch2> for Ptc5 {
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

impl AltFn<super::sig::Cmp0In0> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc6> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sout> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Pdb0Extrg> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxBclk> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd9> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0Mclk> for Ptc6 {
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

impl AltFn<super::sig::Cmp0In1> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc7> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sin> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::UsbSofOut> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxFs> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd8> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Adc1Se4b> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In2> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc8> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch4> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Mclk> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd7> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Adc1Se5b> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In3> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc9> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch5> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxBclk> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd6> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm2Flt0> for Ptc9 {
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

impl AltFn<super::sig::Adc1Se6b> for Ptc10 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc10> for Ptc10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Scl> for Ptc10 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Ch6> for Ptc10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxFs> for Ptc10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd5> for Ptc10 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Adc1Se7b> for Ptc11 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc11> for Ptc11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Sda> for Ptc11 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Ch7> for Ptc11 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Rxd1> for Ptc11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbRwB> for Ptc11 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Uart4RtsB> for Ptc12 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd27> for Ptc12 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm3Flt0> for Ptc12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Uart4CtsB> for Ptc13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd26> for Ptc13 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Ptc14> for Ptc14 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4Rx> for Ptc14 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd25> for Ptc14 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Ptc15> for Ptc15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4Tx> for Ptc15 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd24> for Ptc15 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Ptc16> for Ptc16 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3Rx> for Ptc16 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr0> for Ptc16 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbCs5B> for Ptc16 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbTsiz1> for Ptc16 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbBe2316Bls158B> for Ptc16 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Ptc17> for Ptc17 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3Tx> for Ptc17 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr1> for Ptc17 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbCs4B> for Ptc17 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbTsiz0> for Ptc17 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbBe3124Bls70B> for Ptc17 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC18: Ptc18 = Ptc18 {}; 
pub const PTC18_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 18 };
pub const PTC18_IMPL_REF: &PinImpl = &PTC18_IMPL;

impl ::core::ops::Deref for Ptc18 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC18_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc18 {}

impl Pin<Portc> for Ptc18 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 18 }
}

impl AltFn<super::sig::Ptc18> for Ptc18 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3RtsB> for Ptc18 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr2> for Ptc18 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbTbstB> for Ptc18 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbCs2B> for Ptc18 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbBe158Bls2316B> for Ptc18 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC19: Ptc19 = Ptc19 {}; 
pub const PTC19_IMPL: PinImpl = PinImpl { port: PORTC_IMPL, index: 19 };
pub const PTC19_IMPL_REF: &PinImpl = &PTC19_IMPL;

impl ::core::ops::Deref for Ptc19 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTC19_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc19 {}

impl Pin<Portc> for Ptc19 {
   #[inline]
   fn port(&self) -> Portc { PORTC }
   #[inline]
   fn index(&self) -> usize { 19 }
}

impl AltFn<super::sig::Ptc19> for Ptc19 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3CtsB> for Ptc19 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr3> for Ptc19 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbCs3B> for Ptc19 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbBe70Bls3124B> for Ptc19 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbTaB> for Ptc19 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Spi0Pcs0> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2RtsB> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch0> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAle> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbCs1B> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbTsB> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Adc0Se5b> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd1> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sck> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2CtsB> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch1> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbCs0B> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Ptd2> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sout> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Rx> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch2> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd4> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c0Scl> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Ptd3> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sin> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Tx> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch3> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd3> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c0Sda> for Ptd3 {
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

impl AltFn<super::sig::Ptd4> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs1> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0RtsB> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch4> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd2> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmIn> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Spi1Pcs0> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Adc0Se6b> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd5> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs2> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0CtsB> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart0ColB> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch5> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd1> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmOutB> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Spi1Sck> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Adc0Se7b> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd6> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs3> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Rx> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch6> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd0> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm0Flt0> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Spi1Sout> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Ptd7> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::CmtIro> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Tx> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch7> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm0Flt1> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Spi1Sin> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::I2c0Scl> for Ptd8 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5Rx> for Ptd8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbA16> for Ptd8 {
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

impl AltFn<super::sig::I2c0Sda> for Ptd9 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5Tx> for Ptd9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbA17> for Ptd9 {
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

impl AltFn<super::sig::Uart5RtsB> for Ptd10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbA18> for Ptd10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Spi2Pcs0> for Ptd11 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5CtsB> for Ptd11 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0Clkin> for Ptd11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA19> for Ptd11 {
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

impl AltFn<super::sig::Spi2Sck> for Ptd12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt0> for Ptd12 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0D4> for Ptd12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA20> for Ptd12 {
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

impl AltFn<super::sig::Spi2Sout> for Ptd13 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sdhc0D5> for Ptd13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA21> for Ptd13 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Spi2Sin> for Ptd14 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sdhc0D6> for Ptd14 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA22> for Ptd14 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Spi2Pcs1> for Ptd15 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sdhc0D7> for Ptd15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA23> for Ptd15 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Adc1Se4a> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte0> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Pcs1> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Tx> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0D1> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TraceClkout> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c1Sda> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::RtcClkout> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Adc1Se5a> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte1> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sout> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Rx> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0D0> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TraceD3> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c1Scl> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Spi1Sin> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Adc0Dp2> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se6a> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte2> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sck> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1CtsB> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0Dclk> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TraceD2> for Pte2 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Adc0Dm2> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se7a> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte3> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sin> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1RtsB> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0Cmd> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TraceD1> for Pte3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi1Sout> for Pte3 {
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

impl AltFn<super::sig::Spi1Pcs0> for Pte4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3Tx> for Pte4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0D3> for Pte4 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TraceD0> for Pte4 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Spi1Pcs2> for Pte5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3Rx> for Pte5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0D2> for Pte5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch0> for Pte5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Pte6> for Pte6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Pcs3> for Pte6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3CtsB> for Pte6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Mclk> for Pte6 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch1> for Pte6 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::UsbSofOut> for Pte6 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Uart3RtsB> for Pte7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Rxd0> for Pte7 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch2> for Pte7 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Pte8> for Pte8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2s0Rxd1> for Pte8 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5Tx> for Pte8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxFs> for Pte8 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch3> for Pte8 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::I2s0Txd1> for Pte9 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5Rx> for Pte9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxBclk> for Pte9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch4> for Pte9 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Uart5CtsB> for Pte10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Txd0> for Pte10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch5> for Pte10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Uart5RtsB> for Pte11 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxFs> for Pte11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch6> for Pte11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::I2s0TxBclk> for Pte12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch7> for Pte12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE24: Pte24 = Pte24 {}; 
pub const PTE24_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 24 };
pub const PTE24_IMPL_REF: &PinImpl = &PTE24_IMPL;

impl ::core::ops::Deref for Pte24 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE24_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte24 {}

impl Pin<Porte> for Pte24 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 24 }
}

impl AltFn<super::sig::Adc0Se17> for Pte24 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte24> for Pte24 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4Tx> for Pte24 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c0Scl> for Pte24 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmOutB> for Pte24 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE25: Pte25 = Pte25 {}; 
pub const PTE25_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 25 };
pub const PTE25_IMPL_REF: &PinImpl = &PTE25_IMPL;

impl ::core::ops::Deref for Pte25 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE25_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte25 {}

impl Pin<Porte> for Pte25 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 25 }
}

impl AltFn<super::sig::Adc0Se18> for Pte25 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte25> for Pte25 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4Rx> for Pte25 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c0Sda> for Pte25 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmIn> for Pte25 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE26: Pte26 = Pte26 {}; 
pub const PTE26_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 26 };
pub const PTE26_IMPL_REF: &PinImpl = &PTE26_IMPL;

impl ::core::ops::Deref for Pte26 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE26_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte26 {}

impl Pin<Porte> for Pte26 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 26 }
}

impl AltFn<super::sig::Pte26> for Pte26 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Enet1588Clkin> for Pte26 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart4CtsB> for Pte26 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::RtcClkout> for Pte26 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::UsbClkin> for Pte26 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE27: Pte27 = Pte27 {}; 
pub const PTE27_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 27 };
pub const PTE27_IMPL_REF: &PinImpl = &PTE27_IMPL;

impl ::core::ops::Deref for Pte27 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE27_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte27 {}

impl Pin<Porte> for Pte27 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 27 }
}

impl AltFn<super::sig::Pte27> for Pte27 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4RtsB> for Pte27 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTE28: Pte28 = Pte28 {}; 
pub const PTE28_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 28 };
pub const PTE28_IMPL_REF: &PinImpl = &PTE28_IMPL;

impl ::core::ops::Deref for Pte28 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE28_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte28 {}

impl Pin<Porte> for Pte28 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 28 }
}

impl AltFn<super::sig::Pte28> for Pte28 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

