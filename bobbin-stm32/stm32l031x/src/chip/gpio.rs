pub use stm32_common::chip::gpio::*;

pub const GPIOA: Gpioa = Periph(0x50000000, GpioaId {});
pub const GPIOB: Gpiob = Periph(0x50000400, GpiobId {});
pub const GPIOC: Gpioc = Periph(0x50000800, GpiocId {});
pub const GPIOH: Gpioh = Periph(0x50001c00, GpiohId {});

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
pub struct GpiohId {}
pub type Gpioh = Periph<GpiohId>;






pub const PA0: Pin<Pa0Id, GpioaId> = Pin { port: GPIOA, index: 0, id: Pa0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa0Id {}
pub type Pa0 = Pin<Pa0Id, GpioaId>;
impl AltFn<super::sig::Lptim1In1> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Ch1> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart2Cts> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim2Etr> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Comp1Out> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA1: Pin<Pa1Id, GpioaId> = Pin { port: GPIOA, index: 1, id: Pa1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa1Id {}
pub type Pa1 = Pin<Pa1Id, GpioaId>;
impl AltFn<super::sig::Eventout> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Lptim1In2> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Ch2> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c1Smba> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Rts> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim21Etr> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PA2: Pin<Pa2Id, GpioaId> = Pin { port: GPIOA, index: 2, id: Pa2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa2Id {}
pub type Pa2 = Pin<Pa2Id, GpioaId>;
impl AltFn<super::sig::Tim21Ch1> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch3> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart2Tx> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lpuart1Tx> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Comp2Out> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA3: Pin<Pa3Id, GpioaId> = Pin { port: GPIOA, index: 3, id: Pa3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa3Id {}
pub type Pa3 = Pin<Pa3Id, GpioaId>;
impl AltFn<super::sig::Tim21Ch2> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch4> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart2Rx> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lpuart1Rx> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA4: Pin<Pa4Id, GpioaId> = Pin { port: GPIOA, index: 4, id: Pa4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa4Id {}
pub type Pa4 = Pin<Pa4Id, GpioaId>;
impl AltFn<super::sig::Spi1Nss> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Lptim1In1> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart2Ck> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim22Etr> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PA5: Pin<Pa5Id, GpioaId> = Pin { port: GPIOA, index: 5, id: Pa5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa5Id {}
pub type Pa5 = Pin<Pa5Id, GpioaId>;
impl AltFn<super::sig::Spi1Sck> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Lptim1In2> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim2Etr> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim2Ch1> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PA6: Pin<Pa6Id, GpioaId> = Pin { port: GPIOA, index: 6, id: Pa6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa6Id {}
pub type Pa6 = Pin<Pa6Id, GpioaId>;
impl AltFn<super::sig::SpiMiso> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Lptim1Etr> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Cts> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim22Ch1> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Eventout> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Comp1Out> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA7: Pin<Pa7Id, GpioaId> = Pin { port: GPIOA, index: 7, id: Pa7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa7Id {}
pub type Pa7 = Pin<Pa7Id, GpioaId>;
impl AltFn<super::sig::Spi1Mosi> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Lptim1Out> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart2Cts> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim22Ch2> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Eventout> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Comp2Out> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA8: Pin<Pa8Id, GpioaId> = Pin { port: GPIOA, index: 8, id: Pa8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa8Id {}
pub type Pa8 = Pin<Pa8Id, GpioaId>;
impl AltFn<super::sig::Mco> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Lptim1In1> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Eventout> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Ck> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim2Ch1> for Pa8Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PA9: Pin<Pa9Id, GpioaId> = Pin { port: GPIOA, index: 9, id: Pa9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa9Id {}
pub type Pa9 = Pin<Pa9Id, GpioaId>;
impl AltFn<super::sig::Mco> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c1Scl> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart2Tx> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim22Ch1> for Pa9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PA10: Pin<Pa10Id, GpioaId> = Pin { port: GPIOA, index: 10, id: Pa10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa10Id {}
pub type Pa10 = Pin<Pa10Id, GpioaId>;
impl AltFn<super::sig::I2c1Sda> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart2Rx> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim22Ch2> for Pa10Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PA11: Pin<Pa11Id, GpioaId> = Pin { port: GPIOA, index: 11, id: Pa11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa11Id {}
pub type Pa11 = Pin<Pa11Id, GpioaId>;
impl AltFn<super::sig::Spi1Mio> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Eventout> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart2Cts> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim21Ch2> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Comp1Out> for Pa11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA12: Pin<Pa12Id, GpioaId> = Pin { port: GPIOA, index: 12, id: Pa12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa12Id {}
pub type Pa12 = Pin<Pa12Id, GpioaId>;
impl AltFn<super::sig::Spi1Mosi> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Eventout> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usart2Rts> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Comp2Out> for Pa12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA13: Pin<Pa13Id, GpioaId> = Pin { port: GPIOA, index: 13, id: Pa13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa13Id {}
pub type Pa13 = Pin<Pa13Id, GpioaId>;
impl AltFn<super::sig::Swdio> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Lptim1Etr> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Rx> for Pa13Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA14: Pin<Pa14Id, GpioaId> = Pin { port: GPIOA, index: 14, id: Pa14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa14Id {}
pub type Pa14 = Pin<Pa14Id, GpioaId>;
impl AltFn<super::sig::Swclk> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Lptim1Out> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Smba> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Tx> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lpuart1Tx> for Pa14Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PA15: Pin<Pa15Id, GpioaId> = Pin { port: GPIOA, index: 15, id: Pa15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa15Id {}
pub type Pa15 = Pin<Pa15Id, GpioaId>;
impl AltFn<super::sig::Spi1Nss> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Etr> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Eventout> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usart2Rx> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim2Ch1> for Pa15Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PB0: Pin<Pb0Id, GpiobId> = Pin { port: GPIOB, index: 0, id: Pb0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb0Id {}
pub type Pb0 = Pin<Pb0Id, GpiobId>;
impl AltFn<super::sig::Eventout> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Spi1Miso> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usart2Rts> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim2Ch3> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PB1: Pin<Pb1Id, GpiobId> = Pin { port: GPIOB, index: 1, id: Pb1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb1Id {}
pub type Pb1 = Pin<Pb1Id, GpiobId>;
impl AltFn<super::sig::Usart2Ck> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Spi1Mosi> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Rts> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tim2Ch4> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PB2: Pin<Pb2Id, GpiobId> = Pin { port: GPIOB, index: 2, id: Pb2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb2Id {}
pub type Pb2 = Pin<Pb2Id, GpiobId>;
impl AltFn<super::sig::Lptim1Out> for Pb2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PB3: Pin<Pb3Id, GpiobId> = Pin { port: GPIOB, index: 3, id: Pb3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb3Id {}
pub type Pb3 = Pin<Pb3Id, GpiobId>;
impl AltFn<super::sig::Spi1Sck> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch2> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Eventout> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB4: Pin<Pb4Id, GpiobId> = Pin { port: GPIOB, index: 4, id: Pb4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb4Id {}
pub type Pb4 = Pin<Pb4Id, GpiobId>;
impl AltFn<super::sig::Spi1Miso> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Eventout> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim22Ch1> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB5: Pin<Pb5Id, GpiobId> = Pin { port: GPIOB, index: 5, id: Pb5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb5Id {}
pub type Pb5 = Pin<Pb5Id, GpiobId>;
impl AltFn<super::sig::Spi1Mosi> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Lptim1In1> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c1Smba> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tim22Ch2> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB6: Pin<Pb6Id, GpiobId> = Pin { port: GPIOB, index: 6, id: Pb6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb6Id {}
pub type Pb6 = Pin<Pb6Id, GpiobId>;
impl AltFn<super::sig::Usart2Tx> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c1Scl> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lptim1Etr> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim21Ch1> for Pb6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PB7: Pin<Pb7Id, GpiobId> = Pin { port: GPIOB, index: 7, id: Pb7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb7Id {}
pub type Pb7 = Pin<Pb7Id, GpiobId>;
impl AltFn<super::sig::Usart2Rx> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c1Sda> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lptim1In2> for Pb7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PB8: Pin<Pb8Id, GpiobId> = Pin { port: GPIOB, index: 8, id: Pb8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb8Id {}
pub type Pb8 = Pin<Pb8Id, GpiobId>;
impl AltFn<super::sig::I2c1Scl> for Pb8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB9: Pin<Pb9Id, GpiobId> = Pin { port: GPIOB, index: 9, id: Pb9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb9Id {}
pub type Pb9 = Pin<Pb9Id, GpiobId>;
impl AltFn<super::sig::Eventout> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c1Sda> for Pb9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PB10: Pin<Pb10Id, GpiobId> = Pin { port: GPIOB, index: 10, id: Pb10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb10Id {}
pub type Pb10 = Pin<Pb10Id, GpiobId>;
impl AltFn<super::sig::Tim2Ch3> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart1Tx> for Pb10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PB11: Pin<Pb11Id, GpiobId> = Pin { port: GPIOB, index: 11, id: Pb11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb11Id {}
pub type Pb11 = Pin<Pb11Id, GpiobId>;
impl AltFn<super::sig::Eventout> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tim2Ch4> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart1Rx> for Pb11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PB12: Pin<Pb12Id, GpiobId> = Pin { port: GPIOB, index: 12, id: Pb12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb12Id {}
pub type Pb12 = Pin<Pb12Id, GpiobId>;
impl AltFn<super::sig::Spi1Nss> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Eventout> for Pb12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PB13: Pin<Pb13Id, GpiobId> = Pin { port: GPIOB, index: 13, id: Pb13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb13Id {}
pub type Pb13 = Pin<Pb13Id, GpiobId>;
impl AltFn<super::sig::SpiSck> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Mco> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tim21Ch1> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Lpuart1Cts> for Pb13Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PB14: Pin<Pb14Id, GpiobId> = Pin { port: GPIOB, index: 14, id: Pb14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb14Id {}
pub type Pb14 = Pin<Pb14Id, GpiobId>;
impl AltFn<super::sig::Spi1Miso> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::RtcOut> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tim21Ch2> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Lpuart1Rts> for Pb14Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PB15: Pin<Pb15Id, GpiobId> = Pin { port: GPIOB, index: 15, id: Pb15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb15Id {}
pub type Pb15 = Pin<Pb15Id, GpiobId>;
impl AltFn<super::sig::Spi1Mosi> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::RtcRefin> for Pb15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PC0: Pin<Pc0Id, GpiocId> = Pin { port: GPIOC, index: 0, id: Pc0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc0Id {}
pub type Pc0 = Pin<Pc0Id, GpiocId>;
impl AltFn<super::sig::Lptim1In1> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Eventout> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart1Rx> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
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
pub const PH0: Pin<Ph0Id, GpiohId> = Pin { port: GPIOH, index: 0, id: Ph0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph0Id {}
pub type Ph0 = Pin<Ph0Id, GpiohId>;
pub const PH1: Pin<Ph1Id, GpiohId> = Pin { port: GPIOH, index: 1, id: Ph1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ph1Id {}
pub type Ph1 = Pin<Ph1Id, GpiohId>;
