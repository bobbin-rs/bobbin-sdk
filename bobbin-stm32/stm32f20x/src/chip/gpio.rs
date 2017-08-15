#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::gpio::*;

pub const GPIOA: Gpioa = Periph(0x40020000, GpioaId {});
pub const GPIOB: Gpiob = Periph(0x40020400, GpiobId {});
pub const GPIOC: Gpioc = Periph(0x40020800, GpiocId {});
pub const GPIOD: Gpiod = Periph(0x40020c00, GpiodId {});
pub const GPIOE: Gpioe = Periph(0x40021000, GpioeId {});
pub const GPIOF: Gpiof = Periph(0x40021400, GpiofId {});
pub const GPIOG: Gpiog = Periph(0x40021800, GpiogId {});
pub const GPIOH: Gpioh = Periph(0x40021c00, GpiohId {});
pub const GPIOI: Gpioi = Periph(0x40022000, GpioiId {});
pub const GPIOJ: Gpioj = Periph(0x40022400, GpiojId {});
pub const GPIOK: Gpiok = Periph(0x40022800, GpiokId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpioaId {}
pub type Gpioa = Periph<GpioaId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiobId {}
pub type Gpiob = Periph<GpiobId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiocId {}
pub type Gpioc = Periph<GpiocId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiodId {}
pub type Gpiod = Periph<GpiodId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpioeId {}
pub type Gpioe = Periph<GpioeId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiofId {}
pub type Gpiof = Periph<GpiofId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiogId {}
pub type Gpiog = Periph<GpiogId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiohId {}
pub type Gpioh = Periph<GpiohId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpioiId {}
pub type Gpioi = Periph<GpioiId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiojId {}
pub type Gpioj = Periph<GpiojId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiokId {}
pub type Gpiok = Periph<GpiokId>;













pub const PA0: Pin<Pa0Id, GpioaId> = Pin { port: GPIOA, index: 0, id: Pa0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa0Id {}
pub type Pa0 = Pin<Pa0Id, GpioaId>;
impl AltFn<super::sig::Adc1In0> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In0> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc3In0> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch1> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Etr> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim5Ch1> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Etr> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Cts> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Uart4Tx> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EthMiiCrs> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA1: Pin<Pa1Id, GpioaId> = Pin { port: GPIOA, index: 1, id: Pa1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa1Id {}
pub type Pa1 = Pin<Pa1Id, GpioaId>;
impl AltFn<super::sig::Adc1In1> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In1> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc3In1> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch2> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim5Ch2> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart2Rts> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Uart4Rx> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EthMiiRxClk> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiRefClk> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA2: Pin<Pa2Id, GpioaId> = Pin { port: GPIOA, index: 2, id: Pa2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa2Id {}
pub type Pa2 = Pin<Pa2Id, GpioaId>;
impl AltFn<super::sig::Adc1In2> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In2> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc3In2> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch3> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim5Ch3> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim9Ch1> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Tx> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::EthMdio> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA3: Pin<Pa3Id, GpioaId> = Pin { port: GPIOA, index: 3, id: Pa3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa3Id {}
pub type Pa3 = Pin<Pa3Id, GpioaId>;
impl AltFn<super::sig::Adc1In3> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In3> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc3In3> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch4> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim5Ch4> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim9Ch2> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Rx> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::OtgHsUlpiD0> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiCol> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::LcdB5> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA4: Pin<Pa4Id, GpioaId> = Pin { port: GPIOA, index: 4, id: Pa4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa4Id {}
pub type Pa4 = Pin<Pa4Id, GpioaId>;
impl AltFn<super::sig::Adc1In4> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In4> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Spi1Nss> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi3Nss> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ws> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Ck> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::OtgHsSof> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiHsync> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdVsync> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA5: Pin<Pa5Id, GpioaId> = Pin { port: GPIOA, index: 5, id: Pa5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa5Id {}
pub type Pa5 = Pin<Pa5Id, GpioaId>;
impl AltFn<super::sig::Adc1In5> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In5> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch1> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Etr> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim8Ch1n> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Sck> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::OtgHsUlpiCk> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA6: Pin<Pa6Id, GpioaId> = Pin { port: GPIOA, index: 6, id: Pa6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa6Id {}
pub type Pa6 = Pin<Pa6Id, GpioaId>;
impl AltFn<super::sig::Adc1In6> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In6> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim1Bkin> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch1> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Bkin> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Miso> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim13Ch1> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::DcmiPxclk> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdG2> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA7: Pin<Pa7Id, GpioaId> = Pin { port: GPIOA, index: 7, id: Pa7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa7Id {}
pub type Pa7 = Pin<Pa7Id, GpioaId>;
impl AltFn<super::sig::Adc1In7> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In7> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim1Ch1n> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch2> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch1n> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Mosi> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim14Ch1> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::EthMiiRxDv> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiCrsDv> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA8: Pin<Pa8Id, GpioaId> = Pin { port: GPIOA, index: 8, id: Pa8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa8Id {}
pub type Pa8 = Pin<Pa8Id, GpioaId>;
impl AltFn<super::sig::Mco1> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim1Ch1> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c3Scl> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Ck> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::OtgFsSof> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::LcdR6> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA9: Pin<Pa9Id, GpioaId> = Pin { port: GPIOA, index: 9, id: Pa9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa9Id {}
pub type Pa9 = Pin<Pa9Id, GpioaId>;
impl AltFn<super::sig::Tim1Ch2> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c3Smba> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Tx> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::DcmiD0> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA10: Pin<Pa10Id, GpioaId> = Pin { port: GPIOA, index: 10, id: Pa10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa10Id {}
pub type Pa10 = Pin<Pa10Id, GpioaId>;
impl AltFn<super::sig::Tim1Ch3> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart1Rx> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::OtgFsId> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::DcmiD1> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA11: Pin<Pa11Id, GpioaId> = Pin { port: GPIOA, index: 11, id: Pa11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa11Id {}
pub type Pa11 = Pin<Pa11Id, GpioaId>;
impl AltFn<super::sig::Tim1Ch4> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart1Cts> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Can1Rx> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgFsDm> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::LcdR4> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA12: Pin<Pa12Id, GpioaId> = Pin { port: GPIOA, index: 12, id: Pa12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa12Id {}
pub type Pa12 = Pin<Pa12Id, GpioaId>;
impl AltFn<super::sig::Tim1Etr> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart1Rts> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Can1Tx> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgFsDp> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::LcdR5> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA13: Pin<Pa13Id, GpioaId> = Pin { port: GPIOA, index: 13, id: Pa13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa13Id {}
pub type Pa13 = Pin<Pa13Id, GpioaId>;
impl AltFn<super::sig::Jtms> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Swdio> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Eventout> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA14: Pin<Pa14Id, GpioaId> = Pin { port: GPIOA, index: 14, id: Pa14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa14Id {}
pub type Pa14 = Pin<Pa14Id, GpioaId>;
impl AltFn<super::sig::Jtck> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Swclk> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Eventout> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA15: Pin<Pa15Id, GpioaId> = Pin { port: GPIOA, index: 15, id: Pa15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa15Id {}
pub type Pa15 = Pin<Pa15Id, GpioaId>;
impl AltFn<super::sig::Jtdi> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch1> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Etr> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Nss> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi3Nss> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ws> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Eventout> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB0: Pin<Pb0Id, GpiobId> = Pin { port: GPIOB, index: 0, id: Pb0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb0Id {}
pub type Pb0 = Pin<Pb0Id, GpiobId>;
impl AltFn<super::sig::Adc1In8> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In8> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim1Ch2n> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch3> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch2n> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::LcdR3> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsUlpiD1> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiRxd2> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB1: Pin<Pb1Id, GpiobId> = Pin { port: GPIOB, index: 1, id: Pb1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb1Id {}
pub type Pb1 = Pin<Pb1Id, GpiobId>;
impl AltFn<super::sig::Adc1In9> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In9> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim1Ch3n> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch4> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch3n> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::LcdR6> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsUlpiD2> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiRxd3> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB2: Pin<Pb2Id, GpiobId> = Pin { port: GPIOB, index: 2, id: Pb2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb2Id {}
pub type Pb2 = Pin<Pb2Id, GpiobId>;
impl AltFn<super::sig::Eventout> for Pb2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB3: Pin<Pb3Id, GpiobId> = Pin { port: GPIOB, index: 3, id: Pb3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb3Id {}
pub type Pb3 = Pin<Pb3Id, GpiobId>;
impl AltFn<super::sig::Jtdo> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Traceswo> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch2> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sck> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi3Sck> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Is2cCk> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Eventout> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB4: Pin<Pb4Id, GpiobId> = Pin { port: GPIOB, index: 4, id: Pb4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb4Id {}
pub type Pb4 = Pin<Pb4Id, GpiobId>;
impl AltFn<super::sig::Njtrst> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim3Ch1> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Spi1Miso> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi3Miso> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3extSd> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB5: Pin<Pb5Id, GpiobId> = Pin { port: GPIOB, index: 5, id: Pb5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb5Id {}
pub type Pb5 = Pin<Pb5Id, GpiobId>;
impl AltFn<super::sig::Tim3Ch2> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c1Smba> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi1Mosi> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi3Mosi> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Sd> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Can2Rx> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsUlpiD7> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthPpsOut> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcSdcke1> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD10> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB6: Pin<Pb6Id, GpiobId> = Pin { port: GPIOB, index: 6, id: Pb6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb6Id {}
pub type Pb6 = Pin<Pb6Id, GpiobId>;
impl AltFn<super::sig::Tim4Ch1> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c1Scl> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Tx> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Can2Tx> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FmcSdne1> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD5> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB7: Pin<Pb7Id, GpiobId> = Pin { port: GPIOB, index: 7, id: Pb7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb7Id {}
pub type Pb7 = Pin<Pb7Id, GpiobId>;
impl AltFn<super::sig::Tim4Ch2> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c1Sda> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Rx> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcNl> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiVsync> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB8: Pin<Pb8Id, GpiobId> = Pin { port: GPIOB, index: 8, id: Pb8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb8Id {}
pub type Pb8 = Pin<Pb8Id, GpiobId>;
impl AltFn<super::sig::Tim4Ch3> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim10Ch1> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Scl> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Can1Rx> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::EthMiiTxd3> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::SdioD4> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD6> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdB6> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB9: Pin<Pb9Id, GpiobId> = Pin { port: GPIOB, index: 9, id: Pb9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb9Id {}
pub type Pb9 = Pin<Pb9Id, GpiobId>;
impl AltFn<super::sig::Tim4Ch4> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim11Ch1> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Sda> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Nss> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ws> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Can1Tx> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::SdioD5> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD7> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdB7> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB10: Pin<Pb10Id, GpiobId> = Pin { port: GPIOB, index: 10, id: Pb10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb10Id {}
pub type Pb10 = Pin<Pb10Id, GpiobId>;
impl AltFn<super::sig::Tim2Ch3> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c2Scl> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Sck> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart3Tx> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::OtgHsUlpiD3> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiRxEr> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::LcdG4> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB11: Pin<Pb11Id, GpiobId> = Pin { port: GPIOB, index: 11, id: Pb11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb11Id {}
pub type Pb11 = Pin<Pb11Id, GpiobId>;
impl AltFn<super::sig::Sim2Ch4> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c2Sda> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart3Rx> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::OtgHsUlpiD4> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiTxEn> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxEn> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::LcdG5> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB12: Pin<Pb12Id, GpiobId> = Pin { port: GPIOB, index: 12, id: Pb12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb12Id {}
pub type Pb12 = Pin<Pb12Id, GpiobId>;
impl AltFn<super::sig::Tim1Bkin> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c2Smba> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Nss> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ws> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart3Ck> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Can2Rx> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsUlpiD5> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiTxd0> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxd0> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::OtgHsId> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB13: Pin<Pb13Id, GpiobId> = Pin { port: GPIOB, index: 13, id: Pb13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb13Id {}
pub type Pb13 = Pin<Pb13Id, GpiobId>;
impl AltFn<super::sig::Tim1Ch1n> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sck> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart3Cts> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Can2Tx> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsUlpiD6> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiTxd1> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxd1> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB14: Pin<Pb14Id, GpiobId> = Pin { port: GPIOB, index: 14, id: Pb14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb14Id {}
pub type Pb14 = Pin<Pb14Id, GpiobId>;
impl AltFn<super::sig::Tim2Ch2n> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim8Ch2n> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Miso> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2extSd> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Rts> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim12Ch1> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsDm> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB15: Pin<Pb15Id, GpiobId> = Pin { port: GPIOB, index: 15, id: Pb15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb15Id {}
pub type Pb15 = Pin<Pb15Id, GpiobId>;
impl AltFn<super::sig::RtcRefin> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim1Ch3n> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim8Ch3n> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Mosi> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Sd> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim12Ch2> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::OtgHsDp> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC0: Pin<Pc0Id, GpiocId> = Pin { port: GPIOC, index: 0, id: Pc0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc0Id {}
pub type Pc0 = Pin<Pc0Id, GpiocId>;
impl AltFn<super::sig::Adc1In10> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In10> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc3In10> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::OtgHsUlpiStp> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::FmcSdnwe> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC1: Pin<Pc1Id, GpiocId> = Pin { port: GPIOC, index: 1, id: Pc1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc1Id {}
pub type Pc1 = Pin<Pc1Id, GpiocId>;
impl AltFn<super::sig::Adc1In11> for Pc1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In11> for Pc1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc3In11> for Pc1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::EthMdc> for Pc1Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pc1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC2: Pin<Pc2Id, GpiocId> = Pin { port: GPIOC, index: 2, id: Pc2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc2Id {}
pub type Pc2 = Pin<Pc2Id, GpiocId>;
impl AltFn<super::sig::Adc1In12> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In12> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc3In12> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::OtgHsUlpiDir> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiTxd2> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcSdne0> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC3: Pin<Pc3Id, GpiocId> = Pin { port: GPIOC, index: 3, id: Pc3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc3Id {}
pub type Pc3 = Pin<Pc3Id, GpiocId>;
impl AltFn<super::sig::Adc1In13> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In13> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc3In13> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::OtgHsUlpiNxt> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EthMiiTxClk> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcSdcke0> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC4: Pin<Pc4Id, GpiocId> = Pin { port: GPIOC, index: 4, id: Pc4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc4Id {}
pub type Pc4 = Pin<Pc4Id, GpiocId>;
impl AltFn<super::sig::Adc1In14> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In14> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::EthMiiRxd0> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiRxd0> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC5: Pin<Pc5Id, GpiocId> = Pin { port: GPIOC, index: 5, id: Pc5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc5Id {}
pub type Pc5 = Pin<Pc5Id, GpiocId>;
impl AltFn<super::sig::Adc1In15> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc2In15> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::EthMiiRxd1> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiRxd1> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC6: Pin<Pc6Id, GpiocId> = Pin { port: GPIOC, index: 6, id: Pc6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc6Id {}
pub type Pc6 = Pin<Pc6Id, GpiocId>;
impl AltFn<super::sig::Tim3Ch1> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch1> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s2Mck> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart6Tx> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::SdioD6> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD0> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdHsync> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC7: Pin<Pc7Id, GpiocId> = Pin { port: GPIOC, index: 7, id: Pc7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc7Id {}
pub type Pc7 = Pin<Pc7Id, GpiocId>;
impl AltFn<super::sig::Tim3Ch2> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch2> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s3Mck> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart6Rx> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::SdioD7> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD1> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdG6> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC8: Pin<Pc8Id, GpiocId> = Pin { port: GPIOC, index: 8, id: Pc8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc8Id {}
pub type Pc8 = Pin<Pc8Id, GpiocId>;
impl AltFn<super::sig::Tim3Ch3> for Pc8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch3> for Pc8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart6Ck> for Pc8Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::SdioD0> for Pc8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD2> for Pc8Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pc8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC9: Pin<Pc9Id, GpiocId> = Pin { port: GPIOC, index: 9, id: Pc9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc9Id {}
pub type Pc9 = Pin<Pc9Id, GpiocId>;
impl AltFn<super::sig::Mco2> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim3Ch4> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch4> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c3Sda> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2sCkin> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::SdioD1> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD3> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC10: Pin<Pc10Id, GpiocId> = Pin { port: GPIOC, index: 10, id: Pc10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc10Id {}
pub type Pc10 = Pin<Pc10Id, GpiocId>;
impl AltFn<super::sig::Spi3Sck> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ck> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Tx> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::SdioD2> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD8> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdR2> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC11: Pin<Pc11Id, GpiocId> = Pin { port: GPIOC, index: 11, id: Pc11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc11Id {}
pub type Pc11 = Pin<Pc11Id, GpiocId>;
impl AltFn<super::sig::Spi3Miso> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Rx> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::SdioD3> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD4> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC12: Pin<Pc12Id, GpiocId> = Pin { port: GPIOC, index: 12, id: Pc12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc12Id {}
pub type Pc12 = Pin<Pc12Id, GpiocId>;
impl AltFn<super::sig::Spi3Mosi> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Sd> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Ck> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::SdioCk> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD9> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC13: Pin<Pc13Id, GpiocId> = Pin { port: GPIOC, index: 13, id: Pc13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc13Id {}
pub type Pc13 = Pin<Pc13Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC14: Pin<Pc14Id, GpiocId> = Pin { port: GPIOC, index: 14, id: Pc14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc14Id {}
pub type Pc14 = Pin<Pc14Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC15: Pin<Pc15Id, GpiocId> = Pin { port: GPIOC, index: 15, id: Pc15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc15Id {}
pub type Pc15 = Pin<Pc15Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD0: Pin<Pd0Id, GpiodId> = Pin { port: GPIOD, index: 0, id: Pd0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd0Id {}
pub type Pd0 = Pin<Pd0Id, GpiodId>;
impl AltFn<super::sig::Can1Rx> for Pd0Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FmcD2> for Pd0Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD1: Pin<Pd1Id, GpiodId> = Pin { port: GPIOD, index: 1, id: Pd1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd1Id {}
pub type Pd1 = Pin<Pd1Id, GpiodId>;
impl AltFn<super::sig::Can1Tx> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FmcD3> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD2: Pin<Pd2Id, GpiodId> = Pin { port: GPIOD, index: 2, id: Pd2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd2Id {}
pub type Pd2 = Pin<Pd2Id, GpiodId>;
impl AltFn<super::sig::Tim3Etr> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5Rx> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::SdioCmd> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD11> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD3: Pin<Pd3Id, GpiodId> = Pin { port: GPIOD, index: 3, id: Pd3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd3Id {}
pub type Pd3 = Pin<Pd3Id, GpiodId>;
impl AltFn<super::sig::Spi2Sck> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart2Cts> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcClk> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD5> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdG7> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD4: Pin<Pd4Id, GpiodId> = Pin { port: GPIOD, index: 4, id: Pd4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd4Id {}
pub type Pd4 = Pin<Pd4Id, GpiodId>;
impl AltFn<super::sig::Usart2Rts> for Pd4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcNoe> for Pd4Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD5: Pin<Pd5Id, GpiodId> = Pin { port: GPIOD, index: 5, id: Pd5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd5Id {}
pub type Pd5 = Pin<Pd5Id, GpiodId>;
impl AltFn<super::sig::Usart2Tx> for Pd5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcNwe> for Pd5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD6: Pin<Pd6Id, GpiodId> = Pin { port: GPIOD, index: 6, id: Pd6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd6Id {}
pub type Pd6 = Pin<Pd6Id, GpiodId>;
impl AltFn<super::sig::Spi3Mosi> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s3Sd> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Sai1SdA> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Rx> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcNwait> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD10> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdB2> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD7: Pin<Pd7Id, GpiodId> = Pin { port: GPIOD, index: 7, id: Pd7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd7Id {}
pub type Pd7 = Pin<Pd7Id, GpiodId>;
impl AltFn<super::sig::Usart2Ck> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcNe1> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::FmcNce2> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD8: Pin<Pd8Id, GpiodId> = Pin { port: GPIOD, index: 8, id: Pd8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd8Id {}
pub type Pd8 = Pin<Pd8Id, GpiodId>;
impl AltFn<super::sig::Usart3Tx> for Pd8Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcD13> for Pd8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD9: Pin<Pd9Id, GpiodId> = Pin { port: GPIOD, index: 9, id: Pd9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd9Id {}
pub type Pd9 = Pin<Pd9Id, GpiodId>;
impl AltFn<super::sig::Usart3Rx> for Pd9Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcD14> for Pd9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD10: Pin<Pd10Id, GpiodId> = Pin { port: GPIOD, index: 10, id: Pd10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd10Id {}
pub type Pd10 = Pin<Pd10Id, GpiodId>;
impl AltFn<super::sig::Usart3Ck> for Pd10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcD15> for Pd10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD11: Pin<Pd11Id, GpiodId> = Pin { port: GPIOD, index: 11, id: Pd11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd11Id {}
pub type Pd11 = Pin<Pd11Id, GpiodId>;
impl AltFn<super::sig::Usart3Cts> for Pd11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcA16> for Pd11Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD12: Pin<Pd12Id, GpiodId> = Pin { port: GPIOD, index: 12, id: Pd12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd12Id {}
pub type Pd12 = Pin<Pd12Id, GpiodId>;
impl AltFn<super::sig::Tim4Ch1> for Pd12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart3Rts> for Pd12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcA17> for Pd12Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD13: Pin<Pd13Id, GpiodId> = Pin { port: GPIOD, index: 13, id: Pd13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd13Id {}
pub type Pd13 = Pin<Pd13Id, GpiodId>;
impl AltFn<super::sig::Tim4Ch2> for Pd13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA18> for Pd13Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD14: Pin<Pd14Id, GpiodId> = Pin { port: GPIOD, index: 14, id: Pd14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd14Id {}
pub type Pd14 = Pin<Pd14Id, GpiodId>;
impl AltFn<super::sig::Tim4Ch3> for Pd14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcD0> for Pd14Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD15: Pin<Pd15Id, GpiodId> = Pin { port: GPIOD, index: 15, id: Pd15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd15Id {}
pub type Pd15 = Pin<Pd15Id, GpiodId>;
impl AltFn<super::sig::Tim4Ch4> for Pd15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcD1> for Pd15Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pd15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE0: Pin<Pe0Id, GpioeId> = Pin { port: GPIOE, index: 0, id: Pe0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe0Id {}
pub type Pe0 = Pin<Pe0Id, GpioeId>;
impl AltFn<super::sig::Tim4Etr> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart8Rx> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::FmcNbl0> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE1: Pin<Pe1Id, GpioeId> = Pin { port: GPIOE, index: 1, id: Pe1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe1Id {}
pub type Pe1 = Pin<Pe1Id, GpioeId>;
impl AltFn<super::sig::Uart8Tx> for Pe1Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::FmcNbl1> for Pe1Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE2: Pin<Pe2Id, GpioeId> = Pin { port: GPIOE, index: 2, id: Pe2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe2Id {}
pub type Pe2 = Pin<Pe2Id, GpioeId>;
impl AltFn<super::sig::Traceclk> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Spi4Sck> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Sai1MclkA> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::EthMiiTxd3> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcA23> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD2> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE3: Pin<Pe3Id, GpioeId> = Pin { port: GPIOE, index: 3, id: Pe3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe3Id {}
pub type Pe3 = Pin<Pe3Id, GpioeId>;
impl AltFn<super::sig::Traced0> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Sai1SdB> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcA19> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD3> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE4: Pin<Pe4Id, GpioeId> = Pin { port: GPIOE, index: 4, id: Pe4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe4Id {}
pub type Pe4 = Pin<Pe4Id, GpioeId>;
impl AltFn<super::sig::Traced1> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Spi4Nss> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Sai1FsA> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcA20> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE5: Pin<Pe5Id, GpioeId> = Pin { port: GPIOE, index: 5, id: Pe5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe5Id {}
pub type Pe5 = Pin<Pe5Id, GpioeId>;
impl AltFn<super::sig::Traced2> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim9Ch1> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi4Miso> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Sai1SckA> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcA21> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE6: Pin<Pe6Id, GpioeId> = Pin { port: GPIOE, index: 6, id: Pe6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe6Id {}
pub type Pe6 = Pin<Pe6Id, GpioeId>;
impl AltFn<super::sig::Traced3> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim9Ch2> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi4Mosi> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Sai1SdA> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcA22> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD4> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE7: Pin<Pe7Id, GpioeId> = Pin { port: GPIOE, index: 7, id: Pe7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe7Id {}
pub type Pe7 = Pin<Pe7Id, GpioeId>;
impl AltFn<super::sig::Tim1Etr> for Pe7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcD4> for Pe7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD6> for Pe7Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pe7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE8: Pin<Pe8Id, GpioeId> = Pin { port: GPIOE, index: 8, id: Pe8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe8Id {}
pub type Pe8 = Pin<Pe8Id, GpioeId>;
impl AltFn<super::sig::Tim1Ch1n> for Pe8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcD5> for Pe8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD7> for Pe8Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pe8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE9: Pin<Pe9Id, GpioeId> = Pin { port: GPIOE, index: 9, id: Pe9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe9Id {}
pub type Pe9 = Pin<Pe9Id, GpioeId>;
impl AltFn<super::sig::Tim1Ch1> for Pe9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcD6> for Pe9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE10: Pin<Pe10Id, GpioeId> = Pin { port: GPIOE, index: 10, id: Pe10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe10Id {}
pub type Pe10 = Pin<Pe10Id, GpioeId>;
impl AltFn<super::sig::Tim1Ch2n> for Pe10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcD7> for Pe10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pe10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE11: Pin<Pe11Id, GpioeId> = Pin { port: GPIOE, index: 11, id: Pe11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe11Id {}
pub type Pe11 = Pin<Pe11Id, GpioeId>;
impl AltFn<super::sig::Tim1Ch2> for Pe11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi4Nss> for Pe11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcD8> for Pe11Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::LcdG3> for Pe11Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pe11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE12: Pin<Pe12Id, GpioeId> = Pin { port: GPIOE, index: 12, id: Pe12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe12Id {}
pub type Pe12 = Pin<Pe12Id, GpioeId>;
impl AltFn<super::sig::Tim1Ch3n> for Pe12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi4Sck> for Pe12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcD9> for Pe12Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::LcdB4> for Pe12Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pe12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE13: Pin<Pe13Id, GpioeId> = Pin { port: GPIOE, index: 13, id: Pe13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe13Id {}
pub type Pe13 = Pin<Pe13Id, GpioeId>;
impl AltFn<super::sig::Tim1Ch3> for Pe13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi4Miso> for Pe13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcD10> for Pe13Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::LcdDe> for Pe13Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pe13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE14: Pin<Pe14Id, GpioeId> = Pin { port: GPIOE, index: 14, id: Pe14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe14Id {}
pub type Pe14 = Pin<Pe14Id, GpioeId>;
impl AltFn<super::sig::Tim1Ch4> for Pe14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi4Mosi> for Pe14Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcD11> for Pe14Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::LcdClk> for Pe14Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pe14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE15: Pin<Pe15Id, GpioeId> = Pin { port: GPIOE, index: 15, id: Pe15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe15Id {}
pub type Pe15 = Pin<Pe15Id, GpioeId>;
impl AltFn<super::sig::Tim1Bkin> for Pe15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcD12> for Pe15Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::LcdR7> for Pe15Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pe15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF0: Pin<Pf0Id, GpiofId> = Pin { port: GPIOF, index: 0, id: Pf0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf0Id {}
pub type Pf0 = Pin<Pf0Id, GpiofId>;
impl AltFn<super::sig::I2c2Sda> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FmcA0> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF1: Pin<Pf1Id, GpiofId> = Pin { port: GPIOF, index: 1, id: Pf1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf1Id {}
pub type Pf1 = Pin<Pf1Id, GpiofId>;
impl AltFn<super::sig::I2c2Scl> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FmcA1> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF2: Pin<Pf2Id, GpiofId> = Pin { port: GPIOF, index: 2, id: Pf2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf2Id {}
pub type Pf2 = Pin<Pf2Id, GpiofId>;
impl AltFn<super::sig::I2c2Smba> for Pf2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FmcA2> for Pf2Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF3: Pin<Pf3Id, GpiofId> = Pin { port: GPIOF, index: 3, id: Pf3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf3Id {}
pub type Pf3 = Pin<Pf3Id, GpiofId>;
impl AltFn<super::sig::Adc3In9> for Pf3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::FmcA3> for Pf3Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF4: Pin<Pf4Id, GpiofId> = Pin { port: GPIOF, index: 4, id: Pf4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf4Id {}
pub type Pf4 = Pin<Pf4Id, GpiofId>;
impl AltFn<super::sig::Adc3In14> for Pf4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::FmcA4> for Pf4Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF5: Pin<Pf5Id, GpiofId> = Pin { port: GPIOF, index: 5, id: Pf5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf5Id {}
pub type Pf5 = Pin<Pf5Id, GpiofId>;
impl AltFn<super::sig::Adc3In15> for Pf5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::FmcA5> for Pf5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF6: Pin<Pf6Id, GpiofId> = Pin { port: GPIOF, index: 6, id: Pf6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf6Id {}
pub type Pf6 = Pin<Pf6Id, GpiofId>;
impl AltFn<super::sig::Adc3In4> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim10Ch1> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi5Nss> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Sai1SdB> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Uart7Rx> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::FmcNiord> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF7: Pin<Pf7Id, GpiofId> = Pin { port: GPIOF, index: 7, id: Pf7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf7Id {}
pub type Pf7 = Pin<Pf7Id, GpiofId>;
impl AltFn<super::sig::Adc3In5> for Pf7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim11Ch1> for Pf7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi5Sck> for Pf7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Sai1MclkB> for Pf7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Uart7Tx> for Pf7Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::FmcNreg> for Pf7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF8: Pin<Pf8Id, GpiofId> = Pin { port: GPIOF, index: 8, id: Pf8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf8Id {}
pub type Pf8 = Pin<Pf8Id, GpiofId>;
impl AltFn<super::sig::Adc3In6> for Pf8Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Spi5Miso> for Pf8Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::SaiSckB> for Pf8Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Tim13Ch1> for Pf8Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FmcNiowr> for Pf8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF9: Pin<Pf9Id, GpiofId> = Pin { port: GPIOF, index: 9, id: Pf9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf9Id {}
pub type Pf9 = Pin<Pf9Id, GpiofId>;
impl AltFn<super::sig::Adc3In7> for Pf9Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Spi5Mosi> for Pf9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::SaiFsB> for Pf9Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Tim14Ch1> for Pf9Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FmcCd> for Pf9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF10: Pin<Pf10Id, GpiofId> = Pin { port: GPIOF, index: 10, id: Pf10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf10Id {}
pub type Pf10 = Pin<Pf10Id, GpiofId>;
impl AltFn<super::sig::Adc3In8> for Pf10Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::FmcIntr> for Pf10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD11> for Pf10Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdDe> for Pf10Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pf10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF11: Pin<Pf11Id, GpiofId> = Pin { port: GPIOF, index: 11, id: Pf11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf11Id {}
pub type Pf11 = Pin<Pf11Id, GpiofId>;
impl AltFn<super::sig::Spi5Mosi> for Pf11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcSdnras> for Pf11Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD12> for Pf11Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pf11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF12: Pin<Pf12Id, GpiofId> = Pin { port: GPIOF, index: 12, id: Pf12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf12Id {}
pub type Pf12 = Pin<Pf12Id, GpiofId>;
impl AltFn<super::sig::FmcA6> for Pf12Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF13: Pin<Pf13Id, GpiofId> = Pin { port: GPIOF, index: 13, id: Pf13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf13Id {}
pub type Pf13 = Pin<Pf13Id, GpiofId>;
impl AltFn<super::sig::FmcA7> for Pf13Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF14: Pin<Pf14Id, GpiofId> = Pin { port: GPIOF, index: 14, id: Pf14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf14Id {}
pub type Pf14 = Pin<Pf14Id, GpiofId>;
impl AltFn<super::sig::FmcA8> for Pf14Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF15: Pin<Pf15Id, GpiofId> = Pin { port: GPIOF, index: 15, id: Pf15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf15Id {}
pub type Pf15 = Pin<Pf15Id, GpiofId>;
impl AltFn<super::sig::FmcA9> for Pf15Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pf15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG0: Pin<Pg0Id, GpiogId> = Pin { port: GPIOG, index: 0, id: Pg0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg0Id {}
pub type Pg0 = Pin<Pg0Id, GpiogId>;
impl AltFn<super::sig::FmcA10> for Pg0Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG1: Pin<Pg1Id, GpiogId> = Pin { port: GPIOG, index: 1, id: Pg1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg1Id {}
pub type Pg1 = Pin<Pg1Id, GpiogId>;
impl AltFn<super::sig::FmcA11> for Pg1Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG2: Pin<Pg2Id, GpiogId> = Pin { port: GPIOG, index: 2, id: Pg2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg2Id {}
pub type Pg2 = Pin<Pg2Id, GpiogId>;
impl AltFn<super::sig::FmcA12> for Pg2Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG3: Pin<Pg3Id, GpiogId> = Pin { port: GPIOG, index: 3, id: Pg3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg3Id {}
pub type Pg3 = Pin<Pg3Id, GpiogId>;
impl AltFn<super::sig::FmcA13> for Pg3Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG4: Pin<Pg4Id, GpiogId> = Pin { port: GPIOG, index: 4, id: Pg4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg4Id {}
pub type Pg4 = Pin<Pg4Id, GpiogId>;
impl AltFn<super::sig::FmcA14> for Pg4Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::FmcBa0> for Pg4Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG5: Pin<Pg5Id, GpiogId> = Pin { port: GPIOG, index: 5, id: Pg5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg5Id {}
pub type Pg5 = Pin<Pg5Id, GpiogId>;
impl AltFn<super::sig::FmcA15> for Pg5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::FmcBa1> for Pg5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG6: Pin<Pg6Id, GpiogId> = Pin { port: GPIOG, index: 6, id: Pg6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg6Id {}
pub type Pg6 = Pin<Pg6Id, GpiogId>;
impl AltFn<super::sig::FmcInt2> for Pg6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD12> for Pg6Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdR7> for Pg6Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pg6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG7: Pin<Pg7Id, GpiogId> = Pin { port: GPIOG, index: 7, id: Pg7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg7Id {}
pub type Pg7 = Pin<Pg7Id, GpiogId>;
impl AltFn<super::sig::Usart6Ck> for Pg7Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::FmcInt3> for Pg7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD13> for Pg7Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdClk> for Pg7Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pg7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG8: Pin<Pg8Id, GpiogId> = Pin { port: GPIOG, index: 8, id: Pg8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg8Id {}
pub type Pg8 = Pin<Pg8Id, GpiogId>;
impl AltFn<super::sig::Spi6Nss> for Pg8Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart6Rts> for Pg8Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EthPsOut> for Pg8Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcSdclk> for Pg8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG9: Pin<Pg9Id, GpiogId> = Pin { port: GPIOG, index: 9, id: Pg9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg9Id {}
pub type Pg9 = Pin<Pg9Id, GpiogId>;
impl AltFn<super::sig::Usart6Rx> for Pg9Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::FmcNe2> for Pg9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::FmcNce3> for Pg9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiVsync> for Pg9Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pg9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG10: Pin<Pg10Id, GpiogId> = Pin { port: GPIOG, index: 10, id: Pg10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg10Id {}
pub type Pg10 = Pin<Pg10Id, GpiogId>;
impl AltFn<super::sig::LcdG3> for Pg10Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FmcNce41> for Pg10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::FmcNe3> for Pg10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD2> for Pg10Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdB2> for Pg10Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pg10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG11: Pin<Pg11Id, GpiogId> = Pin { port: GPIOG, index: 11, id: Pg11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg11Id {}
pub type Pg11 = Pin<Pg11Id, GpiogId>;
impl AltFn<super::sig::Spi6Miso> for Pg11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart6Rts> for Pg11Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EthMiiTxEn> for Pg11Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxEn> for Pg11Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcNce42> for Pg11Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD3> for Pg11Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdB3> for Pg11Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pg11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG12: Pin<Pg12Id, GpiogId> = Pin { port: GPIOG, index: 12, id: Pg12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg12Id {}
pub type Pg12 = Pin<Pg12Id, GpiogId>;
impl AltFn<super::sig::Spi6Sck> for Pg12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart6Cts> for Pg12Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::LcdB4> for Pg12Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FmcNe4> for Pg12Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::LcdB1> for Pg12Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pg12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG13: Pin<Pg13Id, GpiogId> = Pin { port: GPIOG, index: 13, id: Pg13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg13Id {}
pub type Pg13 = Pin<Pg13Id, GpiogId>;
impl AltFn<super::sig::Spi6Mosi> for Pg13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart6Tx> for Pg13Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EthMiiTxd0> for Pg13Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxd0> for Pg13Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcA24> for Pg13Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG14: Pin<Pg14Id, GpiogId> = Pin { port: GPIOG, index: 14, id: Pg14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg14Id {}
pub type Pg14 = Pin<Pg14Id, GpiogId>;
impl AltFn<super::sig::Usart6Cts> for Pg14Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EthMiiTxd1> for Pg14Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EthRmiiTxd1> for Pg14Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcA25> for Pg14Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pg14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG15: Pin<Pg15Id, GpiogId> = Pin { port: GPIOG, index: 15, id: Pg15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg15Id {}
pub type Pg15 = Pin<Pg15Id, GpiogId>;
impl AltFn<super::sig::FmcSndcas> for Pg15Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD13> for Pg15Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pg15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH0: Pin<Ph0Id, GpiohId> = Pin { port: GPIOH, index: 0, id: Ph0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph0Id {}
pub type Ph0 = Pin<Ph0Id, GpiohId>;
impl AltFn<super::sig::Eventout> for Ph0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH1: Pin<Ph1Id, GpiohId> = Pin { port: GPIOH, index: 1, id: Ph1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph1Id {}
pub type Ph1 = Pin<Ph1Id, GpiohId>;
impl AltFn<super::sig::Eventout> for Ph1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH2: Pin<Ph2Id, GpiohId> = Pin { port: GPIOH, index: 2, id: Ph2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph2Id {}
pub type Ph2 = Pin<Ph2Id, GpiohId>;
impl AltFn<super::sig::EthMiiCrs> for Ph2Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcSdcke0> for Ph2Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::LcdR0> for Ph2Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Ph2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH3: Pin<Ph3Id, GpiohId> = Pin { port: GPIOH, index: 3, id: Ph3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph3Id {}
pub type Ph3 = Pin<Ph3Id, GpiohId>;
impl AltFn<super::sig::EthMiiCol> for Ph3Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcSdne0> for Ph3Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::LcdR1> for Ph3Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Ph3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH4: Pin<Ph4Id, GpiohId> = Pin { port: GPIOH, index: 4, id: Ph4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph4Id {}
pub type Ph4 = Pin<Ph4Id, GpiohId>;
impl AltFn<super::sig::I2c2Scl> for Ph4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::OtgHsUlpiNxt> for Ph4Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Ph4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH5: Pin<Ph5Id, GpiohId> = Pin { port: GPIOH, index: 5, id: Ph5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph5Id {}
pub type Ph5 = Pin<Ph5Id, GpiohId>;
impl AltFn<super::sig::I2c2Sda> for Ph5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi5Nss> for Ph5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcSdnwe> for Ph5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Ph5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH6: Pin<Ph6Id, GpiohId> = Pin { port: GPIOH, index: 6, id: Ph6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph6Id {}
pub type Ph6 = Pin<Ph6Id, GpiohId>;
impl AltFn<super::sig::I2c2Smba> for Ph6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi5Sck> for Ph6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim12Ch1> for Ph6Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FmcSdne1> for Ph6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD8> for Ph6Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Ph6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH7: Pin<Ph7Id, GpiohId> = Pin { port: GPIOH, index: 7, id: Ph7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph7Id {}
pub type Ph7 = Pin<Ph7Id, GpiohId>;
impl AltFn<super::sig::I2c3Scl> for Ph7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi5Miso> for Ph7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EthMiiRxd3> for Ph7Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcSdcke1> for Ph7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD9> for Ph7Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Ph7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH8: Pin<Ph8Id, GpiohId> = Pin { port: GPIOH, index: 8, id: Ph8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph8Id {}
pub type Ph8 = Pin<Ph8Id, GpiohId>;
impl AltFn<super::sig::I2c3Sda> for Ph8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FmcD16> for Ph8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiHsync> for Ph8Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdR2> for Ph8Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Ph8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH9: Pin<Ph9Id, GpiohId> = Pin { port: GPIOH, index: 9, id: Ph9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph9Id {}
pub type Ph9 = Pin<Ph9Id, GpiohId>;
impl AltFn<super::sig::I2c3Smba> for Ph9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim2Ch2> for Ph9Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FmcD17> for Ph9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD0> for Ph9Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdR3> for Ph9Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Ph9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH10: Pin<Ph10Id, GpiohId> = Pin { port: GPIOH, index: 10, id: Ph10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph10Id {}
pub type Ph10 = Pin<Ph10Id, GpiohId>;
impl AltFn<super::sig::Tim5Ch1> for Ph10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcD18> for Ph10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD1> for Ph10Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdR4> for Ph10Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Ph10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH11: Pin<Ph11Id, GpiohId> = Pin { port: GPIOH, index: 11, id: Ph11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph11Id {}
pub type Ph11 = Pin<Ph11Id, GpiohId>;
impl AltFn<super::sig::Tim5Ch2> for Ph11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcD19> for Ph11Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD2> for Ph11Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdR5> for Ph11Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Ph11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH12: Pin<Ph12Id, GpiohId> = Pin { port: GPIOH, index: 12, id: Ph12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph12Id {}
pub type Ph12 = Pin<Ph12Id, GpiohId>;
impl AltFn<super::sig::Tim5Ch3> for Ph12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcD20> for Ph12Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD3> for Ph12Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdR6> for Ph12Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Ph12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH13: Pin<Ph13Id, GpiohId> = Pin { port: GPIOH, index: 13, id: Ph13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph13Id {}
pub type Ph13 = Pin<Ph13Id, GpiohId>;
impl AltFn<super::sig::Tim8Ch1n> for Ph13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can1Tx> for Ph13Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FmcD21> for Ph13Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::LcdG2> for Ph13Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Ph13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH14: Pin<Ph14Id, GpiohId> = Pin { port: GPIOH, index: 14, id: Ph14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph14Id {}
pub type Ph14 = Pin<Ph14Id, GpiohId>;
impl AltFn<super::sig::Tim8Ch2n> for Ph14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FmcD22> for Ph14Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD4> for Ph14Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdG3> for Ph14Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Ph14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH15: Pin<Ph15Id, GpiohId> = Pin { port: GPIOH, index: 15, id: Ph15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph15Id {}
pub type Ph15 = Pin<Ph15Id, GpiohId>;
impl AltFn<super::sig::Tim8Ch3n> for Ph15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FmcD23> for Ph15Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD11> for Ph15Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdG4> for Ph15Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Ph15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI0: Pin<Pi0Id, GpioiId> = Pin { port: GPIOI, index: 0, id: Pi0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi0Id {}
pub type Pi0 = Pin<Pi0Id, GpioiId>;
impl AltFn<super::sig::Tim5Ch4> for Pi0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Spi2Nss> for Pi0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ws> for Pi0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcD24> for Pi0Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD13> for Pi0Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdG5> for Pi0Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI1: Pin<Pi1Id, GpioiId> = Pin { port: GPIOI, index: 1, id: Pi1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi1Id {}
pub type Pi1 = Pin<Pi1Id, GpioiId>;
impl AltFn<super::sig::Spi2Sck> for Pi1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pi1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcD25> for Pi1Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD8> for Pi1Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdG6> for Pi1Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI2: Pin<Pi2Id, GpioiId> = Pin { port: GPIOI, index: 2, id: Pi2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi2Id {}
pub type Pi2 = Pin<Pi2Id, GpioiId>;
impl AltFn<super::sig::Tim8Ch4> for Pi2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Miso> for Pi2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2extSd> for Pi2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcD26> for Pi2Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD9> for Pi2Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdG7> for Pi2Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI3: Pin<Pi3Id, GpioiId> = Pin { port: GPIOI, index: 3, id: Pi3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi3Id {}
pub type Pi3 = Pin<Pi3Id, GpioiId>;
impl AltFn<super::sig::Tim8Etr> for Pi3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Mosi> for Pi3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Sd> for Pi3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcD27> for Pi3Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD10> for Pi3Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Eventout> for Pi3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI4: Pin<Pi4Id, GpioiId> = Pin { port: GPIOI, index: 4, id: Pi4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi4Id {}
pub type Pi4 = Pin<Pi4Id, GpioiId>;
impl AltFn<super::sig::Tim8Bkin> for Pi4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FmcNbl2> for Pi4Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD5> for Pi4Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdB4> for Pi4Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI5: Pin<Pi5Id, GpioiId> = Pin { port: GPIOI, index: 5, id: Pi5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi5Id {}
pub type Pi5 = Pin<Pi5Id, GpioiId>;
impl AltFn<super::sig::Tim8Ch1> for Pi5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FmcNbl3> for Pi5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmVsync> for Pi5Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdB5> for Pi5Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI6: Pin<Pi6Id, GpioiId> = Pin { port: GPIOI, index: 6, id: Pi6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi6Id {}
pub type Pi6 = Pin<Pi6Id, GpioiId>;
impl AltFn<super::sig::Tim8Ch2> for Pi6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FmcD28> for Pi6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD6> for Pi6Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdB6> for Pi6Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI7: Pin<Pi7Id, GpioiId> = Pin { port: GPIOI, index: 7, id: Pi7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi7Id {}
pub type Pi7 = Pin<Pi7Id, GpioiId>;
impl AltFn<super::sig::Tim8Ch3> for Pi7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FmcD29> for Pi7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::DcmiD7> for Pi7Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::LcdB7> for Pi7Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI8: Pin<Pi8Id, GpioiId> = Pin { port: GPIOI, index: 8, id: Pi8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi8Id {}
pub type Pi8 = Pin<Pi8Id, GpioiId>;
impl AltFn<super::sig::Eventout> for Pi8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI9: Pin<Pi9Id, GpioiId> = Pin { port: GPIOI, index: 9, id: Pi9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi9Id {}
pub type Pi9 = Pin<Pi9Id, GpioiId>;
impl AltFn<super::sig::Can1Rx> for Pi9Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::FmcD30> for Pi9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::LcdVsync> for Pi9Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI10: Pin<Pi10Id, GpioiId> = Pin { port: GPIOI, index: 10, id: Pi10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi10Id {}
pub type Pi10 = Pin<Pi10Id, GpioiId>;
impl AltFn<super::sig::EthMiiRxEr> for Pi10Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::FmcD31> for Pi10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::LcdHsync> for Pi10Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI11: Pin<Pi11Id, GpioiId> = Pin { port: GPIOI, index: 11, id: Pi11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi11Id {}
pub type Pi11 = Pin<Pi11Id, GpioiId>;
impl AltFn<super::sig::OtgHsUlpiDir> for Pi11Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pi11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI12: Pin<Pi12Id, GpioiId> = Pin { port: GPIOI, index: 12, id: Pi12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi12Id {}
pub type Pi12 = Pin<Pi12Id, GpioiId>;
impl AltFn<super::sig::LcdHsync> for Pi12Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI13: Pin<Pi13Id, GpioiId> = Pin { port: GPIOI, index: 13, id: Pi13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi13Id {}
pub type Pi13 = Pin<Pi13Id, GpioiId>;
impl AltFn<super::sig::LcdVsync> for Pi13Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI14: Pin<Pi14Id, GpioiId> = Pin { port: GPIOI, index: 14, id: Pi14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi14Id {}
pub type Pi14 = Pin<Pi14Id, GpioiId>;
impl AltFn<super::sig::LcdClk> for Pi14Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PI15: Pin<Pi15Id, GpioiId> = Pin { port: GPIOI, index: 15, id: Pi15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pi15Id {}
pub type Pi15 = Pin<Pi15Id, GpioiId>;
impl AltFn<super::sig::LcdR0> for Pi15Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pi15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ0: Pin<Pj0Id, GpiojId> = Pin { port: GPIOJ, index: 0, id: Pj0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj0Id {}
pub type Pj0 = Pin<Pj0Id, GpiojId>;
impl AltFn<super::sig::LcdR1> for Pj0Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ1: Pin<Pj1Id, GpiojId> = Pin { port: GPIOJ, index: 1, id: Pj1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj1Id {}
pub type Pj1 = Pin<Pj1Id, GpiojId>;
impl AltFn<super::sig::LcdR2> for Pj1Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ2: Pin<Pj2Id, GpiojId> = Pin { port: GPIOJ, index: 2, id: Pj2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj2Id {}
pub type Pj2 = Pin<Pj2Id, GpiojId>;
impl AltFn<super::sig::LcdR3> for Pj2Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ3: Pin<Pj3Id, GpiojId> = Pin { port: GPIOJ, index: 3, id: Pj3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj3Id {}
pub type Pj3 = Pin<Pj3Id, GpiojId>;
impl AltFn<super::sig::LcdR4> for Pj3Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ4: Pin<Pj4Id, GpiojId> = Pin { port: GPIOJ, index: 4, id: Pj4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj4Id {}
pub type Pj4 = Pin<Pj4Id, GpiojId>;
impl AltFn<super::sig::LcdR5> for Pj4Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ5: Pin<Pj5Id, GpiojId> = Pin { port: GPIOJ, index: 5, id: Pj5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj5Id {}
pub type Pj5 = Pin<Pj5Id, GpiojId>;
impl AltFn<super::sig::LcdR6> for Pj5Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ6: Pin<Pj6Id, GpiojId> = Pin { port: GPIOJ, index: 6, id: Pj6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj6Id {}
pub type Pj6 = Pin<Pj6Id, GpiojId>;
impl AltFn<super::sig::LcdR7> for Pj6Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ7: Pin<Pj7Id, GpiojId> = Pin { port: GPIOJ, index: 7, id: Pj7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj7Id {}
pub type Pj7 = Pin<Pj7Id, GpiojId>;
impl AltFn<super::sig::LcdG0> for Pj7Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ8: Pin<Pj8Id, GpiojId> = Pin { port: GPIOJ, index: 8, id: Pj8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj8Id {}
pub type Pj8 = Pin<Pj8Id, GpiojId>;
impl AltFn<super::sig::LcdG1> for Pj8Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ9: Pin<Pj9Id, GpiojId> = Pin { port: GPIOJ, index: 9, id: Pj9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj9Id {}
pub type Pj9 = Pin<Pj9Id, GpiojId>;
impl AltFn<super::sig::LcdG2> for Pj9Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ10: Pin<Pj10Id, GpiojId> = Pin { port: GPIOJ, index: 10, id: Pj10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj10Id {}
pub type Pj10 = Pin<Pj10Id, GpiojId>;
impl AltFn<super::sig::LcdG3> for Pj10Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ11: Pin<Pj11Id, GpiojId> = Pin { port: GPIOJ, index: 11, id: Pj11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj11Id {}
pub type Pj11 = Pin<Pj11Id, GpiojId>;
impl AltFn<super::sig::LcdG4> for Pj11Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ12: Pin<Pj12Id, GpiojId> = Pin { port: GPIOJ, index: 12, id: Pj12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj12Id {}
pub type Pj12 = Pin<Pj12Id, GpiojId>;
impl AltFn<super::sig::LcdB0> for Pj12Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ13: Pin<Pj13Id, GpiojId> = Pin { port: GPIOJ, index: 13, id: Pj13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj13Id {}
pub type Pj13 = Pin<Pj13Id, GpiojId>;
impl AltFn<super::sig::LcdB1> for Pj13Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ14: Pin<Pj14Id, GpiojId> = Pin { port: GPIOJ, index: 14, id: Pj14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj14Id {}
pub type Pj14 = Pin<Pj14Id, GpiojId>;
impl AltFn<super::sig::LcdB2> for Pj14Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ15: Pin<Pj15Id, GpiojId> = Pin { port: GPIOJ, index: 15, id: Pj15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pj15Id {}
pub type Pj15 = Pin<Pj15Id, GpiojId>;
impl AltFn<super::sig::LcdB3> for Pj15Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pj15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK0: Pin<Pk0Id, GpiokId> = Pin { port: GPIOK, index: 0, id: Pk0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pk0Id {}
pub type Pk0 = Pin<Pk0Id, GpiokId>;
impl AltFn<super::sig::LcdG5> for Pk0Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pk0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK1: Pin<Pk1Id, GpiokId> = Pin { port: GPIOK, index: 1, id: Pk1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pk1Id {}
pub type Pk1 = Pin<Pk1Id, GpiokId>;
impl AltFn<super::sig::LcdG6> for Pk1Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pk1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK2: Pin<Pk2Id, GpiokId> = Pin { port: GPIOK, index: 2, id: Pk2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pk2Id {}
pub type Pk2 = Pin<Pk2Id, GpiokId>;
impl AltFn<super::sig::LcdG7> for Pk2Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pk2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK3: Pin<Pk3Id, GpiokId> = Pin { port: GPIOK, index: 3, id: Pk3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pk3Id {}
pub type Pk3 = Pin<Pk3Id, GpiokId>;
impl AltFn<super::sig::LcdB4> for Pk3Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pk3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK4: Pin<Pk4Id, GpiokId> = Pin { port: GPIOK, index: 4, id: Pk4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pk4Id {}
pub type Pk4 = Pin<Pk4Id, GpiokId>;
impl AltFn<super::sig::LcdB5> for Pk4Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pk4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK5: Pin<Pk5Id, GpiokId> = Pin { port: GPIOK, index: 5, id: Pk5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pk5Id {}
pub type Pk5 = Pin<Pk5Id, GpiokId>;
impl AltFn<super::sig::LcdB6> for Pk5Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pk5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK6: Pin<Pk6Id, GpiokId> = Pin { port: GPIOK, index: 6, id: Pk6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pk6Id {}
pub type Pk6 = Pin<Pk6Id, GpiokId>;
impl AltFn<super::sig::LcdB7> for Pk6Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pk6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK7: Pin<Pk7Id, GpiokId> = Pin { port: GPIOK, index: 7, id: Pk7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pk7Id {}
pub type Pk7 = Pin<Pk7Id, GpiokId>;
impl AltFn<super::sig::LcdDe> for Pk7Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Eventout> for Pk7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

