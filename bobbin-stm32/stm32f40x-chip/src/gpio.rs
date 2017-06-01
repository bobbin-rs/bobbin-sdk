pub use stm32_common::chip::gpio::*;

pub const GPIOA: Gpioa = Gpioa {};
pub const GPIOA_IMPL: GpioImpl = GpioImpl(0x40020000);
pub const GPIOA_IMPL_REF: &GpioImpl = &GPIOA_IMPL;

pub struct Gpioa {}
impl ::core::ops::Deref for Gpioa {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOA_IMPL_REF }
}


pub const GPIOB: Gpiob = Gpiob {};
pub const GPIOB_IMPL: GpioImpl = GpioImpl(0x40020400);
pub const GPIOB_IMPL_REF: &GpioImpl = &GPIOB_IMPL;

pub struct Gpiob {}
impl ::core::ops::Deref for Gpiob {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOB_IMPL_REF }
}


pub const GPIOC: Gpioc = Gpioc {};
pub const GPIOC_IMPL: GpioImpl = GpioImpl(0x40020800);
pub const GPIOC_IMPL_REF: &GpioImpl = &GPIOC_IMPL;

pub struct Gpioc {}
impl ::core::ops::Deref for Gpioc {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOC_IMPL_REF }
}


pub const GPIOD: Gpiod = Gpiod {};
pub const GPIOD_IMPL: GpioImpl = GpioImpl(0x40020c00);
pub const GPIOD_IMPL_REF: &GpioImpl = &GPIOD_IMPL;

pub struct Gpiod {}
impl ::core::ops::Deref for Gpiod {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOD_IMPL_REF }
}


pub const GPIOE: Gpioe = Gpioe {};
pub const GPIOE_IMPL: GpioImpl = GpioImpl(0x40021000);
pub const GPIOE_IMPL_REF: &GpioImpl = &GPIOE_IMPL;

pub struct Gpioe {}
impl ::core::ops::Deref for Gpioe {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOE_IMPL_REF }
}


pub const GPIOF: Gpiof = Gpiof {};
pub const GPIOF_IMPL: GpioImpl = GpioImpl(0x40021400);
pub const GPIOF_IMPL_REF: &GpioImpl = &GPIOF_IMPL;

pub struct Gpiof {}
impl ::core::ops::Deref for Gpiof {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOF_IMPL_REF }
}


pub const GPIOG: Gpiog = Gpiog {};
pub const GPIOG_IMPL: GpioImpl = GpioImpl(0x40021800);
pub const GPIOG_IMPL_REF: &GpioImpl = &GPIOG_IMPL;

pub struct Gpiog {}
impl ::core::ops::Deref for Gpiog {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOG_IMPL_REF }
}


pub const GPIOH: Gpioh = Gpioh {};
pub const GPIOH_IMPL: GpioImpl = GpioImpl(0x40021c00);
pub const GPIOH_IMPL_REF: &GpioImpl = &GPIOH_IMPL;

pub struct Gpioh {}
impl ::core::ops::Deref for Gpioh {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOH_IMPL_REF }
}


pub const GPIOI: Gpioi = Gpioi {};
pub const GPIOI_IMPL: GpioImpl = GpioImpl(0x40022000);
pub const GPIOI_IMPL_REF: &GpioImpl = &GPIOI_IMPL;

pub struct Gpioi {}
impl ::core::ops::Deref for Gpioi {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOI_IMPL_REF }
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

impl AltFn<super::sig::Tim2Ch1Etr> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim5Ch1> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Etr> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Cts> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Uart4Tx> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EthMiiCrs> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
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

impl AltFn<super::sig::Tim2Ch2> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim5Ch2> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart2Rts> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Uart4Rx> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EthMiiRxClk> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiRefClk> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
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

impl AltFn<super::sig::Tim5Ch3> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim9Ch1> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Tx> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::EthMdio> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
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

impl AltFn<super::sig::Tim5Ch4> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim9Ch2> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Rx> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::OtgHsUlpiD0> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiCol> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
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

impl AltFn<super::sig::Spi1Nss> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::OtgHsSof> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiHsync> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
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

impl AltFn<super::sig::Tim2Ch1Etr> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim8Ch1n> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Sck> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::OtgHsUlpiCk> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
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

impl AltFn<super::sig::Tim1Bkin> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch1> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Bkin> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Miso> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim13Ch1> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::DcmiPixck> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
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

impl AltFn<super::sig::Tim1Ch1n> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch2> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch1n> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Mosi> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim14Ch1> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::EthMiiRxDv> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiCrsDv> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
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

impl AltFn<super::sig::Mco1> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim1Ch1> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c3Scl> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Ck> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::OtgFsSof> for Pa8 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
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

impl AltFn<super::sig::Tim1Ch2> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c3Smba> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Tx> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::DcmiD0> for Pa9 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
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

impl AltFn<super::sig::Tim1Ch3> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart1Rx> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::OtgFsId> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::DcmiD1> for Pa10 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
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

impl AltFn<super::sig::Tim1Ch4> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart1Cts> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Can1Rx> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgFsDm> for Pa11 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
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

impl AltFn<super::sig::Tim1Etr> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart1Rts> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Can1Tx> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgFsDp> for Pa12 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
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

impl AltFn<super::sig::Jtms> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Swdio> for Pa13 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
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

impl AltFn<super::sig::Jtck> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Swclk> for Pa14 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
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

