pub use stm32_common::chip::gpio::*;

pub const GPIOA: Gpioa = Periph(0x48000000, GpioaId {});
pub const GPIOB: Gpiob = Periph(0x48000400, GpiobId {});
pub const GPIOC: Gpioc = Periph(0x48000800, GpiocId {});
pub const GPIOD: Gpiod = Periph(0x48000c00, GpiodId {});
pub const GPIOE: Gpioe = Periph(0x48001000, GpioeId {});
pub const GPIOF: Gpiof = Periph(0x48001400, GpiofId {});
pub const GPIOG: Gpiog = Periph(0x48001800, GpiogId {});
pub const GPIOH: Gpioh = Periph(0x48001c00, GpiohId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpioaId {}
pub type Gpioa = Periph<GpioaId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiobId {}
pub type Gpiob = Periph<GpiobId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiocId {}
pub type Gpioc = Periph<GpiocId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiodId {}
pub type Gpiod = Periph<GpiodId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpioeId {}
pub type Gpioe = Periph<GpioeId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiofId {}
pub type Gpiof = Periph<GpiofId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiogId {}
pub type Gpiog = Periph<GpiogId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiohId {}
pub type Gpioh = Periph<GpiohId>;










pub const PA0: Pin<Pa0Id, GpioaId> = Pin { port: GPIOA, index: 0, id: Pa0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Comp1Out> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim8Bkin> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim8Etr> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EventOut> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA1: Pin<Pa1Id, GpioaId> = Pin { port: GPIOA, index: 1, id: Pa1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Usart2Rts> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim15Ch1n> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::EventOut> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA2: Pin<Pa2Id, GpioaId> = Pin { port: GPIOA, index: 2, id: Pa2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::EventOut> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA3: Pin<Pa3Id, GpioaId> = Pin { port: GPIOA, index: 3, id: Pa3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa3Id {}
pub type Pa3 = Pin<Pa3Id, GpioaId>;
impl AltFn<super::sig::Tim2Ch3> for Pa3Id {
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

impl AltFn<super::sig::EventOut> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA4: Pin<Pa4Id, GpioaId> = Pin { port: GPIOA, index: 4, id: Pa4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa4Id {}
pub type Pa4 = Pin<Pa4Id, GpioaId>;
impl AltFn<super::sig::Tim3Ch2> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG2Io1> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
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

impl AltFn<super::sig::EventOut> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA5: Pin<Pa5Id, GpioaId> = Pin { port: GPIOA, index: 5, id: Pa5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Spi1Sck> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EventOut> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA6: Pin<Pa6Id, GpioaId> = Pin { port: GPIOA, index: 6, id: Pa6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa6Id {}
pub type Pa6 = Pin<Pa6Id, GpioaId>;
impl AltFn<super::sig::Tim16Ch1> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch1> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG2Io3> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim8Bkin> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi1Miso> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Bkin> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Comp1Out> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EventOut> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA7: Pin<Pa7Id, GpioaId> = Pin { port: GPIOA, index: 7, id: Pa7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa7Id {}
pub type Pa7 = Pin<Pa7Id, GpioaId>;
impl AltFn<super::sig::Tim17Ch1> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch2> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG2Io4> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim8Ch1n> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi1Mosi> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Ch1n> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::EventOut> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA8: Pin<Pa8Id, GpioaId> = Pin { port: GPIOA, index: 8, id: Pa8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Comp3Out> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim4Etr> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EventOut> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA9: Pin<Pa9Id, GpioaId> = Pin { port: GPIOA, index: 9, id: Pa9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Comp5Out> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim15Bkin> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim2Ch3> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EventOut> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA10: Pin<Pa10Id, GpioaId> = Pin { port: GPIOA, index: 10, id: Pa10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Tim8Bkin> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EventOut> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA11: Pin<Pa11Id, GpioaId> = Pin { port: GPIOA, index: 11, id: Pa11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Comp1Out> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::CanRx> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim4Ch1> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Tim1Ch4> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Tim1Bkin2> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::EventOut> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA12: Pin<Pa12Id, GpioaId> = Pin { port: GPIOA, index: 12, id: Pa12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Usart1Rts> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Comp2Out> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::CanTx> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim4Ch2> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Tim1Etr> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::EventOut> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA13: Pin<Pa13Id, GpioaId> = Pin { port: GPIOA, index: 13, id: Pa13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa13Id {}
pub type Pa13 = Pin<Pa13Id, GpioaId>;
impl AltFn<super::sig::Swdio> for Pa13Id {
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

impl AltFn<super::sig::Tim4Ch3> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EventOut> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA14: Pin<Pa14Id, GpioaId> = Pin { port: GPIOA, index: 14, id: Pa14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Tim8Ch2> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Bkin> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Tx> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::EventOut> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA15: Pin<Pa15Id, GpioaId> = Pin { port: GPIOA, index: 15, id: Pa15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Tim8Ch1> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscSync> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Scl> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
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

impl AltFn<super::sig::Usart2Rx> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim1Bkin> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::EventOut> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB0: Pin<Pb0Id, GpiobId> = Pin { port: GPIOB, index: 0, id: Pb0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb0Id {}
pub type Pb0 = Pin<Pb0Id, GpiobId>;
impl AltFn<super::sig::Tim3Ch3> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG3Io2> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim8Ch2n> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim1Ch2n> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::EventOut> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB1: Pin<Pb1Id, GpiobId> = Pin { port: GPIOB, index: 1, id: Pb1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb1Id {}
pub type Pb1 = Pin<Pb1Id, GpiobId>;
impl AltFn<super::sig::Tim3Ch4> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG3Io3> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim8Ch3n> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim1Ch3n> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Comp4Out> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::EventOut> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB2: Pin<Pb2Id, GpiobId> = Pin { port: GPIOB, index: 2, id: Pb2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb2Id {}
pub type Pb2 = Pin<Pb2Id, GpiobId>;
impl AltFn<super::sig::TscG3Io4> for Pb2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::EventOut> for Pb2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB3: Pin<Pb3Id, GpiobId> = Pin { port: GPIOB, index: 3, id: Pb3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Tim4Etr> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG5Io1> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim8Ch1n> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi1Sck> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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

impl AltFn<super::sig::Tim3Etr> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EventOut> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB4: Pin<Pb4Id, GpiobId> = Pin { port: GPIOB, index: 4, id: Pb4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb4Id {}
pub type Pb4 = Pin<Pb4Id, GpiobId>;
impl AltFn<super::sig::Jtrst> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim16Ch1> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch1> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG5Io1> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim8Ch2n> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi1Miso> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi3Miso> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s3extSd> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart2Rx> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim17Bkin> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EventOut> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB5: Pin<Pb5Id, GpiobId> = Pin { port: GPIOB, index: 5, id: Pb5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb5Id {}
pub type Pb5 = Pin<Pb5Id, GpiobId>;
impl AltFn<super::sig::Tim16Bkin> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch2> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch3n> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Smbal> for Pb5Id {
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

impl AltFn<super::sig::Usart2Ck> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::I2c3Sda> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Tim17Ch1> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EventOut> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB6: Pin<Pb6Id, GpiobId> = Pin { port: GPIOB, index: 6, id: Pb6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb6Id {}
pub type Pb6 = Pin<Pb6Id, GpiobId>;
impl AltFn<super::sig::Tim16Ch1n> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim4Ch1> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG5Io3> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Scl> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim8Ch1> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim8Etr> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1Tx> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim8Bkin2> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EventOut> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB7: Pin<Pb7Id, GpiobId> = Pin { port: GPIOB, index: 7, id: Pb7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb7Id {}
pub type Pb7 = Pin<Pb7Id, GpiobId>;
impl AltFn<super::sig::Tim17Ch1n> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim4Ch2> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG5Io4> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Sda> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim8Bkin> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usart1Rx> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tim3Ch4> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::FmcNadv> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::EventOut> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB8: Pin<Pb8Id, GpiobId> = Pin { port: GPIOB, index: 8, id: Pb8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb8Id {}
pub type Pb8 = Pin<Pb8Id, GpiobId>;
impl AltFn<super::sig::Tim16Ch1> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim4Ch3> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
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

impl AltFn<super::sig::Comp1Out> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::CanRx> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 9 }
}

impl AltFn<super::sig::Tim8Ch2> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::Tim1Bkin> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::EventOut> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB9: Pin<Pb9Id, GpiobId> = Pin { port: GPIOB, index: 9, id: Pb9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb9Id {}
pub type Pb9 = Pin<Pb9Id, GpiobId>;
impl AltFn<super::sig::Tim17Ch1> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim4Ch4> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
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

impl AltFn<super::sig::Tim8Ch3> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 10 }
}

impl AltFn<super::sig::EventOut> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB10: Pin<Pb10Id, GpiobId> = Pin { port: GPIOB, index: 10, id: Pb10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::EventOut> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB11: Pin<Pb11Id, GpiobId> = Pin { port: GPIOB, index: 11, id: Pb11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::EventOut> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB12: Pin<Pb12Id, GpiobId> = Pin { port: GPIOB, index: 12, id: Pb12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb12Id {}
pub type Pb12 = Pin<Pb12Id, GpiobId>;
impl AltFn<super::sig::TscG6Io2> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c1Smbal> for Pb12Id {
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

impl AltFn<super::sig::EventOut> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB13: Pin<Pb13Id, GpiobId> = Pin { port: GPIOB, index: 13, id: Pb13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::EventOut> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB14: Pin<Pb14Id, GpiobId> = Pin { port: GPIOB, index: 14, id: Pb14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Usart3Rts> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::EventOut> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB15: Pin<Pb15Id, GpiobId> = Pin { port: GPIOB, index: 15, id: Pb15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Mosi> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Sd> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EventOut> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC0: Pin<Pc0Id, GpiocId> = Pin { port: GPIOC, index: 0, id: Pc0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc0Id {}
pub type Pc0 = Pin<Pc0Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch1> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PC1: Pin<Pc1Id, GpiocId> = Pin { port: GPIOC, index: 1, id: Pc1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc1Id {}
pub type Pc1 = Pin<Pc1Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch2> for Pc1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PC2: Pin<Pc2Id, GpiocId> = Pin { port: GPIOC, index: 2, id: Pc2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc2Id {}
pub type Pc2 = Pin<Pc2Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch3> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Comp7Out> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PC3: Pin<Pc3Id, GpiocId> = Pin { port: GPIOC, index: 3, id: Pc3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc3Id {}
pub type Pc3 = Pin<Pc3Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc3Id {
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
pub struct Pc4Id {}
pub type Pc4 = Pin<Pc4Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Chetr> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart1Tx> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC5: Pin<Pc5Id, GpiocId> = Pin { port: GPIOC, index: 5, id: Pc5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc5Id {}
pub type Pc5 = Pin<Pc5Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc5Id {
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
pub struct Pc6Id {}
pub type Pc6 = Pin<Pc6Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch1> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch1> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s2Mck> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Comp6Out> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC7: Pin<Pc7Id, GpiocId> = Pin { port: GPIOC, index: 7, id: Pc7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc7Id {}
pub type Pc7 = Pin<Pc7Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch2> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch2> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s3Mck> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Comp5Out> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC8: Pin<Pc8Id, GpiocId> = Pin { port: GPIOC, index: 8, id: Pc8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc8Id {}
pub type Pc8 = Pin<Pc8Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch3> for Pc8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Ch3> for Pc8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Comp3Out> for Pc8Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PC9: Pin<Pc9Id, GpiocId> = Pin { port: GPIOC, index: 9, id: Pc9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc9Id {}
pub type Pc9 = Pin<Pc9Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch4> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c3Sda> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim8Ch4> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2sckin> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim8Bkin2> for Pc9Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PC10: Pin<Pc10Id, GpiocId> = Pin { port: GPIOC, index: 10, id: Pc10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc10Id {}
pub type Pc10 = Pin<Pc10Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim8Ch1n> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart4Tx> for Pc10Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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
pub struct Pc11Id {}
pub type Pc11 = Pin<Pc11Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim8Ch2n> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart4Rx> for Pc11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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
pub struct Pc12Id {}
pub type Pc12 = Pin<Pc12Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim8Ch3n> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart5Tx> for Pc12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
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
pub struct Pc13Id {}
pub type Pc13 = Pin<Pc13Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim8Ch1n> for Pc13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PC14: Pin<Pc14Id, GpiocId> = Pin { port: GPIOC, index: 14, id: Pc14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc14Id {}
pub type Pc14 = Pin<Pc14Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PC15: Pin<Pc15Id, GpiocId> = Pin { port: GPIOC, index: 15, id: Pc15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc15Id {}
pub type Pc15 = Pin<Pc15Id, GpiocId>;
impl AltFn<super::sig::EventOut> for Pc15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PD0: Pin<Pd0Id, GpiodId> = Pin { port: GPIOD, index: 0, id: Pd0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd0Id {}
pub type Pd0 = Pin<Pd0Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::CanRx> for Pd0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcD2> for Pd0Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD1: Pin<Pd1Id, GpiodId> = Pin { port: GPIOD, index: 1, id: Pd1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd1Id {}
pub type Pd1 = Pin<Pd1Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim8Ch4> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim8Bkin2> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::CanTx> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcD3> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD2: Pin<Pd2Id, GpiodId> = Pin { port: GPIOD, index: 2, id: Pd2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd2Id {}
pub type Pd2 = Pin<Pd2Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Etr> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim8Bkin> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart5Rx> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PD3: Pin<Pd3Id, GpiodId> = Pin { port: GPIOD, index: 3, id: Pd3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd3Id {}
pub type Pd3 = Pin<Pd3Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Ch1> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim2Etr> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart2Cts> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcClk> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD4: Pin<Pd4Id, GpiodId> = Pin { port: GPIOD, index: 4, id: Pd4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd4Id {}
pub type Pd4 = Pin<Pd4Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Ch2> for Pd4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart2Rts> for Pd4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcNoe> for Pd4Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD5: Pin<Pd5Id, GpiodId> = Pin { port: GPIOD, index: 5, id: Pd5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd5Id {}
pub type Pd5 = Pin<Pd5Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart2Tx> for Pd5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcNwe> for Pd5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD6: Pin<Pd6Id, GpiodId> = Pin { port: GPIOD, index: 6, id: Pd6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd6Id {}
pub type Pd6 = Pin<Pd6Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Ch4> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart2Rx> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcNwait> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD7: Pin<Pd7Id, GpiodId> = Pin { port: GPIOD, index: 7, id: Pd7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd7Id {}
pub type Pd7 = Pin<Pd7Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Ch3> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart2Ck> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcNe1> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::FmcNce2> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD8: Pin<Pd8Id, GpiodId> = Pin { port: GPIOD, index: 8, id: Pd8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd8Id {}
pub type Pd8 = Pin<Pd8Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart3Tx> for Pd8Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcD13> for Pd8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD9: Pin<Pd9Id, GpiodId> = Pin { port: GPIOD, index: 9, id: Pd9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd9Id {}
pub type Pd9 = Pin<Pd9Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart3Rx> for Pd9Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcD14> for Pd9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD10: Pin<Pd10Id, GpiodId> = Pin { port: GPIOD, index: 10, id: Pd10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd10Id {}
pub type Pd10 = Pin<Pd10Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart3Ck> for Pd10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcD15> for Pd10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD11: Pin<Pd11Id, GpiodId> = Pin { port: GPIOD, index: 11, id: Pd11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd11Id {}
pub type Pd11 = Pin<Pd11Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart3Cts> for Pd11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcA16> for Pd11Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD12: Pin<Pd12Id, GpiodId> = Pin { port: GPIOD, index: 12, id: Pd12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd12Id {}
pub type Pd12 = Pin<Pd12Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim4Ch1> for Pd12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG8Io1> for Pd12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart3Rts> for Pd12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcA17> for Pd12Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD13: Pin<Pd13Id, GpiodId> = Pin { port: GPIOD, index: 13, id: Pd13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd13Id {}
pub type Pd13 = Pin<Pd13Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim4Ch2> for Pd13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG8Io2> for Pd13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FmcA18> for Pd13Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD14: Pin<Pd14Id, GpiodId> = Pin { port: GPIOD, index: 14, id: Pd14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd14Id {}
pub type Pd14 = Pin<Pd14Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim4Ch3> for Pd14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG8Io3> for Pd14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FmcD0> for Pd14Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PD15: Pin<Pd15Id, GpiodId> = Pin { port: GPIOD, index: 15, id: Pd15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd15Id {}
pub type Pd15 = Pin<Pd15Id, GpiodId>;
impl AltFn<super::sig::EventOut> for Pd15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim4Ch4> for Pd15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG8Io4> for Pd15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Nss> for Pd15Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcD1> for Pd15Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE0: Pin<Pe0Id, GpioeId> = Pin { port: GPIOE, index: 0, id: Pe0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe0Id {}
pub type Pe0 = Pin<Pe0Id, GpioeId>;
impl AltFn<super::sig::EventOut> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim4Etr> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim16Ch1> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim20Etr> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1Tx> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcNbl0> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE1: Pin<Pe1Id, GpioeId> = Pin { port: GPIOE, index: 1, id: Pe1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe1Id {}
pub type Pe1 = Pin<Pe1Id, GpioeId>;
impl AltFn<super::sig::EventOut> for Pe1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim17Ch1> for Pe1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim20Ch4> for Pe1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usart1Rx> for Pe1Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::FmcNbl1> for Pe1Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE2: Pin<Pe2Id, GpioeId> = Pin { port: GPIOE, index: 2, id: Pe2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe2Id {}
pub type Pe2 = Pin<Pe2Id, GpioeId>;
impl AltFn<super::sig::Traceck> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::EventOut> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch1> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG7Io1> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi4Sck> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim20Ch1> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcA23> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE3: Pin<Pe3Id, GpioeId> = Pin { port: GPIOE, index: 3, id: Pe3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe3Id {}
pub type Pe3 = Pin<Pe3Id, GpioeId>;
impl AltFn<super::sig::Traced0> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::EventOut> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch2> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG7Io2> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi4Nss> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim20Ch2> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcA19> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE4: Pin<Pe4Id, GpioeId> = Pin { port: GPIOE, index: 4, id: Pe4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe4Id {}
pub type Pe4 = Pin<Pe4Id, GpioeId>;
impl AltFn<super::sig::Traced1> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::EventOut> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch3> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG7Io3> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi4Nss> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim20Ch1n> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcA20> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE5: Pin<Pe5Id, GpioeId> = Pin { port: GPIOE, index: 5, id: Pe5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe5Id {}
pub type Pe5 = Pin<Pe5Id, GpioeId>;
impl AltFn<super::sig::Traced2> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::EventOut> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim3Ch4> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TscG7Io4> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi4Miso> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim20Ch2n> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcA21> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE6: Pin<Pe6Id, GpioeId> = Pin { port: GPIOE, index: 6, id: Pe6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe6Id {}
pub type Pe6 = Pin<Pe6Id, GpioeId>;
impl AltFn<super::sig::Traced3> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::EventOut> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi4Mosi> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim20Ch3n> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcA22> for Pe6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE7: Pin<Pe7Id, GpioeId> = Pin { port: GPIOE, index: 7, id: Pe7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe7Id {}
pub type Pe7 = Pin<Pe7Id, GpioeId>;
impl AltFn<super::sig::EventOut> for Pe7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Etr> for Pe7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcD4> for Pe7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE8: Pin<Pe8Id, GpioeId> = Pin { port: GPIOE, index: 8, id: Pe8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe8Id {}
pub type Pe8 = Pin<Pe8Id, GpioeId>;
impl AltFn<super::sig::EventOut> for Pe8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch1n> for Pe8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcD5> for Pe8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE9: Pin<Pe9Id, GpioeId> = Pin { port: GPIOE, index: 9, id: Pe9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe9Id {}
pub type Pe9 = Pin<Pe9Id, GpioeId>;
impl AltFn<super::sig::EventOut> for Pe9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch1> for Pe9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcD6> for Pe9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE10: Pin<Pe10Id, GpioeId> = Pin { port: GPIOE, index: 10, id: Pe10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe10Id {}
pub type Pe10 = Pin<Pe10Id, GpioeId>;
impl AltFn<super::sig::EventOut> for Pe10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch2n> for Pe10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcD7> for Pe10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE11: Pin<Pe11Id, GpioeId> = Pin { port: GPIOE, index: 11, id: Pe11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe11Id {}
pub type Pe11 = Pin<Pe11Id, GpioeId>;
impl AltFn<super::sig::EventOut> for Pe11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch2> for Pe11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Spi4Nss> for Pe11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcD8> for Pe11Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE12: Pin<Pe12Id, GpioeId> = Pin { port: GPIOE, index: 12, id: Pe12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe12Id {}
pub type Pe12 = Pin<Pe12Id, GpioeId>;
impl AltFn<super::sig::EventOut> for Pe12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch3n> for Pe12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Spi4Sck> for Pe12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcD9> for Pe12Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE13: Pin<Pe13Id, GpioeId> = Pin { port: GPIOE, index: 13, id: Pe13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe13Id {}
pub type Pe13 = Pin<Pe13Id, GpioeId>;
impl AltFn<super::sig::EventOut> for Pe13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch3> for Pe13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Spi4Miso> for Pe13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcD10> for Pe13Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE14: Pin<Pe14Id, GpioeId> = Pin { port: GPIOE, index: 14, id: Pe14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe14Id {}
pub type Pe14 = Pin<Pe14Id, GpioeId>;
impl AltFn<super::sig::EventOut> for Pe14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Ch4> for Pe14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Spi4Mosi> for Pe14Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tim1Bkin2> for Pe14Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcD11> for Pe14Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PE15: Pin<Pe15Id, GpioeId> = Pin { port: GPIOE, index: 15, id: Pe15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe15Id {}
pub type Pe15 = Pin<Pe15Id, GpioeId>;
impl AltFn<super::sig::EventOut> for Pe15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim1Bkin> for Pe15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart3Rx> for Pe15Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcD12> for Pe15Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF0: Pin<Pf0Id, GpiofId> = Pin { port: GPIOF, index: 0, id: Pf0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf0Id {}
pub type Pf0 = Pin<Pf0Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

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
pub struct Pf1Id {}
pub type Pf1 = Pin<Pf1Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c2Scl> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi2Sck> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s2Ck> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PF2: Pin<Pf2Id, GpiofId> = Pin { port: GPIOF, index: 2, id: Pf2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf2Id {}
pub type Pf2 = Pin<Pf2Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch3> for Pf2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA2> for Pf2Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF3: Pin<Pf3Id, GpiofId> = Pin { port: GPIOF, index: 3, id: Pf3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf3Id {}
pub type Pf3 = Pin<Pf3Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch4> for Pf3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA3> for Pf3Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF4: Pin<Pf4Id, GpiofId> = Pin { port: GPIOF, index: 4, id: Pf4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf4Id {}
pub type Pf4 = Pin<Pf4Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Comp1Out> for Pf4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim20Ch1n> for Pf4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FmcA4> for Pf4Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF5: Pin<Pf5Id, GpiofId> = Pin { port: GPIOF, index: 5, id: Pf5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf5Id {}
pub type Pf5 = Pin<Pf5Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch2n> for Pf5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA5> for Pf5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF6: Pin<Pf6Id, GpiofId> = Pin { port: GPIOF, index: 6, id: Pf6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf6Id {}
pub type Pf6 = Pin<Pf6Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim4Ch4> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c2Scl> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Usart3Rts> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::FmcNiord> for Pf6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF7: Pin<Pf7Id, GpiofId> = Pin { port: GPIOF, index: 7, id: Pf7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf7Id {}
pub type Pf7 = Pin<Pf7Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Bkin> for Pf7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcNreg> for Pf7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF8: Pin<Pf8Id, GpiofId> = Pin { port: GPIOF, index: 8, id: Pf8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf8Id {}
pub type Pf8 = Pin<Pf8Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Bkin2> for Pf8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcNiowr> for Pf8Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF9: Pin<Pf9Id, GpiofId> = Pin { port: GPIOF, index: 9, id: Pf9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf9Id {}
pub type Pf9 = Pin<Pf9Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Bkin> for Pf9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim15Ch1> for Pf9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Sck> for Pf9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcCd> for Pf9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF10: Pin<Pf10Id, GpiofId> = Pin { port: GPIOF, index: 10, id: Pf10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf10Id {}
pub type Pf10 = Pin<Pf10Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Bkin2> for Pf10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim15Ch2> for Pf10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi2Sck> for Pf10Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FmcIntr> for Pf10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF11: Pin<Pf11Id, GpiofId> = Pin { port: GPIOF, index: 11, id: Pf11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf11Id {}
pub type Pf11 = Pin<Pf11Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Etr> for Pf11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PF12: Pin<Pf12Id, GpiofId> = Pin { port: GPIOF, index: 12, id: Pf12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf12Id {}
pub type Pf12 = Pin<Pf12Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch1> for Pf12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA6> for Pf12Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF13: Pin<Pf13Id, GpiofId> = Pin { port: GPIOF, index: 13, id: Pf13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf13Id {}
pub type Pf13 = Pin<Pf13Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch2> for Pf13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA7> for Pf13Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF14: Pin<Pf14Id, GpiofId> = Pin { port: GPIOF, index: 14, id: Pf14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf14Id {}
pub type Pf14 = Pin<Pf14Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch3> for Pf14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA8> for Pf14Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PF15: Pin<Pf15Id, GpiofId> = Pin { port: GPIOF, index: 15, id: Pf15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf15Id {}
pub type Pf15 = Pin<Pf15Id, GpiofId>;
impl AltFn<super::sig::EventOut> for Pf15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch4> for Pf15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA9> for Pf15Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG0: Pin<Pg0Id, GpiogId> = Pin { port: GPIOG, index: 0, id: Pg0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg0Id {}
pub type Pg0 = Pin<Pg0Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch1n> for Pg0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA10> for Pg0Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG1: Pin<Pg1Id, GpiogId> = Pin { port: GPIOG, index: 1, id: Pg1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg1Id {}
pub type Pg1 = Pin<Pg1Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch2n> for Pg1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA11> for Pg1Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG2: Pin<Pg2Id, GpiogId> = Pin { port: GPIOG, index: 2, id: Pg2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg2Id {}
pub type Pg2 = Pin<Pg2Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch3n> for Pg2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA12> for Pg2Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG3: Pin<Pg3Id, GpiogId> = Pin { port: GPIOG, index: 3, id: Pg3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg3Id {}
pub type Pg3 = Pin<Pg3Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Bkin> for Pg3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA13> for Pg3Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG4: Pin<Pg4Id, GpiogId> = Pin { port: GPIOG, index: 4, id: Pg4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg4Id {}
pub type Pg4 = Pin<Pg4Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Bkin2> for Pg4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA14> for Pg4Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG5: Pin<Pg5Id, GpiogId> = Pin { port: GPIOG, index: 5, id: Pg5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg5Id {}
pub type Pg5 = Pin<Pg5Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Etr> for Pg5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA15> for Pg5Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG6: Pin<Pg6Id, GpiogId> = Pin { port: GPIOG, index: 6, id: Pg6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg6Id {}
pub type Pg6 = Pin<Pg6Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcInt2> for Pg6Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG7: Pin<Pg7Id, GpiogId> = Pin { port: GPIOG, index: 7, id: Pg7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg7Id {}
pub type Pg7 = Pin<Pg7Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcInt3> for Pg7Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG8: Pin<Pg8Id, GpiogId> = Pin { port: GPIOG, index: 8, id: Pg8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg8Id {}
pub type Pg8 = Pin<Pg8Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PG9: Pin<Pg9Id, GpiogId> = Pin { port: GPIOG, index: 9, id: Pg9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg9Id {}
pub type Pg9 = Pin<Pg9Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcNe2> for Pg9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::FmcNce3> for Pg9Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG10: Pin<Pg10Id, GpiogId> = Pin { port: GPIOG, index: 10, id: Pg10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg10Id {}
pub type Pg10 = Pin<Pg10Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcNce41> for Pg10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

impl AltFn<super::sig::FmcNe3> for Pg10Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG11: Pin<Pg11Id, GpiogId> = Pin { port: GPIOG, index: 11, id: Pg11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg11Id {}
pub type Pg11 = Pin<Pg11Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Fmc42> for Pg11Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG12: Pin<Pg12Id, GpiogId> = Pin { port: GPIOG, index: 12, id: Pg12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg12Id {}
pub type Pg12 = Pin<Pg12Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcNe4> for Pg12Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG13: Pin<Pg13Id, GpiogId> = Pin { port: GPIOG, index: 13, id: Pg13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg13Id {}
pub type Pg13 = Pin<Pg13Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcA24> for Pg13Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG14: Pin<Pg14Id, GpiogId> = Pin { port: GPIOG, index: 14, id: Pg14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg14Id {}
pub type Pg14 = Pin<Pg14Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FmcA25> for Pg14Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PG15: Pin<Pg15Id, GpiogId> = Pin { port: GPIOG, index: 15, id: Pg15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg15Id {}
pub type Pg15 = Pin<Pg15Id, GpiogId>;
impl AltFn<super::sig::EventOut> for Pg15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PH0: Pin<Ph0Id, GpiohId> = Pin { port: GPIOH, index: 0, id: Ph0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ph0Id {}
pub type Ph0 = Pin<Ph0Id, GpiohId>;
impl AltFn<super::sig::EventOut> for Ph0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch1> for Ph0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA0> for Ph0Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PH1: Pin<Ph1Id, GpiohId> = Pin { port: GPIOH, index: 1, id: Ph1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ph1Id {}
pub type Ph1 = Pin<Ph1Id, GpiohId>;
impl AltFn<super::sig::EventOut> for Ph1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim20Ch2> for Ph1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FmcA1> for Ph1Id {
   #[inline] fn alt_fn(&self) -> usize { 12 }
}

pub const PH2: Pin<Ph2Id, GpiohId> = Pin { port: GPIOH, index: 2, id: Ph2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ph2Id {}
pub type Ph2 = Pin<Ph2Id, GpiohId>;
impl AltFn<super::sig::EventOut> for Ph2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

