pub use stm32_common::chip::gpio::*;

pub const GPIOA: Gpioa = Gpioa {};
pub const GPIOA_IMPL: GpioImpl = GpioImpl(0x48000000);
pub const GPIOA_IMPL_REF: &GpioImpl = &GPIOA_IMPL;

pub struct Gpioa {}
impl ::core::ops::Deref for Gpioa {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOA_IMPL_REF }
}


pub const GPIOB: Gpiob = Gpiob {};
pub const GPIOB_IMPL: GpioImpl = GpioImpl(0x48000400);
pub const GPIOB_IMPL_REF: &GpioImpl = &GPIOB_IMPL;

pub struct Gpiob {}
impl ::core::ops::Deref for Gpiob {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOB_IMPL_REF }
}


pub const GPIOC: Gpioc = Gpioc {};
pub const GPIOC_IMPL: GpioImpl = GpioImpl(0x48000800);
pub const GPIOC_IMPL_REF: &GpioImpl = &GPIOC_IMPL;

pub struct Gpioc {}
impl ::core::ops::Deref for Gpioc {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOC_IMPL_REF }
}


pub const GPIOD: Gpiod = Gpiod {};
pub const GPIOD_IMPL: GpioImpl = GpioImpl(0x48000c00);
pub const GPIOD_IMPL_REF: &GpioImpl = &GPIOD_IMPL;

pub struct Gpiod {}
impl ::core::ops::Deref for Gpiod {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOD_IMPL_REF }
}


pub const GPIOF: Gpiof = Gpiof {};
pub const GPIOF_IMPL: GpioImpl = GpioImpl(0x48001400);
pub const GPIOF_IMPL_REF: &GpioImpl = &GPIOF_IMPL;

pub struct Gpiof {}
impl ::core::ops::Deref for Gpiof {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOF_IMPL_REF }
}



pub const PA0: Pa0 = Pa0 {}; 
pub const PA0_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 0 };
pub const PA0_IMPL_REF: &PinImpl = &PA0_IMPL;

impl ::core::ops::Deref for Pa0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa0 {}

impl Pin<Gpioa> for Pa0 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch1> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Etr> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG1Io1> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Cts> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA1: Pa1 = Pa1 {}; 
pub const PA1_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 1 };
pub const PA1_IMPL_REF: &PinImpl = &PA1_IMPL;

impl ::core::ops::Deref for Pa1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa1 {}

impl Pin<Gpioa> for Pa1 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::RtcRefin> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch2> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG1Io2> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2RtsDe> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim15Ch1n> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA2: Pa2 = Pa2 {}; 
pub const PA2_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 2 };
pub const PA2_IMPL_REF: &PinImpl = &PA2_IMPL;

impl ::core::ops::Deref for Pa2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa2 {}

impl Pin<Gpioa> for Pa2 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim2Ch3> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG1Io3> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Tx> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Comp2Out> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim15Ch1> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA3: Pa3 = Pa3 {}; 
pub const PA3_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 3 };
pub const PA3_IMPL_REF: &PinImpl = &PA3_IMPL;

impl ::core::ops::Deref for Pa3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa3 {}

impl Pin<Gpioa> for Pa3 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim2Ch4> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG1Io4> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Rx> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim15Ch2> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA4: Pa4 = Pa4 {}; 
pub const PA4_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 4 };
pub const PA4_IMPL_REF: &PinImpl = &PA4_IMPL;

impl ::core::ops::Deref for Pa4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa4 {}

impl Pin<Gpioa> for Pa4 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::TscG2Io1> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi3Nss> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ws> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Ck> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA5: Pa5 = Pa5 {}; 
pub const PA5_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 5 };
pub const PA5_IMPL_REF: &PinImpl = &PA5_IMPL;

impl ::core::ops::Deref for Pa5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa5 {}

impl Pin<Gpioa> for Pa5 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim2Ch1> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Etr> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG2Io2> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Eventout> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA6: Pa6 = Pa6 {}; 
pub const PA6_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 6 };
pub const PA6_IMPL_REF: &PinImpl = &PA6_IMPL;

