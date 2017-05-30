pub use stm32_common::chip::gpio::*;

pub const GPIOA: Gpioa = Gpioa {};
pub const GPIOA_IMPL: GpioImpl = GpioImpl(0x50000000);
pub const GPIOA_IMPL_REF: &GpioImpl = &GPIOA_IMPL;

pub struct Gpioa {}
impl ::core::ops::Deref for Gpioa {
   type Target = GpioImpl;
   fn deref(&self) -> &GpioImpl { GPIOA_IMPL_REF }
}

pub const GPIOB: Gpiob = Gpiob {};
pub const GPIOB_IMPL: GpioImpl = GpioImpl(0x50000400);
pub const GPIOB_IMPL_REF: &GpioImpl = &GPIOB_IMPL;

pub struct Gpiob {}
impl ::core::ops::Deref for Gpiob {
   type Target = GpioImpl;
   fn deref(&self) -> &GpioImpl { GPIOB_IMPL_REF }
}

pub const GPIOC: Gpioc = Gpioc {};
pub const GPIOC_IMPL: GpioImpl = GpioImpl(0x50000800);
pub const GPIOC_IMPL_REF: &GpioImpl = &GPIOC_IMPL;

pub struct Gpioc {}
impl ::core::ops::Deref for Gpioc {
   type Target = GpioImpl;
   fn deref(&self) -> &GpioImpl { GPIOC_IMPL_REF }
}

pub const GPIOH: Gpioh = Gpioh {};
pub const GPIOH_IMPL: GpioImpl = GpioImpl(0x50001c00);
pub const GPIOH_IMPL_REF: &GpioImpl = &GPIOH_IMPL;

pub struct Gpioh {}
impl ::core::ops::Deref for Gpioh {
   type Target = GpioImpl;
   fn deref(&self) -> &GpioImpl { GPIOH_IMPL_REF }
}


pub struct PinImpl {
  pub port: GpioImpl,
  pub index: usize,
}

pub trait Pin<T> {
   fn port(&self) -> T;
   fn index(&self) -> usize;
}

pub const PA0: Pa0 = Pa0 {}; 
pub const PA0_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 0 };
pub const PA0_IMPL_REF: &PinImpl = &PA0_IMPL;

impl ::core::ops::Deref for Pa0 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa0 {}

impl Pin<Gpioa> for Pa0 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 0 }
}

pub const PA1: Pa1 = Pa1 {}; 
pub const PA1_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 1 };
pub const PA1_IMPL_REF: &PinImpl = &PA1_IMPL;

impl ::core::ops::Deref for Pa1 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa1 {}

impl Pin<Gpioa> for Pa1 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 1 }
}

pub const PA2: Pa2 = Pa2 {}; 
pub const PA2_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 2 };
pub const PA2_IMPL_REF: &PinImpl = &PA2_IMPL;

impl ::core::ops::Deref for Pa2 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa2 {}

impl Pin<Gpioa> for Pa2 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 2 }
}

pub const PA3: Pa3 = Pa3 {}; 
pub const PA3_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 3 };
pub const PA3_IMPL_REF: &PinImpl = &PA3_IMPL;

impl ::core::ops::Deref for Pa3 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa3 {}

impl Pin<Gpioa> for Pa3 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 3 }
}

pub const PA4: Pa4 = Pa4 {}; 
pub const PA4_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 4 };
pub const PA4_IMPL_REF: &PinImpl = &PA4_IMPL;

impl ::core::ops::Deref for Pa4 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa4 {}

impl Pin<Gpioa> for Pa4 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 4 }
}

pub const PA5: Pa5 = Pa5 {}; 
pub const PA5_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 5 };
pub const PA5_IMPL_REF: &PinImpl = &PA5_IMPL;

impl ::core::ops::Deref for Pa5 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa5 {}

impl Pin<Gpioa> for Pa5 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 5 }
}

pub const PA6: Pa6 = Pa6 {}; 
pub const PA6_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 6 };
pub const PA6_IMPL_REF: &PinImpl = &PA6_IMPL;