impl AltFn<super::sig::Spi1Nss> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi3Nss> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ws> for Pa15 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Tim1Ch2n> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch3> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch2n> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::OtgHsUlpiD1> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiRxd2> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
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

impl AltFn<super::sig::Tim1Ch3n> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch4> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch3n> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::OtgHsUlpiD2> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiRxd3> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
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

impl AltFn<super::sig::Spi1Sck> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi3Sck> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ck> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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

impl AltFn<super::sig::Njtrst> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim3Ch1> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Spi1Miso> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi3Miso> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3extSd> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Tim3Ch2> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c1Smba> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi1Mosi> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi3Mosi> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Sd> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Can2Rx> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsUlpiD7> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthPpsOut> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::DcmiD10> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
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

impl AltFn<super::sig::Tim4Ch1> for Pb6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c1Scl> for Pb6 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Tx> for Pb6 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Can2Tx> for Pb6 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::DcmiD5> for Pb6 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
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

impl AltFn<super::sig::Tim4Ch2> for Pb7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c1Sda> for Pb7 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Rx> for Pb7 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FsmcNl> for Pb7 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiVsync> for Pb7 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
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

impl AltFn<super::sig::Tim4Ch3> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim10Ch1> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Scl> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Can1Rx> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::EthMiiTxd3> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::SdioD4> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD6> for Pb8 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
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

impl AltFn<super::sig::Tim4Ch4> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim11Ch1> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Sda> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Nss> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ws> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Can1Tx> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::SdioD5> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD7> for Pb9 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
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

impl AltFn<super::sig::I2c2Scl> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Sck> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart3Tx> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::OtgHsUlpiD3> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiRxEr> for Pb10 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
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

impl AltFn<super::sig::I2c2Sda> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart3Rx> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::OtgHsUlpiD4> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiTxEn> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxEn> for Pb11 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
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

impl AltFn<super::sig::Tim1Bkin> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c2Smba> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Nss> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ws> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart3Ck> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Can2Rx> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsUlpiD5> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiTxd0> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxd0> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::OtgHsId> for Pb12 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
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

impl AltFn<super::sig::Tim1Ch1n> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sck> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart3Cts> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Can2Tx> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsUlpiD6> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiTxd1> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxd1> for Pb13 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
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

impl AltFn<super::sig::Tim1Ch2n> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim8Ch2n> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Miso> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2extSd> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Rts> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim12Ch1> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsDm> for Pb14 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
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

impl AltFn<super::sig::Tim1Ch3n> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim8Ch3n> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Mosi> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Sd> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim12Ch2> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsDp> for Pb15 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
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

impl AltFn<super::sig::OtgHsUlpiStp> for Pc0 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pc0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::EthMdc> for Pc1 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pc1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Spi2Miso> for Pc2 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2extSd> for Pc2 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::OtgHsUlpiDir> for Pc2 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiTxd2> for Pc2 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pc2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Spi2Mosi> for Pc3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Sd> for Pc3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::OtgHsUlpiNxt> for Pc3 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiTxClk> for Pc3 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pc3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::EthMiiRxd0> for Pc4 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiRxd0> for Pc4 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pc4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::EthMiiRxd1> for Pc5 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiRxd1> for Pc5 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pc5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Tim3Ch1> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch1> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s2Mck> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart6Tx> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::SdioD6> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD0> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Tim3Ch2> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch2> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s3Mck> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart6Rx> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::SdioD7> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD1> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Tim3Ch3> for Pc8 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch3> for Pc8 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart6Ck> for Pc8 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::SdioD0> for Pc8 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD2> for Pc8 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pc8 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Mco2> for Pc9 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim3Ch4> for Pc9 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch4> for Pc9 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c3Sda> for Pc9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2sCkin> for Pc9 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::SdioD1> for Pc9 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD3> for Pc9 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pc9 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Spi3Sck> for Pc10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ck> for Pc10 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Tx> for Pc10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Uart4Tx> for Pc10 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::SdioD2> for Pc10 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD8> for Pc10 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pc10 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::I2s3extSd> for Pc11 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi3Miso> for Pc11 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Rx> for Pc11 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Uart4Rx> for Pc11 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::SdioD3> for Pc11 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD4> for Pc11 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pc11 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Spi3Mosi> for Pc12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Sd> for Pc12 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Ck> for Pc12 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Uart5Tx> for Pc12 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::SdioCk> for Pc12 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD9> for Pc12 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pc12 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Eventout> for Pc13 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Eventout> for Pc14 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Eventout> for Pc15 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD0: Pd0 = Pd0 {}; 
pub const PD0_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 0 };
pub const PD0_IMPL_REF: &PinImpl = &PD0_IMPL;

impl ::core::ops::Deref for Pd0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd0 {}

impl Pin<Gpiod> for Pd0 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Can1Rx> for Pd0 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FsmcD2> for Pd0 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD1: Pd1 = Pd1 {}; 
pub const PD1_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 1 };
pub const PD1_IMPL_REF: &PinImpl = &PD1_IMPL;

impl ::core::ops::Deref for Pd1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd1 {}

impl Pin<Gpiod> for Pd1 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Can1Tx> for Pd1 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FsmcD3> for Pd1 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::Tim3Etr> for Pd2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5Rx> for Pd2 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::SdioCmd> for Pd2 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD11> for Pd2 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pd2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD3: Pd3 = Pd3 {}; 
pub const PD3_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 3 };
pub const PD3_IMPL_REF: &PinImpl = &PD3_IMPL;