impl ::core::ops::Deref for Pa6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa6 {}

impl Pin<Gpioa> for Pa6 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Tim16Ch1> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG2Io3> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim1Bkin> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Eventout> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA7: Pa7 = Pa7 {}; 
pub const PA7_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 7 };
pub const PA7_IMPL_REF: &PinImpl = &PA7_IMPL;

impl ::core::ops::Deref for Pa7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa7 {}

impl Pin<Gpioa> for Pa7 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim17Ch1> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG2Io4> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim1Ch1n> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Eventout> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA8: Pa8 = Pa8 {}; 
pub const PA8_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 8 };
pub const PA8_IMPL_REF: &PinImpl = &PA8_IMPL;

impl ::core::ops::Deref for Pa8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa8 {}

impl Pin<Gpioa> for Pa8 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Mco> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c3Scl> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c2Smbal> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s2Mck> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch1> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1Ck> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA9: Pa9 = Pa9 {}; 
pub const PA9_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 9 };
pub const PA9_IMPL_REF: &PinImpl = &PA9_IMPL;

impl ::core::ops::Deref for Pa9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa9 {}

impl Pin<Gpioa> for Pa9 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::I2c3Smbal> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG4Io1> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c2Scl> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s3Mck> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch2> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1Tx> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim15Bkin> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim2Ch3> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA10: Pa10 = Pa10 {}; 
pub const PA10_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 10 };
pub const PA10_IMPL_REF: &PinImpl = &PA10_IMPL;

impl ::core::ops::Deref for Pa10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa10 {}

impl Pin<Gpioa> for Pa10 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Tim17Bkin> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG4Io2> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c2Sda> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Miso> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2extSd> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch3> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1Rx> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Comp6Out> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim2Ch4> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA11: Pa11 = Pa11 {}; 
pub const PA11_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 11 };
pub const PA11_IMPL_REF: &PinImpl = &PA11_IMPL;

impl ::core::ops::Deref for Pa11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa11 {}

impl Pin<Gpioa> for Pa11 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Spi2Mosi> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Sd> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch1n> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1Cts> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::CanRx> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim1Ch4> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Tim1Bkin2> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA12: Pa12 = Pa12 {}; 
pub const PA12_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 12 };
pub const PA12_IMPL_REF: &PinImpl = &PA12_IMPL;

impl ::core::ops::Deref for Pa12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa12 {}

impl Pin<Gpioa> for Pa12 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Tim16Ch1> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2sckin> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch2n> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1RtsDe> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Comp2Out> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::CanTx> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim1Etr> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA13: Pa13 = Pa13 {}; 
pub const PA13_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 13 };
pub const PA13_IMPL_REF: &PinImpl = &PA13_IMPL;

impl ::core::ops::Deref for Pa13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa13 {}

impl Pin<Gpioa> for Pa13 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Swdat> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Jtms> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim16Ch1n> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG4Io3> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::IrOut> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart3Cts> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA14: Pa14 = Pa14 {}; 
pub const PA14_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 14 };
pub const PA14_IMPL_REF: &PinImpl = &PA14_IMPL;

impl ::core::ops::Deref for Pa14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa14 {}

impl Pin<Gpioa> for Pa14 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Swclk> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Jtck> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::TscG4Io4> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Sda> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim1Bkin> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Tx> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA15: Pa15 = Pa15 {}; 
pub const PA15_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 15 };
pub const PA15_IMPL_REF: &PinImpl = &PA15_IMPL;

impl ::core::ops::Deref for Pa15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa15 {}

impl Pin<Gpioa> for Pa15 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Jtdi> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch1> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Etr> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscSync> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Scl> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi3Nss> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ws> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Rx> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim1Bkin> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB0: Pb0 = Pb0 {}; 
pub const PB0_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 0 };
pub const PB0_IMPL_REF: &PinImpl = &PB0_IMPL;

impl ::core::ops::Deref for Pb0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb0 {}