impl ::core::ops::Deref for Pa6 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa6 {}

impl Pin<Gpioa> for Pa6 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 6 }
}

pub const PA7: Pa7 = Pa7 {}; 
pub const PA7_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 7 };
pub const PA7_IMPL_REF: &PinImpl = &PA7_IMPL;

impl ::core::ops::Deref for Pa7 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa7 {}

impl Pin<Gpioa> for Pa7 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 7 }
}

pub const PA8: Pa8 = Pa8 {}; 
pub const PA8_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 8 };
pub const PA8_IMPL_REF: &PinImpl = &PA8_IMPL;

impl ::core::ops::Deref for Pa8 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa8 {}

impl Pin<Gpioa> for Pa8 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 8 }
}

pub const PA9: Pa9 = Pa9 {}; 
pub const PA9_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 9 };
pub const PA9_IMPL_REF: &PinImpl = &PA9_IMPL;

impl ::core::ops::Deref for Pa9 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa9 {}

impl Pin<Gpioa> for Pa9 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 9 }
}

pub const PA10: Pa10 = Pa10 {}; 
pub const PA10_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 10 };
pub const PA10_IMPL_REF: &PinImpl = &PA10_IMPL;

impl ::core::ops::Deref for Pa10 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa10 {}

impl Pin<Gpioa> for Pa10 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 10 }
}

pub const PA11: Pa11 = Pa11 {}; 
pub const PA11_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 11 };
pub const PA11_IMPL_REF: &PinImpl = &PA11_IMPL;

impl ::core::ops::Deref for Pa11 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa11 {}

impl Pin<Gpioa> for Pa11 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 11 }
}

pub const PA12: Pa12 = Pa12 {}; 
pub const PA12_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 12 };
pub const PA12_IMPL_REF: &PinImpl = &PA12_IMPL;

impl ::core::ops::Deref for Pa12 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa12 {}

impl Pin<Gpioa> for Pa12 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 12 }
}

pub const PA13: Pa13 = Pa13 {}; 
pub const PA13_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 13 };
pub const PA13_IMPL_REF: &PinImpl = &PA13_IMPL;

impl ::core::ops::Deref for Pa13 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa13 {}

impl Pin<Gpioa> for Pa13 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 13 }
}

pub const PA14: Pa14 = Pa14 {}; 
pub const PA14_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 14 };
pub const PA14_IMPL_REF: &PinImpl = &PA14_IMPL;

impl ::core::ops::Deref for Pa14 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa14 {}

impl Pin<Gpioa> for Pa14 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 14 }
}

pub const PA15: Pa15 = Pa15 {}; 
pub const PA15_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 15 };
pub const PA15_IMPL_REF: &PinImpl = &PA15_IMPL;

impl ::core::ops::Deref for Pa15 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PA15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa15 {}

impl Pin<Gpioa> for Pa15 {
   fn port(&self) -> Gpioa { GPIOA }
   fn index(&self) -> usize { 15 }
}

pub const PB0: Pb0 = Pb0 {}; 
pub const PB0_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 0 };
pub const PB0_IMPL_REF: &PinImpl = &PB0_IMPL;

impl ::core::ops::Deref for Pb0 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb0 {}

impl Pin<Gpiob> for Pb0 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 0 }
}

pub const PB1: Pb1 = Pb1 {}; 
pub const PB1_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 1 };
pub const PB1_IMPL_REF: &PinImpl = &PB1_IMPL;

impl ::core::ops::Deref for Pb1 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb1 {}

impl Pin<Gpiob> for Pb1 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 1 }
}

pub const PB2: Pb2 = Pb2 {}; 
pub const PB2_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 2 };
pub const PB2_IMPL_REF: &PinImpl = &PB2_IMPL;

impl ::core::ops::Deref for Pb2 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb2 {}

impl Pin<Gpiob> for Pb2 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 2 }
}

pub const PB3: Pb3 = Pb3 {}; 
pub const PB3_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 3 };
pub const PB3_IMPL_REF: &PinImpl = &PB3_IMPL;