impl ::core::ops::Deref for Pd3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd3 {}

impl Pin<Gpiod> for Pd3 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Cts> for Pd3 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FsmcClk> for Pd3 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD4: Pd4 = Pd4 {}; 
pub const PD4_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 4 };
pub const PD4_IMPL_REF: &PinImpl = &PD4_IMPL;

impl ::core::ops::Deref for Pd4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd4 {}

impl Pin<Gpiod> for Pd4 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart2Rts> for Pd4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FsmcNoe> for Pd4 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD5: Pd5 = Pd5 {}; 
pub const PD5_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 5 };
pub const PD5_IMPL_REF: &PinImpl = &PD5_IMPL;

impl ::core::ops::Deref for Pd5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd5 {}

impl Pin<Gpiod> for Pd5 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart2Tx> for Pd5 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FsmcNwe> for Pd5 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD6: Pd6 = Pd6 {}; 
pub const PD6_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 6 };
pub const PD6_IMPL_REF: &PinImpl = &PD6_IMPL;

impl ::core::ops::Deref for Pd6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd6 {}

impl Pin<Gpiod> for Pd6 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Rx> for Pd6 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FsmcNwait> for Pd6 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD7: Pd7 = Pd7 {}; 
pub const PD7_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 7 };
pub const PD7_IMPL_REF: &PinImpl = &PD7_IMPL;

impl ::core::ops::Deref for Pd7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd7 {}

impl Pin<Gpiod> for Pd7 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Usart2Ck> for Pd7 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FsmcNe1> for Pd7 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::FsmcNce2> for Pd7 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD8: Pd8 = Pd8 {}; 
pub const PD8_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 8 };
pub const PD8_IMPL_REF: &PinImpl = &PD8_IMPL;

impl ::core::ops::Deref for Pd8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd8 {}

impl Pin<Gpiod> for Pd8 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Usart3Tx> for Pd8 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FsmcD13> for Pd8 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd8 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD9: Pd9 = Pd9 {}; 
pub const PD9_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 9 };
pub const PD9_IMPL_REF: &PinImpl = &PD9_IMPL;

impl ::core::ops::Deref for Pd9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd9 {}

impl Pin<Gpiod> for Pd9 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Usart3Rx> for Pd9 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FsmcD14> for Pd9 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd9 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD10: Pd10 = Pd10 {}; 
pub const PD10_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 10 };
pub const PD10_IMPL_REF: &PinImpl = &PD10_IMPL;

impl ::core::ops::Deref for Pd10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd10 {}

impl Pin<Gpiod> for Pd10 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Usart3Ck> for Pd10 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FsmcD15> for Pd10 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd10 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD11: Pd11 = Pd11 {}; 
pub const PD11_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 11 };
pub const PD11_IMPL_REF: &PinImpl = &PD11_IMPL;

impl ::core::ops::Deref for Pd11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd11 {}

impl Pin<Gpiod> for Pd11 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Usart3Cts> for Pd11 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FsmcA16> for Pd11 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd11 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD12: Pd12 = Pd12 {}; 
pub const PD12_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 12 };
pub const PD12_IMPL_REF: &PinImpl = &PD12_IMPL;

impl ::core::ops::Deref for Pd12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd12 {}

impl Pin<Gpiod> for Pd12 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Tim4Ch1> for Pd12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart3Rts> for Pd12 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FsmcA17> for Pd12 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd12 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD13: Pd13 = Pd13 {}; 
pub const PD13_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 13 };
pub const PD13_IMPL_REF: &PinImpl = &PD13_IMPL;

impl ::core::ops::Deref for Pd13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd13 {}

impl Pin<Gpiod> for Pd13 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Tim4Ch2> for Pd13 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FsmcA18> for Pd13 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd13 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD14: Pd14 = Pd14 {}; 
pub const PD14_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 14 };
pub const PD14_IMPL_REF: &PinImpl = &PD14_IMPL;

impl ::core::ops::Deref for Pd14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd14 {}

impl Pin<Gpiod> for Pd14 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Tim4Ch3> for Pd14 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FsmcD0> for Pd14 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd14 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD15: Pd15 = Pd15 {}; 
pub const PD15_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 15 };
pub const PD15_IMPL_REF: &PinImpl = &PD15_IMPL;

impl ::core::ops::Deref for Pd15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd15 {}

impl Pin<Gpiod> for Pd15 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Tim4Ch4> for Pd15 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FsmcD1> for Pd15 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd15 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE0: Pe0 = Pe0 {}; 
pub const PE0_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 0 };
pub const PE0_IMPL_REF: &PinImpl = &PE0_IMPL;

impl ::core::ops::Deref for Pe0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe0 {}

impl Pin<Gpioe> for Pe0 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim4Etr> for Pe0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FsmcNbl0> for Pe0 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD2> for Pe0 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pe0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE1: Pe1 = Pe1 {}; 
pub const PE1_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 1 };
pub const PE1_IMPL_REF: &PinImpl = &PE1_IMPL;

impl ::core::ops::Deref for Pe1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe1 {}

impl Pin<Gpioe> for Pe1 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::FsmcNbl1> for Pe1 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD3> for Pe1 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pe1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE2: Pe2 = Pe2 {}; 
pub const PE2_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 2 };
pub const PE2_IMPL_REF: &PinImpl = &PE2_IMPL;

