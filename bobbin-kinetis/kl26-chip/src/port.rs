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

impl AltFn<super::sig::Tsi0Ch1> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta0> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch5> for Pta0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
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

impl AltFn<super::sig::Tsi0Ch2> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta1> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0Rx> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm2Ch0> for Pta1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
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

impl AltFn<super::sig::Tsi0Ch3> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta2> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0Tx> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm2Ch1> for Pta2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
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

impl AltFn<super::sig::Tsi0Ch4> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta3> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Scl> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch0> for Pta3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
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

impl AltFn<super::sig::Tsi0Ch5> for Pta4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta4> for Pta4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Sda> for Pta4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch1> for Pta4 {
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

impl AltFn<super::sig::Tpm0Ch2> for Pta5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxBclk> for Pta5 {
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

impl AltFn<super::sig::Pta12> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm1Ch0> for Pta12 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Txd0> for Pta12 {
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

impl AltFn<super::sig::Tpm1Ch1> for Pta13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxFs> for Pta13 {
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

impl AltFn<super::sig::Uart1Rx> for Pta18 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin0> for Pta18 {
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

impl AltFn<super::sig::Uart1Tx> for Pta19 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin1> for Pta19 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lptmr0Alt1> for Pta19 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA20: Pta20 = Pta20 {}; 
pub const PTA20_IMPL: PinImpl = PinImpl { port: PORTA_IMPL, index: 20 };
pub const PTA20_IMPL_REF: &PinImpl = &PTA20_IMPL;

impl ::core::ops::Deref for Pta20 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTA20_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pta20 {}

impl Pin<Porta> for Pta20 {
   #[inline]
   fn port(&self) -> Porta { PORTA }
   #[inline]
   fn index(&self) -> usize { 20 }
}

impl AltFn<super::sig::Pta20> for Pta20 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::ResetB> for Pta20 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Tsi0Ch0> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb0> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Scl> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm1Ch0> for Ptb0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
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

impl AltFn<super::sig::Tsi0Ch6> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb1> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Sda> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm1Ch1> for Ptb1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
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

impl AltFn<super::sig::Tsi0Ch7> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb2> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Scl> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm2Ch0> for Ptb2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
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

impl AltFn<super::sig::Tsi0Ch8> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb3> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Sda> for Ptb3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm2Ch1> for Ptb3 {
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

impl AltFn<super::sig::Tsi0Ch9> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb16> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Mosi> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Rx> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin0> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi1Miso> for Ptb16 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Tsi0Ch10> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb17> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Miso> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Tx> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin1> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi1Mosi> for Ptb17 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Tsi0Ch11> for Ptb18 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb18> for Ptb18 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm2Ch0> for Ptb18 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxBclk> for Ptb18 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Tsi0Ch12> for Ptb19 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb19> for Ptb19 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm2Ch1> for Ptb19 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxFs> for Ptb19 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Tsi0Ch13> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc0> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::ExtrgIn> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::AudiousbSofOut> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp0Out> for Ptc0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0Txd0> for Ptc0 {
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

impl AltFn<super::sig::Tsi0Ch14> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc1> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Scl> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch0> for Ptc1 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Adc0Se11> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tsi0Ch15> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc2> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Sda> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch1> for Ptc2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Ptc3> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart1Rx> for Ptc3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tpm0Ch2> for Ptc3 {
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

impl AltFn<super::sig::Tpm0Ch3> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s0Mclk> for Ptc4 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Cmp0Out> for Ptc5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Spi0Mosi> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::ExtrgIn> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxBclk> for Ptc6 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi0Miso> for Ptc6 {
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

impl AltFn<super::sig::Spi0Miso> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::AudiousbSofOut> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxFs> for Ptc7 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi0Mosi> for Ptc7 {
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

impl AltFn<super::sig::Cmp0In2> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc8> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Scl> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch4> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Mclk> for Ptc8 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Cmp0In3> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc9> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Sda> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch5> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxBclk> for Ptc9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::I2c1Scl> for Ptc10 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2s0RxFs> for Ptc10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::I2c1Sda> for Ptc11 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2s0Rxd0> for Ptc11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Tpm0Ch0> for Ptd0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Tpm0Ch1> for Ptd1 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Spi0Mosi> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Rx> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tpm0Ch2> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi0Miso> for Ptd2 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Spi0Miso> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Tx> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tpm0Ch3> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi0Mosi> for Ptd3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Spi1Pcs0> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Rx> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tpm0Ch4> for Ptd4 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Spi1Sck> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Tx> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tpm0Ch5> for Ptd5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Spi1Mosi> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Rx> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Miso> for Ptd6 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Spi1Miso> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Tx> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Mosi> for Ptd7 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Spi1Miso> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Tx> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::RtcClkout> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp0Out> for Pte0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c1Sda> for Pte0 {
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

impl AltFn<super::sig::Spi1Mosi> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Rx> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Miso> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c1Scl> for Pte1 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE20: Pte20 = Pte20 {}; 
pub const PTE20_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 20 };
pub const PTE20_IMPL_REF: &PinImpl = &PTE20_IMPL;

impl ::core::ops::Deref for Pte20 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE20_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte20 {}

impl Pin<Porte> for Pte20 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 20 }
}

impl AltFn<super::sig::Adc0Dp0> for Pte20 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se0> for Pte20 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte20> for Pte20 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm1Ch0> for Pte20 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart0Tx> for Pte20 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE21: Pte21 = Pte21 {}; 
pub const PTE21_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 21 };
pub const PTE21_IMPL_REF: &PinImpl = &PTE21_IMPL;

impl ::core::ops::Deref for Pte21 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE21_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte21 {}

impl Pin<Porte> for Pte21 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 21 }
}

impl AltFn<super::sig::Adc0Dm0> for Pte21 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se4a> for Pte21 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte21> for Pte21 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm1Ch1> for Pte21 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart0Rx> for Pte21 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE22: Pte22 = Pte22 {}; 
pub const PTE22_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 22 };
pub const PTE22_IMPL_REF: &PinImpl = &PTE22_IMPL;

impl ::core::ops::Deref for Pte22 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE22_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte22 {}

impl Pin<Porte> for Pte22 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 22 }
}

impl AltFn<super::sig::Adc0Dp3> for Pte22 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se3> for Pte22 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte22> for Pte22 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm2Ch0> for Pte22 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart2Tx> for Pte22 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE23: Pte23 = Pte23 {}; 
pub const PTE23_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 23 };
pub const PTE23_IMPL_REF: &PinImpl = &PTE23_IMPL;

impl ::core::ops::Deref for Pte23 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE23_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte23 {}

impl Pin<Porte> for Pte23 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 23 }
}