impl ::core::ops::Deref for Pb3 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb3 {}

impl Pin<Gpiob> for Pb3 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 3 }
}

pub const PB4: Pb4 = Pb4 {}; 
pub const PB4_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 4 };
pub const PB4_IMPL_REF: &PinImpl = &PB4_IMPL;

impl ::core::ops::Deref for Pb4 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb4 {}

impl Pin<Gpiob> for Pb4 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 4 }
}

pub const PB5: Pb5 = Pb5 {}; 
pub const PB5_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 5 };
pub const PB5_IMPL_REF: &PinImpl = &PB5_IMPL;

impl ::core::ops::Deref for Pb5 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb5 {}

impl Pin<Gpiob> for Pb5 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 5 }
}

pub const PB6: Pb6 = Pb6 {}; 
pub const PB6_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 6 };
pub const PB6_IMPL_REF: &PinImpl = &PB6_IMPL;

impl ::core::ops::Deref for Pb6 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb6 {}

impl Pin<Gpiob> for Pb6 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 6 }
}

pub const PB7: Pb7 = Pb7 {}; 
pub const PB7_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 7 };
pub const PB7_IMPL_REF: &PinImpl = &PB7_IMPL;

impl ::core::ops::Deref for Pb7 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb7 {}

impl Pin<Gpiob> for Pb7 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 7 }
}

pub const PB8: Pb8 = Pb8 {}; 
pub const PB8_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 8 };
pub const PB8_IMPL_REF: &PinImpl = &PB8_IMPL;

impl ::core::ops::Deref for Pb8 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB8_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb8 {}

impl Pin<Gpiob> for Pb8 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 8 }
}

pub const PB9: Pb9 = Pb9 {}; 
pub const PB9_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 9 };
pub const PB9_IMPL_REF: &PinImpl = &PB9_IMPL;

impl ::core::ops::Deref for Pb9 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB9_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb9 {}

impl Pin<Gpiob> for Pb9 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 9 }
}

pub const PB10: Pb10 = Pb10 {}; 
pub const PB10_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 10 };
pub const PB10_IMPL_REF: &PinImpl = &PB10_IMPL;

impl ::core::ops::Deref for Pb10 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB10_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb10 {}

impl Pin<Gpiob> for Pb10 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 10 }
}

pub const PB11: Pb11 = Pb11 {}; 
pub const PB11_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 11 };
pub const PB11_IMPL_REF: &PinImpl = &PB11_IMPL;

impl ::core::ops::Deref for Pb11 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB11_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb11 {}

impl Pin<Gpiob> for Pb11 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 11 }
}

pub const PB12: Pb12 = Pb12 {}; 
pub const PB12_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 12 };
pub const PB12_IMPL_REF: &PinImpl = &PB12_IMPL;

impl ::core::ops::Deref for Pb12 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB12_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb12 {}

impl Pin<Gpiob> for Pb12 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 12 }
}

pub const PB13: Pb13 = Pb13 {}; 
pub const PB13_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 13 };
pub const PB13_IMPL_REF: &PinImpl = &PB13_IMPL;

impl ::core::ops::Deref for Pb13 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB13_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb13 {}

impl Pin<Gpiob> for Pb13 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 13 }
}

pub const PB14: Pb14 = Pb14 {}; 
pub const PB14_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 14 };
pub const PB14_IMPL_REF: &PinImpl = &PB14_IMPL;

impl ::core::ops::Deref for Pb14 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb14 {}

impl Pin<Gpiob> for Pb14 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 14 }
}

pub const PB15: Pb15 = Pb15 {}; 
pub const PB15_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 15 };
pub const PB15_IMPL_REF: &PinImpl = &PB15_IMPL;

impl ::core::ops::Deref for Pb15 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PB15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb15 {}

impl Pin<Gpiob> for Pb15 {
   fn port(&self) -> Gpiob { GPIOB }
   fn index(&self) -> usize { 15 }
}

pub const PC0: Pc0 = Pc0 {}; 
pub const PC0_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 0 };
pub const PC0_IMPL_REF: &PinImpl = &PC0_IMPL;