impl ::core::ops::Deref for Pe2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe2 {}

impl Pin<Gpioe> for Pe2 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Traceclk> for Pe2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::EthMiiTxd3> for Pe2 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FsmcA23> for Pe2 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE3: Pe3 = Pe3 {}; 
pub const PE3_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 3 };
pub const PE3_IMPL_REF: &PinImpl = &PE3_IMPL;

impl ::core::ops::Deref for Pe3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe3 {}

impl Pin<Gpioe> for Pe3 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Traced0> for Pe3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::FsmcA19> for Pe3 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE4: Pe4 = Pe4 {}; 
pub const PE4_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 4 };
pub const PE4_IMPL_REF: &PinImpl = &PE4_IMPL;

impl ::core::ops::Deref for Pe4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe4 {}

impl Pin<Gpioe> for Pe4 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Traced1> for Pe4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::FsmcA20> for Pe4 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD4> for Pe4 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pe4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE5: Pe5 = Pe5 {}; 
pub const PE5_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 5 };
pub const PE5_IMPL_REF: &PinImpl = &PE5_IMPL;

impl ::core::ops::Deref for Pe5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe5 {}

impl Pin<Gpioe> for Pe5 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Traced2> for Pe5 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim9Ch1> for Pe5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FsmcA21> for Pe5 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD6> for Pe5 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pe5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE6: Pe6 = Pe6 {}; 
pub const PE6_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 6 };
pub const PE6_IMPL_REF: &PinImpl = &PE6_IMPL;

impl ::core::ops::Deref for Pe6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe6 {}

impl Pin<Gpioe> for Pe6 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Traced3> for Pe6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim9Ch2> for Pe6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FsmcA22> for Pe6 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD7> for Pe6 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pe6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE7: Pe7 = Pe7 {}; 
pub const PE7_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 7 };
pub const PE7_IMPL_REF: &PinImpl = &PE7_IMPL;

impl ::core::ops::Deref for Pe7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe7 {}

impl Pin<Gpioe> for Pe7 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim1Etr> for Pe7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FsmcD4> for Pe7 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE8: Pe8 = Pe8 {}; 
pub const PE8_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 8 };
pub const PE8_IMPL_REF: &PinImpl = &PE8_IMPL;

impl ::core::ops::Deref for Pe8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe8 {}

impl Pin<Gpioe> for Pe8 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim1Ch1n> for Pe8 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FsmcD5> for Pe8 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe8 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE9: Pe9 = Pe9 {}; 
pub const PE9_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 9 };
pub const PE9_IMPL_REF: &PinImpl = &PE9_IMPL;

impl ::core::ops::Deref for Pe9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe9 {}

impl Pin<Gpioe> for Pe9 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim1Ch1> for Pe9 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FsmcD6> for Pe9 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe9 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE10: Pe10 = Pe10 {}; 
pub const PE10_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 10 };
pub const PE10_IMPL_REF: &PinImpl = &PE10_IMPL;

impl ::core::ops::Deref for Pe10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe10 {}

impl Pin<Gpioe> for Pe10 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Tim1Ch2n> for Pe10 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FsmcD7> for Pe10 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe10 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE11: Pe11 = Pe11 {}; 
pub const PE11_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 11 };
pub const PE11_IMPL_REF: &PinImpl = &PE11_IMPL;

impl ::core::ops::Deref for Pe11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe11 {}

impl Pin<Gpioe> for Pe11 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Tim1Ch2> for Pe11 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FsmcD8> for Pe11 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe11 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE12: Pe12 = Pe12 {}; 
pub const PE12_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 12 };
pub const PE12_IMPL_REF: &PinImpl = &PE12_IMPL;

impl ::core::ops::Deref for Pe12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe12 {}

impl Pin<Gpioe> for Pe12 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Tim1Ch3n> for Pe12 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FsmcD9> for Pe12 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe12 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE13: Pe13 = Pe13 {}; 
pub const PE13_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 13 };
pub const PE13_IMPL_REF: &PinImpl = &PE13_IMPL;

impl ::core::ops::Deref for Pe13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe13 {}

impl Pin<Gpioe> for Pe13 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Tim1Ch3> for Pe13 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FsmcD10> for Pe13 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe13 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE14: Pe14 = Pe14 {}; 
pub const PE14_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 14 };
pub const PE14_IMPL_REF: &PinImpl = &PE14_IMPL;

impl ::core::ops::Deref for Pe14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe14 {}

impl Pin<Gpioe> for Pe14 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Tim1Ch4> for Pe14 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FsmcD11> for Pe14 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe14 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE15: Pe15 = Pe15 {}; 
pub const PE15_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 15 };
pub const PE15_IMPL_REF: &PinImpl = &PE15_IMPL;

impl ::core::ops::Deref for Pe15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe15 {}

impl Pin<Gpioe> for Pe15 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Tim1Bkin> for Pe15 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FsmcD12> for Pe15 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe15 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::FsmcA0> for Pf0 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
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

impl AltFn<super::sig::FsmcA1> for Pf1 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF2: Pf2 = Pf2 {}; 
pub const PF2_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 2 };
pub const PF2_IMPL_REF: &PinImpl = &PF2_IMPL;

impl ::core::ops::Deref for Pf2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf2 {}

impl Pin<Gpiof> for Pf2 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c2Smba> for Pf2 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FsmcA2> for Pf2 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF3: Pf3 = Pf3 {}; 
pub const PF3_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 3 };
pub const PF3_IMPL_REF: &PinImpl = &PF3_IMPL;