impl Pin<Gpiob> for Pb0 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::TscG3Io2> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim1Ch2n> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Eventout> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB1: Pb1 = Pb1 {}; 
pub const PB1_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 1 };
pub const PB1_IMPL_REF: &PinImpl = &PB1_IMPL;

impl ::core::ops::Deref for Pb1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb1 {}

impl Pin<Gpiob> for Pb1 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG3Io3> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim1Ch3n> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Comp4Out> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Eventout> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB2: Pb2 = Pb2 {}; 
pub const PB2_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 2 };
pub const PB2_IMPL_REF: &PinImpl = &PB2_IMPL;

impl ::core::ops::Deref for Pb2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb2 {}

impl Pin<Gpiob> for Pb2 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG3Io4> for Pb2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Eventout> for Pb2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB3: Pb3 = Pb3 {}; 
pub const PB3_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 3 };
pub const PB3_IMPL_REF: &PinImpl = &PB3_IMPL;

impl ::core::ops::Deref for Pb3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb3 {}

impl Pin<Gpiob> for Pb3 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Jtdo> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Traceswo> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch2> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG5Io1> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi3Sck> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ck> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Tx> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB4: Pb4 = Pb4 {}; 
pub const PB4_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 4 };
pub const PB4_IMPL_REF: &PinImpl = &PB4_IMPL;

impl ::core::ops::Deref for Pb4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb4 {}

impl Pin<Gpiob> for Pb4 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Jtrst> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim16Ch1> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG5Io2> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi3Miso> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Sd> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Rx> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim17Bkin> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB5: Pb5 = Pb5 {}; 
pub const PB5_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 5 };
pub const PB5_IMPL_REF: &PinImpl = &PB5_IMPL;

impl ::core::ops::Deref for Pb5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb5 {}

impl Pin<Gpiob> for Pb5 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim16Bkin> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Smbal> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi3Mosi> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3extSd> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Ck> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::I2c3Sda> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim17Ch1> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB6: Pb6 = Pb6 {}; 
pub const PB6_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 6 };
pub const PB6_IMPL_REF: &PinImpl = &PB6_IMPL;

impl ::core::ops::Deref for Pb6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb6 {}

impl Pin<Gpiob> for Pb6 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Tim16Ch1n> for Pb6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG5Io3> for Pb6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Scl> for Pb6 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Tx> for Pb6 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB7: Pb7 = Pb7 {}; 
pub const PB7_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 7 };
pub const PB7_IMPL_REF: &PinImpl = &PB7_IMPL;

impl ::core::ops::Deref for Pb7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb7 {}

impl Pin<Gpiob> for Pb7 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim17Ch1n> for Pb7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG5Io4> for Pb7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Sda> for Pb7 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Rx> for Pb7 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB8: Pb8 = Pb8 {}; 
pub const PB8_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 8 };
pub const PB8_IMPL_REF: &PinImpl = &PB8_IMPL;

impl ::core::ops::Deref for Pb8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb8 {}

impl Pin<Gpiob> for Pb8 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim16Ch1> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscSync> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Scl> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart3Rx> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::CanRx> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim1Bkin> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB9: Pb9 = Pb9 {}; 
pub const PB9_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 9 };
pub const PB9_IMPL_REF: &PinImpl = &PB9_IMPL;

impl ::core::ops::Deref for Pb9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb9 {}

impl Pin<Gpiob> for Pb9 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim17Ch1> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Sda> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::IrOut> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Tx> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Comp2Out> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::CanTx> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB10: Pb10 = Pb10 {}; 
pub const PB10_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 10 };
pub const PB10_IMPL_REF: &PinImpl = &PB10_IMPL;

impl ::core::ops::Deref for Pb10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb10 {}

impl Pin<Gpiob> for Pb10 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Tim2Ch3> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscSync> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart3Tx> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB11: Pb11 = Pb11 {}; 
pub const PB11_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 11 };
pub const PB11_IMPL_REF: &PinImpl = &PB11_IMPL;

impl ::core::ops::Deref for Pb11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb11 {}