impl ::core::ops::Deref for Pc0 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PC0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc0 {}

impl Pin<Gpioc> for Pc0 {
   fn port(&self) -> Gpioc { GPIOC }
   fn index(&self) -> usize { 0 }
}

pub const PC14: Pc14 = Pc14 {}; 
pub const PC14_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 14 };
pub const PC14_IMPL_REF: &PinImpl = &PC14_IMPL;

impl ::core::ops::Deref for Pc14 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PC14_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc14 {}

impl Pin<Gpioc> for Pc14 {
   fn port(&self) -> Gpioc { GPIOC }
   fn index(&self) -> usize { 14 }
}

pub const PC15: Pc15 = Pc15 {}; 
pub const PC15_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 15 };
pub const PC15_IMPL_REF: &PinImpl = &PC15_IMPL;

impl ::core::ops::Deref for Pc15 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PC15_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc15 {}

impl Pin<Gpioc> for Pc15 {
   fn port(&self) -> Gpioc { GPIOC }
   fn index(&self) -> usize { 15 }
}

pub const PH0: Ph0 = Ph0 {}; 
pub const PH0_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 0 };
pub const PH0_IMPL_REF: &PinImpl = &PH0_IMPL;

impl ::core::ops::Deref for Ph0 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PH0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph0 {}

impl Pin<Gpioh> for Ph0 {
   fn port(&self) -> Gpioh { GPIOH }
   fn index(&self) -> usize { 0 }
}

pub const PH1: Ph1 = Ph1 {}; 
pub const PH1_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 1 };
pub const PH1_IMPL_REF: &PinImpl = &PH1_IMPL;