impl ::core::ops::Deref for Pf3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf3 {}

impl Pin<Gpiof> for Pf3 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::FsmcA3> for Pf3 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF4: Pf4 = Pf4 {}; 
pub const PF4_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 4 };
pub const PF4_IMPL_REF: &PinImpl = &PF4_IMPL;

impl ::core::ops::Deref for Pf4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf4 {}

impl Pin<Gpiof> for Pf4 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::FsmcA4> for Pf4 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF5: Pf5 = Pf5 {}; 
pub const PF5_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 5 };
pub const PF5_IMPL_REF: &PinImpl = &PF5_IMPL;

impl ::core::ops::Deref for Pf5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf5 {}

impl Pin<Gpiof> for Pf5 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::FsmcA5> for Pf5 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF6: Pf6 = Pf6 {}; 
pub const PF6_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 6 };
pub const PF6_IMPL_REF: &PinImpl = &PF6_IMPL;

impl ::core::ops::Deref for Pf6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf6 {}

impl Pin<Gpiof> for Pf6 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Tim10Ch1> for Pf6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FsmcNiord> for Pf6 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF7: Pf7 = Pf7 {}; 
pub const PF7_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 7 };
pub const PF7_IMPL_REF: &PinImpl = &PF7_IMPL;

impl ::core::ops::Deref for Pf7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf7 {}

impl Pin<Gpiof> for Pf7 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim11Ch1> for Pf7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FsmcNreg> for Pf7 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF8: Pf8 = Pf8 {}; 
pub const PF8_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 8 };
pub const PF8_IMPL_REF: &PinImpl = &PF8_IMPL;

impl ::core::ops::Deref for Pf8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf8 {}

impl Pin<Gpiof> for Pf8 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim13Ch1> for Pf8 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FsmcNiowr> for Pf8 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf8 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF9: Pf9 = Pf9 {}; 
pub const PF9_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 9 };
pub const PF9_IMPL_REF: &PinImpl = &PF9_IMPL;

impl ::core::ops::Deref for Pf9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf9 {}

impl Pin<Gpiof> for Pf9 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim14Ch1> for Pf9 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FsmcCd> for Pf9 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf9 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF10: Pf10 = Pf10 {}; 
pub const PF10_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 10 };
pub const PF10_IMPL_REF: &PinImpl = &PF10_IMPL;

impl ::core::ops::Deref for Pf10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf10 {}

impl Pin<Gpiof> for Pf10 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::FsmcIntr> for Pf10 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf10 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF11: Pf11 = Pf11 {}; 
pub const PF11_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 11 };
pub const PF11_IMPL_REF: &PinImpl = &PF11_IMPL;

impl ::core::ops::Deref for Pf11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf11 {}

impl Pin<Gpiof> for Pf11 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::DcmiD12> for Pf11 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pf11 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF12: Pf12 = Pf12 {}; 
pub const PF12_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 12 };
pub const PF12_IMPL_REF: &PinImpl = &PF12_IMPL;

impl ::core::ops::Deref for Pf12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf12 {}

impl Pin<Gpiof> for Pf12 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::FsmcA6> for Pf12 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf12 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF13: Pf13 = Pf13 {}; 
pub const PF13_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 13 };
pub const PF13_IMPL_REF: &PinImpl = &PF13_IMPL;

impl ::core::ops::Deref for Pf13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf13 {}

impl Pin<Gpiof> for Pf13 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::FsmcA7> for Pf13 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf13 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF14: Pf14 = Pf14 {}; 
pub const PF14_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 14 };
pub const PF14_IMPL_REF: &PinImpl = &PF14_IMPL;

impl ::core::ops::Deref for Pf14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf14 {}

impl Pin<Gpiof> for Pf14 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::FsmcA8> for Pf14 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf14 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF15: Pf15 = Pf15 {}; 
pub const PF15_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 15 };
pub const PF15_IMPL_REF: &PinImpl = &PF15_IMPL;

impl ::core::ops::Deref for Pf15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf15 {}

impl Pin<Gpiof> for Pf15 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::FsmcA9> for Pf15 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf15 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG0: Pg0 = Pg0 {}; 
pub const PG0_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 0 };
pub const PG0_IMPL_REF: &PinImpl = &PG0_IMPL;

impl ::core::ops::Deref for Pg0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg0 {}

impl Pin<Gpiog> for Pg0 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::FsmcA10> for Pg0 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG1: Pg1 = Pg1 {}; 
pub const PG1_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 1 };
pub const PG1_IMPL_REF: &PinImpl = &PG1_IMPL;

impl ::core::ops::Deref for Pg1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg1 {}

impl Pin<Gpiog> for Pg1 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::FsmcA11> for Pg1 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG2: Pg2 = Pg2 {}; 
pub const PG2_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 2 };
pub const PG2_IMPL_REF: &PinImpl = &PG2_IMPL;

impl ::core::ops::Deref for Pg2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg2 {}

impl Pin<Gpiog> for Pg2 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::FsmcA12> for Pg2 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG3: Pg3 = Pg3 {}; 
pub const PG3_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 3 };
pub const PG3_IMPL_REF: &PinImpl = &PG3_IMPL;

impl ::core::ops::Deref for Pg3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg3 {}

impl Pin<Gpiog> for Pg3 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::FsmcA13> for Pg3 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG4: Pg4 = Pg4 {}; 
pub const PG4_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 4 };
pub const PG4_IMPL_REF: &PinImpl = &PG4_IMPL;

