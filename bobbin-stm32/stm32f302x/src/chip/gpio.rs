#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::gpio::*;

pub const GPIOA: Gpioa = Periph(0x48000000, GpioaId {});
pub const GPIOB: Gpiob = Periph(0x48000400, GpiobId {});
pub const GPIOC: Gpioc = Periph(0x48000800, GpiocId {});
pub const GPIOD: Gpiod = Periph(0x48000c00, GpiodId {});
pub const GPIOF: Gpiof = Periph(0x48001400, GpiofId {});

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
pub struct GpiofId {}
pub type Gpiof = Periph<GpiofId>;







pub const PA0: Pin<Pa0Id, GpioaId> = Pin { port: GPIOA, index: 0, id: Pa0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa0Id {}
pub type Pa0 = Pin<Pa0Id, GpioaId>;
impl AltFn<super::sig::Tim2Ch1> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Etr> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG1Io1> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Cts> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA1: Pin<Pa1Id, GpioaId> = Pin { port: GPIOA, index: 1, id: Pa1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa1Id {}
pub type Pa1 = Pin<Pa1Id, GpioaId>;
impl AltFn<super::sig::RtcRefin> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch2> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG1Io2> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2RtsDe> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim15Ch1n> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA2: Pin<Pa2Id, GpioaId> = Pin { port: GPIOA, index: 2, id: Pa2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa2Id {}
pub type Pa2 = Pin<Pa2Id, GpioaId>;
impl AltFn<super::sig::Tim2Ch3> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG1Io3> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Tx> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Comp2Out> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim15Ch1> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA3: Pin<Pa3Id, GpioaId> = Pin { port: GPIOA, index: 3, id: Pa3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa3Id {}
pub type Pa3 = Pin<Pa3Id, GpioaId>;
impl AltFn<super::sig::Tim2Ch4> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG1Io4> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Rx> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim15Ch2> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA4: Pin<Pa4Id, GpioaId> = Pin { port: GPIOA, index: 4, id: Pa4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa4Id {}
pub type Pa4 = Pin<Pa4Id, GpioaId>;
impl AltFn<super::sig::TscG2Io1> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
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

impl AltFn<super::sig::Eventout> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA5: Pin<Pa5Id, GpioaId> = Pin { port: GPIOA, index: 5, id: Pa5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa5Id {}
pub type Pa5 = Pin<Pa5Id, GpioaId>;
impl AltFn<super::sig::Tim2Ch1> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Etr> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG2Io2> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Eventout> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA6: Pin<Pa6Id, GpioaId> = Pin { port: GPIOA, index: 6, id: Pa6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa6Id {}
pub type Pa6 = Pin<Pa6Id, GpioaId>;
impl AltFn<super::sig::Tim16Ch1> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG2Io3> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim1Bkin> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Eventout> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA7: Pin<Pa7Id, GpioaId> = Pin { port: GPIOA, index: 7, id: Pa7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa7Id {}
pub type Pa7 = Pin<Pa7Id, GpioaId>;
impl AltFn<super::sig::Tim17Ch1> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG2Io4> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim1Ch1n> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Eventout> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA8: Pin<Pa8Id, GpioaId> = Pin { port: GPIOA, index: 8, id: Pa8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa8Id {}
pub type Pa8 = Pin<Pa8Id, GpioaId>;
impl AltFn<super::sig::Mco> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c3Scl> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c2Smbal> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s2Mck> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch1> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1Ck> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA9: Pin<Pa9Id, GpioaId> = Pin { port: GPIOA, index: 9, id: Pa9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa9Id {}
pub type Pa9 = Pin<Pa9Id, GpioaId>;
impl AltFn<super::sig::I2c3Smbal> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG4Io1> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c2Scl> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s3Mck> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch2> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1Tx> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim15Bkin> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim2Ch3> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA10: Pin<Pa10Id, GpioaId> = Pin { port: GPIOA, index: 10, id: Pa10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa10Id {}
pub type Pa10 = Pin<Pa10Id, GpioaId>;
impl AltFn<super::sig::Tim17Bkin> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG4Io2> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c2Sda> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Miso> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2extSd> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch3> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1Rx> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Comp6Out> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim2Ch4> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA11: Pin<Pa11Id, GpioaId> = Pin { port: GPIOA, index: 11, id: Pa11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa11Id {}
pub type Pa11 = Pin<Pa11Id, GpioaId>;
impl AltFn<super::sig::Spi2Mosi> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Sd> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch1n> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1Cts> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::CanRx> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim1Ch4> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Tim1Bkin2> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA12: Pin<Pa12Id, GpioaId> = Pin { port: GPIOA, index: 12, id: Pa12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa12Id {}
pub type Pa12 = Pin<Pa12Id, GpioaId>;
impl AltFn<super::sig::Tim16Ch1> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2sckin> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch2n> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1RtsDe> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Comp2Out> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::CanTx> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim1Etr> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Eventout> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA13: Pin<Pa13Id, GpioaId> = Pin { port: GPIOA, index: 13, id: Pa13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa13Id {}
pub type Pa13 = Pin<Pa13Id, GpioaId>;
impl AltFn<super::sig::Swdat> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Jtms> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim16Ch1n> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG4Io3> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::IrOut> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart3Cts> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA14: Pin<Pa14Id, GpioaId> = Pin { port: GPIOA, index: 14, id: Pa14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa14Id {}
pub type Pa14 = Pin<Pa14Id, GpioaId>;
impl AltFn<super::sig::Swclk> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Jtck> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::TscG4Io4> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Sda> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim1Bkin> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Tx> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::TscSync> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Scl> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi3Nss> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ws> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Rx> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim1Bkin> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Eventout> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB0: Pin<Pb0Id, GpiobId> = Pin { port: GPIOB, index: 0, id: Pb0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb0Id {}
pub type Pb0 = Pin<Pb0Id, GpiobId>;
impl AltFn<super::sig::TscG3Io2> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim1Ch2n> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Eventout> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB1: Pin<Pb1Id, GpiobId> = Pin { port: GPIOB, index: 1, id: Pb1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb1Id {}
pub type Pb1 = Pin<Pb1Id, GpiobId>;
impl AltFn<super::sig::TscG3Io3> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim1Ch3n> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Comp4Out> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Eventout> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB2: Pin<Pb2Id, GpiobId> = Pin { port: GPIOB, index: 2, id: Pb2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb2Id {}
pub type Pb2 = Pin<Pb2Id, GpiobId>;
impl AltFn<super::sig::TscG3Io4> for Pb2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

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

impl AltFn<super::sig::TscG5Io1> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi3Sck> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ck> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Tx> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB4: Pin<Pb4Id, GpiobId> = Pin { port: GPIOB, index: 4, id: Pb4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb4Id {}
pub type Pb4 = Pin<Pb4Id, GpiobId>;
impl AltFn<super::sig::Jtrst> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim16Ch1> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG5Io2> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi3Miso> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Sd> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Rx> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim17Bkin> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB5: Pin<Pb5Id, GpiobId> = Pin { port: GPIOB, index: 5, id: Pb5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb5Id {}
pub type Pb5 = Pin<Pb5Id, GpiobId>;
impl AltFn<super::sig::Tim16Bkin> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Smbal> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi3Mosi> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3extSd> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Ck> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::I2c3Sda> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim17Ch1> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Eventout> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB6: Pin<Pb6Id, GpiobId> = Pin { port: GPIOB, index: 6, id: Pb6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb6Id {}
pub type Pb6 = Pin<Pb6Id, GpiobId>;
impl AltFn<super::sig::Tim16Ch1n> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG5Io3> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Scl> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Tx> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB7: Pin<Pb7Id, GpiobId> = Pin { port: GPIOB, index: 7, id: Pb7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb7Id {}
pub type Pb7 = Pin<Pb7Id, GpiobId>;
impl AltFn<super::sig::Tim17Ch1n> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG5Io4> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Sda> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart1Rx> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB8: Pin<Pb8Id, GpiobId> = Pin { port: GPIOB, index: 8, id: Pb8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb8Id {}
pub type Pb8 = Pin<Pb8Id, GpiobId>;
impl AltFn<super::sig::Tim16Ch1> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscSync> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Scl> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart3Rx> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::CanRx> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim1Bkin> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::Eventout> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB9: Pin<Pb9Id, GpiobId> = Pin { port: GPIOB, index: 9, id: Pb9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb9Id {}
pub type Pb9 = Pin<Pb9Id, GpiobId>;
impl AltFn<super::sig::Tim17Ch1> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Sda> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::IrOut> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Tx> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Comp2Out> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::CanTx> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
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

impl AltFn<super::sig::TscSync> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart3Tx> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB11: Pin<Pb11Id, GpiobId> = Pin { port: GPIOB, index: 11, id: Pb11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb11Id {}
pub type Pb11 = Pin<Pb11Id, GpiobId>;
impl AltFn<super::sig::Tim2Ch4> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG6Io1> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart3Rx> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB12: Pin<Pb12Id, GpiobId> = Pin { port: GPIOB, index: 12, id: Pb12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb12Id {}
pub type Pb12 = Pin<Pb12Id, GpiobId>;
impl AltFn<super::sig::TscG6Io2> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c2Smbal> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Nss> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ws> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Bkin> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Ck> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB13: Pin<Pb13Id, GpiobId> = Pin { port: GPIOB, index: 13, id: Pb13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb13Id {}
pub type Pb13 = Pin<Pb13Id, GpiobId>;
impl AltFn<super::sig::TscG6Io3> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Sck> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch1n> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Cts> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Eventout> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB14: Pin<Pb14Id, GpiobId> = Pin { port: GPIOB, index: 14, id: Pb14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb14Id {}
pub type Pb14 = Pin<Pb14Id, GpiobId>;
impl AltFn<super::sig::Tim15Ch1> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::TscG6Io4> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Miso> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2extSd> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch2n> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3RtsDe> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
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

impl AltFn<super::sig::Tim15Ch2> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim15Ch1n> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim1Ch3n> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Mosi> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Sd> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Eventout> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC0: Pin<Pc0Id, GpiocId> = Pin { port: GPIOC, index: 0, id: Pc0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc0Id {}
pub type Pc0 = Pin<Pc0Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch1> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PC1: Pin<Pc1Id, GpiocId> = Pin { port: GPIOC, index: 1, id: Pc1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc1Id {}
pub type Pc1 = Pin<Pc1Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch2> for Pc1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PC2: Pin<Pc2Id, GpiocId> = Pin { port: GPIOC, index: 2, id: Pc2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc2Id {}
pub type Pc2 = Pin<Pc2Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch3> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PC3: Pin<Pc3Id, GpiocId> = Pin { port: GPIOC, index: 3, id: Pc3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc3Id {}
pub type Pc3 = Pin<Pc3Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch4> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim1Bkin2> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PC4: Pin<Pc4Id, GpiocId> = Pin { port: GPIOC, index: 4, id: Pc4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc4Id {}
pub type Pc4 = Pin<Pc4Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Etr> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart1Tx> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC5: Pin<Pc5Id, GpiocId> = Pin { port: GPIOC, index: 5, id: Pc5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc5Id {}
pub type Pc5 = Pin<Pc5Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim15Bkin> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG3Io1> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart1Rx> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC6: Pin<Pc6Id, GpiocId> = Pin { port: GPIOC, index: 6, id: Pc6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc6Id {}
pub type Pc6 = Pin<Pc6Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2s2Mck> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Comp6Out> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC7: Pin<Pc7Id, GpiocId> = Pin { port: GPIOC, index: 7, id: Pc7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc7Id {}
pub type Pc7 = Pin<Pc7Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2s3Mck> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PC8: Pin<Pc8Id, GpiocId> = Pin { port: GPIOC, index: 8, id: Pc8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc8Id {}
pub type Pc8 = Pin<Pc8Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PC9: Pin<Pc9Id, GpiocId> = Pin { port: GPIOC, index: 9, id: Pc9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc9Id {}
pub type Pc9 = Pin<Pc9Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c3Sda> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2sckin> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PC10: Pin<Pc10Id, GpiocId> = Pin { port: GPIOC, index: 10, id: Pc10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc10Id {}
pub type Pc10 = Pin<Pc10Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi3Sck> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Ck> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Tx> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC11: Pin<Pc11Id, GpiocId> = Pin { port: GPIOC, index: 11, id: Pc11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc11Id {}
pub type Pc11 = Pin<Pc11Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi3Miso> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3extSd> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Rx> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC12: Pin<Pc12Id, GpiocId> = Pin { port: GPIOC, index: 12, id: Pc12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc12Id {}
pub type Pc12 = Pin<Pc12Id, GpiocId>;
impl AltFn<super::sig::Eventout> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi3Mosi> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3Sd> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart3Ck> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC13: Pin<Pc13Id, GpiocId> = Pin { port: GPIOC, index: 13, id: Pc13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc13Id {}
pub type Pc13 = Pin<Pc13Id, GpiocId>;
impl AltFn<super::sig::Tim1Ch1n> for Pc13Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PC14: Pin<Pc14Id, GpiocId> = Pin { port: GPIOC, index: 14, id: Pc14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc14Id {}
pub type Pc14 = Pin<Pc14Id, GpiocId>;
pub const PC15: Pin<Pc15Id, GpiocId> = Pin { port: GPIOC, index: 15, id: Pc15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc15Id {}
pub type Pc15 = Pin<Pc15Id, GpiocId>;
pub const PD2: Pin<Pd2Id, GpiodId> = Pin { port: GPIOD, index: 2, id: Pd2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd2Id {}
pub type Pd2 = Pin<Pd2Id, GpiodId>;
impl AltFn<super::sig::Eventout> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PF0: Pin<Pf0Id, GpiofId> = Pin { port: GPIOF, index: 0, id: Pf0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf0Id {}
pub type Pf0 = Pin<Pf0Id, GpiofId>;
impl AltFn<super::sig::I2c2Sda> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Nss> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ws> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch3n> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PF1: Pin<Pf1Id, GpiofId> = Pin { port: GPIOF, index: 1, id: Pf1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf1Id {}
pub type Pf1 = Pin<Pf1Id, GpiofId>;
impl AltFn<super::sig::I2c2Scl> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Sck> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