impl ::core::ops::Deref for Ph1 {
   type Target = PinImpl;
   fn deref(&self) -> &PinImpl { PH1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph1 {}

impl Pin<Gpioh> for Ph1 {
   fn port(&self) -> Gpioh { GPIOH }
   fn index(&self) -> usize { 1 }
}

pub trait AfLptim1In1 {
   fn af_lptim1_in1(&self) -> usize;
}

pub trait AfTim2Ch1 {
   fn af_tim2_ch1(&self) -> usize;
}

pub trait AfUsart2Cts {
   fn af_usart2_cts(&self) -> usize;
}

pub trait AfTim2Etr {
   fn af_tim2_etr(&self) -> usize;
}

pub trait AfComp1Out {
   fn af_comp1_out(&self) -> usize;
}

pub trait AfEventout {
   fn af_eventout(&self) -> usize;
}

pub trait AfLptim1In2 {
   fn af_lptim1_in2(&self) -> usize;
}

pub trait AfTim2Ch2 {
   fn af_tim2_ch2(&self) -> usize;
}

pub trait AfI2c1Smba {
   fn af_i2c1_smba(&self) -> usize;
}

pub trait AfUsart2Rts {
   fn af_usart2_rts(&self) -> usize;
}

pub trait AfTim21Etr {
   fn af_tim21_etr(&self) -> usize;
}

pub trait AfTim21Ch1 {
   fn af_tim21_ch1(&self) -> usize;
}

pub trait AfTim2Ch3 {
   fn af_tim2_ch3(&self) -> usize;
}

pub trait AfUsart2Tx {
   fn af_usart2_tx(&self) -> usize;
}

pub trait AfLpuart1Tx {
   fn af_lpuart1_tx(&self) -> usize;
}

pub trait AfComp2Out {
   fn af_comp2_out(&self) -> usize;
}

pub trait AfTim21Ch2 {
   fn af_tim21_ch2(&self) -> usize;
}

pub trait AfTim2Ch4 {
   fn af_tim2_ch4(&self) -> usize;
}

pub trait AfUsart2Rx {
   fn af_usart2_rx(&self) -> usize;
}

pub trait AfLpuart1Rx {
   fn af_lpuart1_rx(&self) -> usize;
}

pub trait AfSpi1Nss {
   fn af_spi1_nss(&self) -> usize;
}

pub trait AfUart2Ck {
   fn af_uart2_ck(&self) -> usize;
}

pub trait AfTim22Etr {
   fn af_tim22_etr(&self) -> usize;
}

pub trait AfSpi1Sck {
   fn af_spi1_sck(&self) -> usize;
}

pub trait AfSpiMiso {
   fn af_spi_miso(&self) -> usize;
}

pub trait AfLptim1Etr {
   fn af_lptim1_etr(&self) -> usize;
}

pub trait AfLpuart1Cts {
   fn af_lpuart1_cts(&self) -> usize;
}

pub trait AfTim22Ch1 {
   fn af_tim22_ch1(&self) -> usize;
}

pub trait AfSpi1Mosi {
   fn af_spi1_mosi(&self) -> usize;
}

pub trait AfLptim1Out {
   fn af_lptim1_out(&self) -> usize;
}

pub trait AfTim22Ch2 {
   fn af_tim22_ch2(&self) -> usize;
}

pub trait AfMco {
   fn af_mco(&self) -> usize;
}

pub trait AfUsart2Ck {
   fn af_usart2_ck(&self) -> usize;
}

pub trait AfI2c1Scl {
   fn af_i2c1_scl(&self) -> usize;
}

pub trait AfI2c1Sda {
   fn af_i2c1_sda(&self) -> usize;
}

pub trait AfSpi1Mio {
   fn af_spi1_mio(&self) -> usize;
}

pub trait AfSwdio {
   fn af_swdio(&self) -> usize;
}

pub trait AfSwclk {
   fn af_swclk(&self) -> usize;
}

pub trait AfSpi1Miso {
   fn af_spi1_miso(&self) -> usize;
}

pub trait AfLpuart1Rts {
   fn af_lpuart1_rts(&self) -> usize;
}

pub trait AfSpiSck {
   fn af_spi_sck(&self) -> usize;
}

pub trait AfRtcOut {
   fn af_rtc_out(&self) -> usize;
}

pub trait AfRtcRefin {
   fn af_rtc_refin(&self) -> usize;
}

impl AfLptim1In1 for Pa0 {
   fn af_lptim1_in1(&self) -> usize { 1 }
}

impl AfTim2Ch1 for Pa0 {
   fn af_tim2_ch1(&self) -> usize { 2 }
}

impl AfUsart2Cts for Pa0 {
   fn af_usart2_cts(&self) -> usize { 4 }
}

impl AfTim2Etr for Pa0 {
   fn af_tim2_etr(&self) -> usize { 5 }
}

impl AfComp1Out for Pa0 {
   fn af_comp1_out(&self) -> usize { 7 }
}

impl AfEventout for Pa1 {
   fn af_eventout(&self) -> usize { 0 }
}

impl AfLptim1In2 for Pa1 {
   fn af_lptim1_in2(&self) -> usize { 1 }
}

impl AfTim2Ch2 for Pa1 {
   fn af_tim2_ch2(&self) -> usize { 2 }
}

impl AfI2c1Smba for Pa1 {
   fn af_i2c1_smba(&self) -> usize { 3 }
}

impl AfUsart2Rts for Pa1 {
   fn af_usart2_rts(&self) -> usize { 4 }
}

impl AfTim21Etr for Pa1 {
   fn af_tim21_etr(&self) -> usize { 5 }
}

impl AfTim21Ch1 for Pa2 {
   fn af_tim21_ch1(&self) -> usize { 0 }
}

impl AfTim2Ch3 for Pa2 {
   fn af_tim2_ch3(&self) -> usize { 2 }
}

impl AfUsart2Tx for Pa2 {
   fn af_usart2_tx(&self) -> usize { 4 }
}

impl AfLpuart1Tx for Pa2 {
   fn af_lpuart1_tx(&self) -> usize { 6 }
}

impl AfComp2Out for Pa2 {
   fn af_comp2_out(&self) -> usize { 7 }
}

impl AfTim21Ch2 for Pa3 {
   fn af_tim21_ch2(&self) -> usize { 0 }
}

impl AfTim2Ch4 for Pa3 {
   fn af_tim2_ch4(&self) -> usize { 2 }
}

impl AfUsart2Rx for Pa3 {
   fn af_usart2_rx(&self) -> usize { 4 }
}

impl AfLpuart1Rx for Pa3 {
   fn af_lpuart1_rx(&self) -> usize { 6 }
}

impl AfSpi1Nss for Pa4 {
   fn af_spi1_nss(&self) -> usize { 0 }
}

impl AfLptim1In1 for Pa4 {
   fn af_lptim1_in1(&self) -> usize { 1 }
}

impl AfUart2Ck for Pa4 {
   fn af_uart2_ck(&self) -> usize { 4 }
}

impl AfTim22Etr for Pa4 {
   fn af_tim22_etr(&self) -> usize { 5 }
}

impl AfSpi1Sck for Pa5 {
   fn af_spi1_sck(&self) -> usize { 0 }
}

impl AfLptim1In2 for Pa5 {
   fn af_lptim1_in2(&self) -> usize { 1 }
}

impl AfTim2Etr for Pa5 {
   fn af_tim2_etr(&self) -> usize { 3 }
}

impl AfTim2Ch1 for Pa5 {
   fn af_tim2_ch1(&self) -> usize { 5 }
}

impl AfSpiMiso for Pa6 {
   fn af_spi_miso(&self) -> usize { 0 }
}

impl AfLptim1Etr for Pa6 {
   fn af_lptim1_etr(&self) -> usize { 1 }
}

impl AfLpuart1Cts for Pa6 {
   fn af_lpuart1_cts(&self) -> usize { 4 }
}

impl AfTim22Ch1 for Pa6 {
   fn af_tim22_ch1(&self) -> usize { 5 }
}

impl AfEventout for Pa6 {
   fn af_eventout(&self) -> usize { 6 }
}

impl AfComp1Out for Pa6 {
   fn af_comp1_out(&self) -> usize { 7 }
}

impl AfSpi1Mosi for Pa7 {
   fn af_spi1_mosi(&self) -> usize { 0 }
}

impl AfLptim1Out for Pa7 {
   fn af_lptim1_out(&self) -> usize { 1 }
}

impl AfUsart2Cts for Pa7 {
   fn af_usart2_cts(&self) -> usize { 4 }
}

impl AfTim22Ch2 for Pa7 {
   fn af_tim22_ch2(&self) -> usize { 5 }
}

impl AfEventout for Pa7 {
   fn af_eventout(&self) -> usize { 6 }
}

impl AfComp2Out for Pa7 {
   fn af_comp2_out(&self) -> usize { 7 }
}

impl AfMco for Pa8 {
   fn af_mco(&self) -> usize { 0 }
}

impl AfLptim1In1 for Pa8 {
   fn af_lptim1_in1(&self) -> usize { 2 }
}

impl AfEventout for Pa8 {
   fn af_eventout(&self) -> usize { 3 }
}

impl AfUsart2Ck for Pa8 {
   fn af_usart2_ck(&self) -> usize { 4 }
}

impl AfTim2Ch1 for Pa8 {
   fn af_tim2_ch1(&self) -> usize { 5 }
}

impl AfMco for Pa9 {
   fn af_mco(&self) -> usize { 0 }
}

impl AfI2c1Scl for Pa9 {
   fn af_i2c1_scl(&self) -> usize { 1 }
}

impl AfUsart2Tx for Pa9 {
   fn af_usart2_tx(&self) -> usize { 4 }
}

impl AfTim22Ch1 for Pa9 {
   fn af_tim22_ch1(&self) -> usize { 5 }
}

impl AfI2c1Sda for Pa10 {
   fn af_i2c1_sda(&self) -> usize { 1 }
}

impl AfUsart2Rx for Pa10 {
   fn af_usart2_rx(&self) -> usize { 4 }
}

impl AfTim22Ch2 for Pa10 {
   fn af_tim22_ch2(&self) -> usize { 5 }
}

impl AfSpi1Mio for Pa11 {
   fn af_spi1_mio(&self) -> usize { 0 }
}

impl AfEventout for Pa11 {
   fn af_eventout(&self) -> usize { 2 }
}

impl AfUsart2Cts for Pa11 {
   fn af_usart2_cts(&self) -> usize { 4 }
}

impl AfTim21Ch2 for Pa11 {
   fn af_tim21_ch2(&self) -> usize { 5 }
}

impl AfComp1Out for Pa11 {
   fn af_comp1_out(&self) -> usize { 7 }
}

impl AfSpi1Mosi for Pa12 {
   fn af_spi1_mosi(&self) -> usize { 0 }
}

impl AfEventout for Pa12 {
   fn af_eventout(&self) -> usize { 2 }
}

impl AfUsart2Rts for Pa12 {
   fn af_usart2_rts(&self) -> usize { 4 }
}

impl AfComp2Out for Pa12 {
   fn af_comp2_out(&self) -> usize { 7 }
}

impl AfSwdio for Pa13 {
   fn af_swdio(&self) -> usize { 0 }
}

impl AfLptim1Etr for Pa13 {
   fn af_lptim1_etr(&self) -> usize { 1 }
}

impl AfLpuart1Rx for Pa13 {
   fn af_lpuart1_rx(&self) -> usize { 6 }
}

impl AfSwclk for Pa14 {
   fn af_swclk(&self) -> usize { 0 }
}

impl AfLptim1Out for Pa14 {
   fn af_lptim1_out(&self) -> usize { 1 }
}

impl AfI2c1Smba for Pa14 {
   fn af_i2c1_smba(&self) -> usize { 3 }
}

impl AfUsart2Tx for Pa14 {
   fn af_usart2_tx(&self) -> usize { 4 }
}

impl AfLpuart1Tx for Pa14 {
   fn af_lpuart1_tx(&self) -> usize { 6 }
}

impl AfSpi1Nss for Pa15 {
   fn af_spi1_nss(&self) -> usize { 0 }
}

impl AfTim2Etr for Pa15 {
   fn af_tim2_etr(&self) -> usize { 2 }
}

impl AfEventout for Pa15 {
   fn af_eventout(&self) -> usize { 3 }
}

impl AfUsart2Rx for Pa15 {
   fn af_usart2_rx(&self) -> usize { 4 }
}

impl AfTim2Ch1 for Pa15 {
   fn af_tim2_ch1(&self) -> usize { 5 }
}

impl AfEventout for Pb0 {
   fn af_eventout(&self) -> usize { 0 }
}

impl AfSpi1Miso for Pb0 {
   fn af_spi1_miso(&self) -> usize { 1 }
}

impl AfUsart2Rts for Pb0 {
   fn af_usart2_rts(&self) -> usize { 4 }
}

impl AfTim2Ch3 for Pb0 {
   fn af_tim2_ch3(&self) -> usize { 5 }
}

impl AfUsart2Ck for Pb1 {
   fn af_usart2_ck(&self) -> usize { 0 }
}

impl AfSpi1Mosi for Pb1 {
   fn af_spi1_mosi(&self) -> usize { 1 }
}

impl AfLpuart1Rts for Pb1 {
   fn af_lpuart1_rts(&self) -> usize { 4 }
}

impl AfTim2Ch4 for Pb1 {
   fn af_tim2_ch4(&self) -> usize { 5 }
}

impl AfLptim1Out for Pb2 {
   fn af_lptim1_out(&self) -> usize { 2 }
}

impl AfSpi1Sck for Pb3 {
   fn af_spi1_sck(&self) -> usize { 0 }
}

impl AfTim2Ch2 for Pb3 {
   fn af_tim2_ch2(&self) -> usize { 2 }
}

impl AfEventout for Pb3 {
   fn af_eventout(&self) -> usize { 4 }
}

impl AfSpi1Miso for Pb4 {
   fn af_spi1_miso(&self) -> usize { 0 }
}

impl AfEventout for Pb4 {
   fn af_eventout(&self) -> usize { 2 }
}

impl AfTim22Ch1 for Pb4 {
   fn af_tim22_ch1(&self) -> usize { 4 }
}

impl AfSpi1Mosi for Pb5 {
   fn af_spi1_mosi(&self) -> usize { 0 }
}

impl AfLptim1In1 for Pb5 {
   fn af_lptim1_in1(&self) -> usize { 2 }
}

impl AfI2c1Smba for Pb5 {
   fn af_i2c1_smba(&self) -> usize { 3 }
}

impl AfTim22Ch2 for Pb5 {
   fn af_tim22_ch2(&self) -> usize { 4 }
}

impl AfUsart2Tx for Pb6 {
   fn af_usart2_tx(&self) -> usize { 0 }
}

impl AfI2c1Scl for Pb6 {
   fn af_i2c1_scl(&self) -> usize { 1 }
}

impl AfLptim1Etr for Pb6 {
   fn af_lptim1_etr(&self) -> usize { 2 }
}

impl AfTim21Ch1 for Pb6 {
   fn af_tim21_ch1(&self) -> usize { 5 }
}

impl AfUsart2Rx for Pb7 {
   fn af_usart2_rx(&self) -> usize { 0 }
}

impl AfI2c1Sda for Pb7 {
   fn af_i2c1_sda(&self) -> usize { 1 }
}

impl AfLptim1In2 for Pb7 {
   fn af_lptim1_in2(&self) -> usize { 2 }
}

impl AfI2c1Scl for Pb8 {
   fn af_i2c1_scl(&self) -> usize { 4 }
}

impl AfEventout for Pb9 {
   fn af_eventout(&self) -> usize { 2 }
}

impl AfI2c1Sda for Pb9 {
   fn af_i2c1_sda(&self) -> usize { 4 }
}

impl AfTim2Ch3 for Pb10 {
   fn af_tim2_ch3(&self) -> usize { 2 }
}

impl AfLpuart1Tx for Pb10 {
   fn af_lpuart1_tx(&self) -> usize { 6 }
}

impl AfEventout for Pb11 {
   fn af_eventout(&self) -> usize { 0 }
}

impl AfTim2Ch4 for Pb11 {
   fn af_tim2_ch4(&self) -> usize { 2 }
}

impl AfLpuart1Rx for Pb11 {
   fn af_lpuart1_rx(&self) -> usize { 6 }
}

impl AfSpi1Nss for Pb12 {
   fn af_spi1_nss(&self) -> usize { 0 }
}

impl AfEventout for Pb12 {
   fn af_eventout(&self) -> usize { 6 }
}

impl AfSpiSck for Pb13 {
   fn af_spi_sck(&self) -> usize { 0 }
}

impl AfMco for Pb13 {
   fn af_mco(&self) -> usize { 1 }
}

impl AfTim21Ch1 for Pb13 {
   fn af_tim21_ch1(&self) -> usize { 5 }
}

impl AfLpuart1Cts for Pb13 {
   fn af_lpuart1_cts(&self) -> usize { 6 }
}

impl AfSpi1Miso for Pb14 {
   fn af_spi1_miso(&self) -> usize { 0 }
}

impl AfRtcOut for Pb14 {
   fn af_rtc_out(&self) -> usize { 2 }
}

impl AfTim21Ch2 for Pb14 {
   fn af_tim21_ch2(&self) -> usize { 5 }
}

impl AfLpuart1Rts for Pb14 {
   fn af_lpuart1_rts(&self) -> usize { 6 }
}

impl AfSpi1Mosi for Pb15 {
   fn af_spi1_mosi(&self) -> usize { 0 }
}

impl AfRtcRefin for Pb15 {
   fn af_rtc_refin(&self) -> usize { 2 }
}

impl AfLptim1In1 for Pc0 {
   fn af_lptim1_in1(&self) -> usize { 0 }
}

impl AfEventout for Pc0 {
   fn af_eventout(&self) -> usize { 2 }
}

impl AfLpuart1Rx for Pc0 {
   fn af_lpuart1_rx(&self) -> usize { 6 }
}