impl ::core::ops::Deref for Pg4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg4 {}

impl Pin<Gpiog> for Pg4 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::FsmcA14> for Pg4 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG5: Pg5 = Pg5 {}; 
pub const PG5_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 5 };
pub const PG5_IMPL_REF: &PinImpl = &PG5_IMPL;

impl ::core::ops::Deref for Pg5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg5 {}

impl Pin<Gpiog> for Pg5 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::FsmcA15> for Pg5 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG6: Pg6 = Pg6 {}; 
pub const PG6_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 6 };
pub const PG6_IMPL_REF: &PinImpl = &PG6_IMPL;

impl ::core::ops::Deref for Pg6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg6 {}

impl Pin<Gpiog> for Pg6 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::FsmcInt2> for Pg6 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG7: Pg7 = Pg7 {}; 
pub const PG7_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 7 };
pub const PG7_IMPL_REF: &PinImpl = &PG7_IMPL;

impl ::core::ops::Deref for Pg7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg7 {}

impl Pin<Gpiog> for Pg7 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Usart6Ck> for Pg7 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::FsmcInt3> for Pg7 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG8: Pg8 = Pg8 {}; 
pub const PG8_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 8 };
pub const PG8_IMPL_REF: &PinImpl = &PG8_IMPL;

impl ::core::ops::Deref for Pg8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg8 {}

impl Pin<Gpiog> for Pg8 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Usart6Rts> for Pg8 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EthPpsOut> for Pg8 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pg8 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG9: Pg9 = Pg9 {}; 
pub const PG9_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 9 };
pub const PG9_IMPL_REF: &PinImpl = &PG9_IMPL;

impl ::core::ops::Deref for Pg9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg9 {}

impl Pin<Gpiog> for Pg9 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Usart6Rx> for Pg9 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::FsmcNe2> for Pg9 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::FsmcNce3> for Pg9 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg9 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG10: Pg10 = Pg10 {}; 
pub const PG10_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 10 };
pub const PG10_IMPL_REF: &PinImpl = &PG10_IMPL;

impl ::core::ops::Deref for Pg10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg10 {}

impl Pin<Gpiog> for Pg10 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::FsmcNce41> for Pg10 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::FsmcNe3> for Pg10 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg10 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG11: Pg11 = Pg11 {}; 
pub const PG11_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 11 };
pub const PG11_IMPL_REF: &PinImpl = &PG11_IMPL;

impl ::core::ops::Deref for Pg11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg11 {}

impl Pin<Gpiog> for Pg11 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthMiiTxEn> for Pg11 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxEn> for Pg11 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FsmcNce42> for Pg11 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg11 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG12: Pg12 = Pg12 {}; 
pub const PG12_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 12 };
pub const PG12_IMPL_REF: &PinImpl = &PG12_IMPL;

impl ::core::ops::Deref for Pg12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg12 {}

impl Pin<Gpiog> for Pg12 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Usart6Rts> for Pg12 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::FsmcNe4> for Pg12 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg12 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG13: Pg13 = Pg13 {}; 
pub const PG13_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 13 };
pub const PG13_IMPL_REF: &PinImpl = &PG13_IMPL;

impl ::core::ops::Deref for Pg13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg13 {}

impl Pin<Gpiog> for Pg13 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Usart6Cts> for Pg13 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EthMiiTxd0> for Pg13 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxd0> for Pg13 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FsmcA24> for Pg13 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg13 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG14: Pg14 = Pg14 {}; 
pub const PG14_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 14 };
pub const PG14_IMPL_REF: &PinImpl = &PG14_IMPL;

impl ::core::ops::Deref for Pg14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg14 {}

impl Pin<Gpiog> for Pg14 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Usart6Tx> for Pg14 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EthMiiTxd1> for Pg14 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxd1> for Pg14 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FsmcA25> for Pg14 {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg14 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG15: Pg15 = Pg15 {}; 
pub const PG15_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 15 };
pub const PG15_IMPL_REF: &PinImpl = &PG15_IMPL;

impl ::core::ops::Deref for Pg15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg15 {}

impl Pin<Gpiog> for Pg15 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Usart6Cts> for Pg15 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::DcmiD13> for Pg15 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pg15 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH0: Ph0 = Ph0 {}; 
pub const PH0_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 0 };
pub const PH0_IMPL_REF: &PinImpl = &PH0_IMPL;

impl ::core::ops::Deref for Ph0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph0 {}

impl Pin<Gpioh> for Ph0 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Eventout> for Ph0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH1: Ph1 = Ph1 {}; 
pub const PH1_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 1 };
pub const PH1_IMPL_REF: &PinImpl = &PH1_IMPL;

impl ::core::ops::Deref for Ph1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph1 {}

impl Pin<Gpioh> for Ph1 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Eventout> for Ph1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH2: Ph2 = Ph2 {}; 
pub const PH2_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 2 };
pub const PH2_IMPL_REF: &PinImpl = &PH2_IMPL;

impl ::core::ops::Deref for Ph2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph2 {}

impl Pin<Gpioh> for Ph2 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::EthMiiCrs> for Ph2 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Ph2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH3: Ph3 = Ph3 {}; 
pub const PH3_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 3 };
pub const PH3_IMPL_REF: &PinImpl = &PH3_IMPL;