impl Pin<Gpiob> for Pb11 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Tim2Ch4> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG6Io1> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart3Rx> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB12: Pb12 = Pb12 {}; 
pub const PB12_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 12 };
pub const PB12_IMPL_REF: &PinImpl = &PB12_IMPL;

impl ::core::ops::Deref for Pb12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb12 {}

impl Pin<Gpiob> for Pb12 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::TscG6Io2> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c2Smbal> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Nss> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ws> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Bkin> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Ck> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB13: Pb13 = Pb13 {}; 
pub const PB13_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 13 };
pub const PB13_IMPL_REF: &PinImpl = &PB13_IMPL;

impl ::core::ops::Deref for Pb13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb13 {}

impl Pin<Gpiob> for Pb13 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::TscG6Io3> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Sck> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch1n> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Cts> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB14: Pb14 = Pb14 {}; 
pub const PB14_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 14 };
pub const PB14_IMPL_REF: &PinImpl = &PB14_IMPL;

impl ::core::ops::Deref for Pb14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb14 {}

impl Pin<Gpiob> for Pb14 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Tim15Ch1> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG6Io4> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Miso> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2extSd> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch2n> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3RtsDe> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB15: Pb15 = Pb15 {}; 
pub const PB15_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 15 };
pub const PB15_IMPL_REF: &PinImpl = &PB15_IMPL;

impl ::core::ops::Deref for Pb15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb15 {}

impl Pin<Gpiob> for Pb15 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::RtcRefin> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim15Ch2> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim15Ch1n> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim1Ch3n> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Mosi> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Sd> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Eventout> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC0: Pc0 = Pc0 {}; 
pub const PC0_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 0 };
pub const PC0_IMPL_REF: &PinImpl = &PC0_IMPL;

impl ::core::ops::Deref for Pc0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc0 {}

impl Pin<Gpioc> for Pc0 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Eventout> for Pc0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch1> for Pc0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PC1: Pc1 = Pc1 {}; 
pub const PC1_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 1 };
pub const PC1_IMPL_REF: &PinImpl = &PC1_IMPL;

impl ::core::ops::Deref for Pc1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc1 {}

impl Pin<Gpioc> for Pc1 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Eventout> for Pc1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch2> for Pc1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PC2: Pc2 = Pc2 {}; 
pub const PC2_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 2 };
pub const PC2_IMPL_REF: &PinImpl = &PC2_IMPL;

impl ::core::ops::Deref for Pc2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc2 {}

impl Pin<Gpioc> for Pc2 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Eventout> for Pc2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch3> for Pc2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PC3: Pc3 = Pc3 {}; 
pub const PC3_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 3 };
pub const PC3_IMPL_REF: &PinImpl = &PC3_IMPL;

impl ::core::ops::Deref for Pc3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc3 {}

impl Pin<Gpioc> for Pc3 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Eventout> for Pc3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch4> for Pc3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim1Bkin2> for Pc3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PC4: Pc4 = Pc4 {}; 
pub const PC4_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 4 };
pub const PC4_IMPL_REF: &PinImpl = &PC4_IMPL;

impl ::core::ops::Deref for Pc4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc4 {}

impl Pin<Gpioc> for Pc4 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Eventout> for Pc4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Etr> for Pc4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart1Tx> for Pc4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC5: Pc5 = Pc5 {}; 
pub const PC5_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 5 };
pub const PC5_IMPL_REF: &PinImpl = &PC5_IMPL;

impl ::core::ops::Deref for Pc5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc5 {}

impl Pin<Gpioc> for Pc5 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Eventout> for Pc5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim15Bkin> for Pc5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG3Io1> for Pc5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart1Rx> for Pc5 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC6: Pc6 = Pc6 {}; 
pub const PC6_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 6 };
pub const PC6_IMPL_REF: &PinImpl = &PC6_IMPL;

impl ::core::ops::Deref for Pc6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc6 {}