impl AltFn<super::sig::Adc0Dm3> for Pte23 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se7a> for Pte23 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte23> for Pte23 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm2Ch1> for Pte23 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart2Rx> for Pte23 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Pte24> for Pte24 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch0> for Pte24 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c0Scl> for Pte24 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Pte25> for Pte25 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch1> for Pte25 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c0Sda> for Pte25 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTE29: Pte29 = Pte29 {}; 
pub const PTE29_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 29 };
pub const PTE29_IMPL_REF: &PinImpl = &PTE29_IMPL;

impl ::core::ops::Deref for Pte29 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE29_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte29 {}

impl Pin<Porte> for Pte29 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 29 }
}

impl AltFn<super::sig::Cmp0In5> for Pte29 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se4b> for Pte29 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte29> for Pte29 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch2> for Pte29 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin0> for Pte29 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE30: Pte30 = Pte30 {}; 
pub const PTE30_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 30 };
pub const PTE30_IMPL_REF: &PinImpl = &PTE30_IMPL;

impl ::core::ops::Deref for Pte30 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE30_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte30 {}

impl Pin<Porte> for Pte30 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 30 }
}

impl AltFn<super::sig::Dac0Out> for Pte30 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se23> for Pte30 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In4> for Pte30 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte30> for Pte30 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch3> for Pte30 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin1> for Pte30 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE31: Pte31 = Pte31 {}; 
pub const PTE31_IMPL: PinImpl = PinImpl { port: PORTE_IMPL, index: 31 };
pub const PTE31_IMPL_REF: &PinImpl = &PTE31_IMPL;

impl ::core::ops::Deref for Pte31 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PTE31_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pte31 {}

impl Pin<Porte> for Pte31 {
   #[inline]
   fn port(&self) -> Porte { PORTE }
   #[inline]
   fn index(&self) -> usize { 31 }
}

impl AltFn<super::sig::Pte31> for Pte31 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch4> for Pte31 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