impl ::core::ops::Deref for Ph3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph3 {}

impl Pin<Gpioh> for Ph3 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::EthMiiCol> for Ph3 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Ph3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH4: Ph4 = Ph4 {}; 
pub const PH4_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 4 };
pub const PH4_IMPL_REF: &PinImpl = &PH4_IMPL;

impl ::core::ops::Deref for Ph4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph4 {}

impl Pin<Gpioh> for Ph4 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2c2Scl> for Ph4 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::OtgHsUlpiNxt> for Ph4 {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Ph4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH5: Ph5 = Ph5 {}; 
pub const PH5_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 5 };
pub const PH5_IMPL_REF: &PinImpl = &PH5_IMPL;

impl ::core::ops::Deref for Ph5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph5 {}

impl Pin<Gpioh> for Ph5 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c2Sda> for Ph5 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Eventout> for Ph5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH6: Ph6 = Ph6 {}; 
pub const PH6_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 6 };
pub const PH6_IMPL_REF: &PinImpl = &PH6_IMPL;

impl ::core::ops::Deref for Ph6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph6 {}

impl Pin<Gpioh> for Ph6 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2c2Smba> for Ph6 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim12Ch1> for Ph6 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::EthMiiRxd2> for Ph6 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Ph6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH7: Ph7 = Ph7 {}; 
pub const PH7_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 7 };
pub const PH7_IMPL_REF: &PinImpl = &PH7_IMPL;

impl ::core::ops::Deref for Ph7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph7 {}

impl Pin<Gpioh> for Ph7 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::I2c3Scl> for Ph7 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::EthMiiRxd3> for Ph7 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Ph7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH8: Ph8 = Ph8 {}; 
pub const PH8_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 8 };
pub const PH8_IMPL_REF: &PinImpl = &PH8_IMPL;

impl ::core::ops::Deref for Ph8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph8 {}

impl Pin<Gpioh> for Ph8 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::I2c3Sda> for Ph8 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::DcmiHsync> for Ph8 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Ph8 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH9: Ph9 = Ph9 {}; 
pub const PH9_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 9 };
pub const PH9_IMPL_REF: &PinImpl = &PH9_IMPL;

impl ::core::ops::Deref for Ph9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph9 {}

impl Pin<Gpioh> for Ph9 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::I2c3Smba> for Ph9 {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim12Ch2> for Ph9 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::DcmiD0> for Ph9 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Ph9 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH10: Ph10 = Ph10 {}; 
pub const PH10_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 10 };
pub const PH10_IMPL_REF: &PinImpl = &PH10_IMPL;

impl ::core::ops::Deref for Ph10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph10 {}

impl Pin<Gpioh> for Ph10 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::Tim5Ch1> for Ph10 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::DcmiD1> for Ph10 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Ph10 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH11: Ph11 = Ph11 {}; 
pub const PH11_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 11 };
pub const PH11_IMPL_REF: &PinImpl = &PH11_IMPL;

impl ::core::ops::Deref for Ph11 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph11 {}

impl Pin<Gpioh> for Ph11 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 11 }
}

impl AltFn<super::sig::Tim5Ch2> for Ph11 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::DcmiD2> for Ph11 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Ph11 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH12: Ph12 = Ph12 {}; 
pub const PH12_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 12 };
pub const PH12_IMPL_REF: &PinImpl = &PH12_IMPL;

impl ::core::ops::Deref for Ph12 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph12 {}

impl Pin<Gpioh> for Ph12 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 12 }
}

impl AltFn<super::sig::Tim5Ch3> for Ph12 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::DcmiD3> for Ph12 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Ph12 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH13: Ph13 = Ph13 {}; 
pub const PH13_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 13 };
pub const PH13_IMPL_REF: &PinImpl = &PH13_IMPL;

impl ::core::ops::Deref for Ph13 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph13 {}

impl Pin<Gpioh> for Ph13 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 13 }
}

impl AltFn<super::sig::Tim8Ch1n> for Ph13 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can1Tx> for Ph13 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Ph13 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH14: Ph14 = Ph14 {}; 
pub const PH14_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 14 };
pub const PH14_IMPL_REF: &PinImpl = &PH14_IMPL;

impl ::core::ops::Deref for Ph14 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph14 {}

impl Pin<Gpioh> for Ph14 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 14 }
}

impl AltFn<super::sig::Tim8Ch2n> for Ph14 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::DcmiD4> for Ph14 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Ph14 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH15: Ph15 = Ph15 {}; 
pub const PH15_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 15 };
pub const PH15_IMPL_REF: &PinImpl = &PH15_IMPL;

impl ::core::ops::Deref for Ph15 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph15 {}

impl Pin<Gpioh> for Ph15 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 15 }
}

impl AltFn<super::sig::Tim8Ch3n> for Ph15 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::DcmiD11> for Ph15 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Ph15 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI0: Pi0 = Pi0 {}; 
pub const PI0_IMPL: PinImpl = PinImpl { port: GPIOI_IMPL, index: 0 };
pub const PI0_IMPL_REF: &PinImpl = &PI0_IMPL;

impl ::core::ops::Deref for Pi0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PI0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pi0 {}