impl Pin<Gpioc> for Pc6 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Eventout> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2s2Mck> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Comp6Out> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC7: Pc7 = Pc7 {}; 
pub const PC7_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 7 };
pub const PC7_IMPL_REF: &PinImpl = &PC7_IMPL;

impl ::core::ops::Deref for Pc7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc7 {}

impl Pin<Gpioc> for Pc7 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2s3Mck> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PC8: Pc8 = Pc8 {}; 
pub const PC8_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 8 };
pub const PC8_IMPL_REF: &PinImpl = &PC8_IMPL;

impl ::core::ops::Deref for Pc8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc8 {}

impl Pin<Gpioc> for Pc8 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Eventout> for Pc8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PC9: Pc9 = Pc9 {}; 
pub const PC9_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 9 };
pub const PC9_IMPL_REF: &PinImpl = &PC9_IMPL;

impl ::core::ops::Deref for Pc9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc9 {}

impl Pin<Gpioc> for Pc9 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Pc9 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c3Sda> for Pc9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2sckin> for Pc9 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PC10: Pc10 = Pc10 {}; 
pub const PC10_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 10 };
pub const PC10_IMPL_REF: &PinImpl = &PC10_IMPL;

impl ::core::ops::Deref for Pc10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc10 {}

impl Pin<Gpioc> for Pc10 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pc10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi3Sck> for Pc10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ck> for Pc10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Tx> for Pc10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC11: Pc11 = Pc11 {}; 
pub const PC11_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 11 };
pub const PC11_IMPL_REF: &PinImpl = &PC11_IMPL;

impl ::core::ops::Deref for Pc11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc11 {}

impl Pin<Gpioc> for Pc11 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pc11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi3Miso> for Pc11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3extSd> for Pc11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Rx> for Pc11 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC12: Pc12 = Pc12 {}; 
pub const PC12_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 12 };
pub const PC12_IMPL_REF: &PinImpl = &PC12_IMPL;

impl ::core::ops::Deref for Pc12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc12 {}

impl Pin<Gpioc> for Pc12 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pc12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi3Mosi> for Pc12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Sd> for Pc12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Ck> for Pc12 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC13: Pc13 = Pc13 {}; 
pub const PC13_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 13 };
pub const PC13_IMPL_REF: &PinImpl = &PC13_IMPL;

impl ::core::ops::Deref for Pc13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc13 {}

impl Pin<Gpioc> for Pc13 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Tim1Ch1n> for Pc13 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PC14: Pc14 = Pc14 {}; 
pub const PC14_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 14 };
pub const PC14_IMPL_REF: &PinImpl = &PC14_IMPL;

impl ::core::ops::Deref for Pc14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc14 {}

impl Pin<Gpioc> for Pc14 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 14 }
}

pub const PC15: Pc15 = Pc15 {}; 
pub const PC15_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 15 };
pub const PC15_IMPL_REF: &PinImpl = &PC15_IMPL;

impl ::core::ops::Deref for Pc15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc15 {}

impl Pin<Gpioc> for Pc15 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 15 }
}

pub const PD2: Pd2 = Pd2 {}; 
pub const PD2_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 2 };
pub const PD2_IMPL_REF: &PinImpl = &PD2_IMPL;

impl ::core::ops::Deref for Pd2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd2 {}

impl Pin<Gpiod> for Pd2 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Eventout> for Pd2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PF0: Pf0 = Pf0 {}; 
pub const PF0_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 0 };
pub const PF0_IMPL_REF: &PinImpl = &PF0_IMPL;

impl ::core::ops::Deref for Pf0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf0 {}

impl Pin<Gpiof> for Pf0 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c2Sda> for Pf0 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Nss> for Pf0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ws> for Pf0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch3n> for Pf0 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PF1: Pf1 = Pf1 {}; 
pub const PF1_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 1 };
pub const PF1_IMPL_REF: &PinImpl = &PF1_IMPL;

impl ::core::ops::Deref for Pf1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf1 {}

impl Pin<Gpiof> for Pf1 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c2Scl> for Pf1 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Sck> for Pf1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pf1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