impl Pin<Gpioi> for Pi0 {
   #[inline]
   fn port(&self) -> Gpioi { GPIOI }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim5Ch4> for Pi0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Spi2Nss> for Pi0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ws> for Pi0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::DcmiD13> for Pi0 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pi0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI1: Pi1 = Pi1 {}; 
pub const PI1_IMPL: PinImpl = PinImpl { port: GPIOI_IMPL, index: 1 };
pub const PI1_IMPL_REF: &PinImpl = &PI1_IMPL;

impl ::core::ops::Deref for Pi1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PI1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pi1 {}

impl Pin<Gpioi> for Pi1 {
   #[inline]
   fn port(&self) -> Gpioi { GPIOI }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sck> for Pi1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pi1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::DcmiD8> for Pi1 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pi1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI2: Pi2 = Pi2 {}; 
pub const PI2_IMPL: PinImpl = PinImpl { port: GPIOI_IMPL, index: 2 };
pub const PI2_IMPL_REF: &PinImpl = &PI2_IMPL;

impl ::core::ops::Deref for Pi2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PI2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pi2 {}

impl Pin<Gpioi> for Pi2 {
   #[inline]
   fn port(&self) -> Gpioi { GPIOI }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch4> for Pi2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Miso> for Pi2 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2extSd> for Pi2 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::DcmiD9> for Pi2 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pi2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI3: Pi3 = Pi3 {}; 
pub const PI3_IMPL: PinImpl = PinImpl { port: GPIOI_IMPL, index: 3 };
pub const PI3_IMPL_REF: &PinImpl = &PI3_IMPL;

impl ::core::ops::Deref for Pi3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PI3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pi3 {}

impl Pin<Gpioi> for Pi3 {
   #[inline]
   fn port(&self) -> Gpioi { GPIOI }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim8Etr> for Pi3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Mosi> for Pi3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Sd> for Pi3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::DcmiD10> for Pi3 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pi3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI4: Pi4 = Pi4 {}; 
pub const PI4_IMPL: PinImpl = PinImpl { port: GPIOI_IMPL, index: 4 };
pub const PI4_IMPL_REF: &PinImpl = &PI4_IMPL;

impl ::core::ops::Deref for Pi4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PI4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pi4 {}

impl Pin<Gpioi> for Pi4 {
   #[inline]
   fn port(&self) -> Gpioi { GPIOI }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim8Bkin> for Pi4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::DcmiD5> for Pi4 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pi4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI5: Pi5 = Pi5 {}; 
pub const PI5_IMPL: PinImpl = PinImpl { port: GPIOI_IMPL, index: 5 };
pub const PI5_IMPL_REF: &PinImpl = &PI5_IMPL;

impl ::core::ops::Deref for Pi5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PI5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pi5 {}

impl Pin<Gpioi> for Pi5 {
   #[inline]
   fn port(&self) -> Gpioi { GPIOI }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim8Ch1> for Pi5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::DcmiVsync> for Pi5 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pi5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI6: Pi6 = Pi6 {}; 
pub const PI6_IMPL: PinImpl = PinImpl { port: GPIOI_IMPL, index: 6 };
pub const PI6_IMPL_REF: &PinImpl = &PI6_IMPL;

impl ::core::ops::Deref for Pi6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PI6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pi6 {}

impl Pin<Gpioi> for Pi6 {
   #[inline]
   fn port(&self) -> Gpioi { GPIOI }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Tim8Ch2> for Pi6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::DcmiD6> for Pi6 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pi6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI7: Pi7 = Pi7 {}; 
pub const PI7_IMPL: PinImpl = PinImpl { port: GPIOI_IMPL, index: 7 };
pub const PI7_IMPL_REF: &PinImpl = &PI7_IMPL;

impl ::core::ops::Deref for Pi7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PI7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pi7 {}

impl Pin<Gpioi> for Pi7 {
   #[inline]
   fn port(&self) -> Gpioi { GPIOI }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim8Ch3> for Pi7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::DcmiD7> for Pi7 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pi7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI8: Pi8 = Pi8 {}; 
pub const PI8_IMPL: PinImpl = PinImpl { port: GPIOI_IMPL, index: 8 };
pub const PI8_IMPL_REF: &PinImpl = &PI8_IMPL;

impl ::core::ops::Deref for Pi8 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PI8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pi8 {}

impl Pin<Gpioi> for Pi8 {
   #[inline]
   fn port(&self) -> Gpioi { GPIOI }
   #[inline]
   fn index(&self) -> usize { 8 }
}

impl AltFn<super::sig::Eventout> for Pi8 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI9: Pi9 = Pi9 {}; 
pub const PI9_IMPL: PinImpl = PinImpl { port: GPIOI_IMPL, index: 9 };
pub const PI9_IMPL_REF: &PinImpl = &PI9_IMPL;

impl ::core::ops::Deref for Pi9 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PI9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pi9 {}

impl Pin<Gpioi> for Pi9 {
   #[inline]
   fn port(&self) -> Gpioi { GPIOI }
   #[inline]
   fn index(&self) -> usize { 9 }
}

impl AltFn<super::sig::Can1Rx> for Pi9 {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Pi9 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI10: Pi10 = Pi10 {}; 
pub const PI10_IMPL: PinImpl = PinImpl { port: GPIOI_IMPL, index: 10 };
pub const PI10_IMPL_REF: &PinImpl = &PI10_IMPL;

impl ::core::ops::Deref for Pi10 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PI10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pi10 {}

impl Pin<Gpioi> for Pi10 {
   #[inline]
   fn port(&self) -> Gpioi { GPIOI }
   #[inline]
   fn index(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiRxEr> for Pi10 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pi10 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

