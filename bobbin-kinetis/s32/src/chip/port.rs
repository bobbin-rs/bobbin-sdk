#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::port::*;

pub trait LinkGpio<T> {
   fn gpio(&self) -> T;
}

pub const PORTA: Porta = Periph(0x40049000, PortaId {});
pub const PORTB: Portb = Periph(0x4004a000, PortbId {});
pub const PORTC: Portc = Periph(0x4004b000, PortcId {});
pub const PORTD: Portd = Periph(0x4004c000, PortdId {});
pub const PORTE: Porte = Periph(0x4004d000, PorteId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct PortaId {}
pub type Porta = Periph<PortaId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct PortbId {}
pub type Portb = Periph<PortbId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct PortcId {}
pub type Portc = Periph<PortcId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct PortdId {}
pub type Portd = Periph<PortdId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct PorteId {}
pub type Porte = Periph<PorteId>;

impl LinkGpio<super::gpio::Periph<super::gpio::GpioaId>> for Porta {
   fn gpio(&self) -> super::gpio::Periph<super::gpio::GpioaId> { super::gpio::GPIOA }
}


impl LinkGpio<super::gpio::Periph<super::gpio::GpiobId>> for Portb {
   fn gpio(&self) -> super::gpio::Periph<super::gpio::GpiobId> { super::gpio::GPIOB }
}


impl LinkGpio<super::gpio::Periph<super::gpio::GpiocId>> for Portc {
   fn gpio(&self) -> super::gpio::Periph<super::gpio::GpiocId> { super::gpio::GPIOC }
}


impl LinkGpio<super::gpio::Periph<super::gpio::GpiodId>> for Portd {
   fn gpio(&self) -> super::gpio::Periph<super::gpio::GpiodId> { super::gpio::GPIOD }
}


impl LinkGpio<super::gpio::Periph<super::gpio::GpioeId>> for Porte {
   fn gpio(&self) -> super::gpio::Periph<super::gpio::GpioeId> { super::gpio::GPIOE }
}



pub const PTA0: Pin<Pta0Id, PortaId> = Pin { port: PORTA, index: 0, id: Pta0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta0Id {}
pub type Pta0 = Pin<Pta0Id, PortaId>;
impl AltFn<super::sig::Adc0Se0> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In0> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta0> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch1> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Scls> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD2> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm2QdPha> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Lpuart0Cts> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut3> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA1: Pin<Pta1Id, PortaId> = Pin { port: PORTA, index: 1, id: Pta1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta1Id {}
pub type Pta1 = Pin<Pta1Id, PortaId>;
impl AltFn<super::sig::Adc0Se1> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In1> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta1> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch1> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Sdas> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD3> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPha> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Lpuart0Rts> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut0> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA2: Pin<Pta2Id, PortaId> = Pin { port: PORTA, index: 2, id: Pta2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta2Id {}
pub type Pta2 = Pin<Pta2Id, PortaId>;
impl AltFn<super::sig::Adc1Se0> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta2> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch0> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Sda> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::EwmOutB> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD4> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Lpuart0Rx> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA3: Pin<Pta3Id, PortaId> = Pin { port: PORTA, index: 3, id: Pta3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta3Id {}
pub type Pta3 = Pin<Pta3Id, PortaId>;
impl AltFn<super::sig::Adc1Se1> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta3> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch1> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Scl> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::EwmIn> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD5> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Lpuart0Tx> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA4: Pin<Pta4Id, PortaId> = Pin { port: PORTA, index: 4, id: Pta4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta4Id {}
pub type Pta4 = Pin<Pta4Id, PortaId>;
impl AltFn<super::sig::Pta4> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Cmp0Out> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::EwmOutB> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::JtagTms> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::SwdDio> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA5: Pin<Pta5Id, PortaId> = Pin { port: PORTA, index: 5, id: Pta5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta5Id {}
pub type Pta5 = Pin<Pta5Id, PortaId>;
impl AltFn<super::sig::Pta5> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tclk1> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::ResetB> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA6: Pin<Pta6Id, PortaId> = Pin { port: PORTA, index: 6, id: Pta6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta6Id {}
pub type Pta6 = Pin<Pta6Id, PortaId>;
impl AltFn<super::sig::Adc0Se2> for Pta6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta6> for Pta6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt1> for Pta6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Pcs1> for Pta6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart1Cts> for Pta6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA7: Pin<Pta7Id, PortaId> = Pin { port: PORTA, index: 7, id: Pta7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta7Id {}
pub type Pta7 = Pin<Pta7Id, PortaId>;
impl AltFn<super::sig::Adc0Se3> for Pta7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta7> for Pta7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt2> for Pta7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::RtcClkin> for Pta7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lpuart1Rts> for Pta7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA8: Pin<Pta8Id, PortaId> = Pin { port: PORTA, index: 8, id: Pta8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta8Id {}
pub type Pta8 = Pin<Pta8Id, PortaId>;
impl AltFn<super::sig::Pta8> for Pta8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart2Rx> for Pta8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sout> for Pta8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD6> for Pta8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Flt3> for Pta8Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTA9: Pin<Pta9Id, PortaId> = Pin { port: PORTA, index: 9, id: Pta9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta9Id {}
pub type Pta9 = Pin<Pta9Id, PortaId>;
impl AltFn<super::sig::Pta9> for Pta9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart2Tx> for Pta9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Pcs0> for Pta9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD7> for Pta9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Flt2> for Pta9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm1Flt3> for Pta9Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA10: Pin<Pta10Id, PortaId> = Pin { port: PORTA, index: 10, id: Pta10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta10Id {}
pub type Pta10 = Pin<Pta10Id, PortaId>;
impl AltFn<super::sig::Pta10> for Pta10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch4> for Pta10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FxioD0> for Pta10Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::JtagTdo> for Pta10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::NoetmTraceSwo> for Pta10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA11: Pin<Pta11Id, PortaId> = Pin { port: PORTA, index: 11, id: Pta11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta11Id {}
pub type Pta11 = Pin<Pta11Id, PortaId>;
impl AltFn<super::sig::Pta11> for Pta11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch5> for Pta11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FxioD1> for Pta11Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp0Rrt> for Pta11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTA12: Pin<Pta12Id, PortaId> = Pin { port: PORTA, index: 12, id: Pta12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta12Id {}
pub type Pta12 = Pin<Pta12Id, PortaId>;
impl AltFn<super::sig::Pta12> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch6> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can1Rx> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA13: Pin<Pta13Id, PortaId> = Pin { port: PORTA, index: 13, id: Pta13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta13Id {}
pub type Pta13 = Pin<Pta13Id, PortaId>;
impl AltFn<super::sig::Pta13> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch7> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can1Tx> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2QdPha> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA14: Pin<Pta14Id, PortaId> = Pin { port: PORTA, index: 14, id: Pta14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta14Id {}
pub type Pta14 = Pin<Pta14Id, PortaId>;
impl AltFn<super::sig::Pta14> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt0> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt1> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::EwmIn> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1Flt0> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA15: Pin<Pta15Id, PortaId> = Pin { port: PORTA, index: 15, id: Pta15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta15Id {}
pub type Pta15 = Pin<Pta15Id, PortaId>;
impl AltFn<super::sig::Adc1Se12> for Pta15Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta15> for Pta15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch2> for Pta15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Pcs3> for Pta15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpspi2Pcs3> for Pta15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTA16: Pin<Pta16Id, PortaId> = Pin { port: PORTA, index: 16, id: Pta16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta16Id {}
pub type Pta16 = Pin<Pta16Id, PortaId>;
impl AltFn<super::sig::Adc1Se13> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta16> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch3> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Pcs2> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTA17: Pin<Pta17Id, PortaId> = Pin { port: PORTA, index: 17, id: Pta17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta17Id {}
pub type Pta17 = Pin<Pta17Id, PortaId>;
impl AltFn<super::sig::Pta17> for Pta17Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch6> for Pta17Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt0> for Pta17Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::EwmOutB> for Pta17Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTB0: Pin<Ptb0Id, PortbId> = Pin { port: PORTB, index: 0, id: Ptb0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb0Id {}
pub type Ptb0 = Pin<Ptb0Id, PortbId>;
impl AltFn<super::sig::Adc0Se4> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se14> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb0> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart0Rx> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Pcs0> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lptmr0Alt3> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Can0Rx> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTB1: Pin<Ptb1Id, PortbId> = Pin { port: PORTB, index: 1, id: Ptb1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb1Id {}
pub type Ptb1 = Pin<Ptb1Id, PortbId>;
impl AltFn<super::sig::Adc0Se5> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se15> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb1> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart0Tx> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sout> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tclk0> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Can0Tx> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTB2: Pin<Ptb2Id, PortbId> = Pin { port: PORTB, index: 2, id: Ptb2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb2Id {}
pub type Ptb2 = Pin<Ptb2Id, PortbId>;
impl AltFn<super::sig::Adc0Se6> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb2> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch0> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sck> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TrgmuxIn3> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB3: Pin<Ptb3Id, PortbId> = Pin { port: PORTB, index: 3, id: Ptb3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb3Id {}
pub type Ptb3 = Pin<Ptb3Id, PortbId>;
impl AltFn<super::sig::Adc0Se7> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb3> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch1> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sin> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm1QdPha> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TrgmuxIn2> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB4: Pin<Ptb4Id, PortbId> = Pin { port: PORTB, index: 4, id: Ptb4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb4Id {}
pub type Ptb4 = Pin<Ptb4Id, PortbId>;
impl AltFn<super::sig::Ptb4> for Ptb4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch4> for Ptb4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sout> for Ptb4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TrgmuxIn1> for Ptb4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB5: Pin<Ptb5Id, PortbId> = Pin { port: PORTB, index: 5, id: Ptb5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb5Id {}
pub type Ptb5 = Pin<Ptb5Id, PortbId>;
impl AltFn<super::sig::Ptb5> for Ptb5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch5> for Ptb5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Pcs1> for Ptb5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpspi0Pcs0> for Ptb5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Clkout> for Ptb5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::TrgmuxIn0> for Ptb5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB6: Pin<Ptb6Id, PortbId> = Pin { port: PORTB, index: 6, id: Ptb6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb6Id {}
pub type Ptb6 = Pin<Ptb6Id, PortbId>;
impl AltFn<super::sig::Xtal> for Ptb6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb6> for Ptb6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpi2c0Sda> for Ptb6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PTB7: Pin<Ptb7Id, PortbId> = Pin { port: PORTB, index: 7, id: Ptb7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb7Id {}
pub type Ptb7 = Pin<Ptb7Id, PortbId>;
impl AltFn<super::sig::Extal> for Ptb7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb7> for Ptb7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpi2c0Scl> for Ptb7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PTB8: Pin<Ptb8Id, PortbId> = Pin { port: PORTB, index: 8, id: Ptb8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb8Id {}
pub type Ptb8 = Pin<Ptb8Id, PortbId>;
impl AltFn<super::sig::Ptb8> for Ptb8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch0> for Ptb8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PTB9: Pin<Ptb9Id, PortbId> = Pin { port: PORTB, index: 9, id: Ptb9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb9Id {}
pub type Ptb9 = Pin<Ptb9Id, PortbId>;
impl AltFn<super::sig::Ptb9> for Ptb9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch1> for Ptb9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Scls> for Ptb9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB10: Pin<Ptb10Id, PortbId> = Pin { port: PORTB, index: 10, id: Ptb10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb10Id {}
pub type Ptb10 = Pin<Ptb10Id, PortbId>;
impl AltFn<super::sig::Ptb10> for Ptb10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch2> for Ptb10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Sdas> for Ptb10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB11: Pin<Ptb11Id, PortbId> = Pin { port: PORTB, index: 11, id: Ptb11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb11Id {}
pub type Ptb11 = Pin<Ptb11Id, PortbId>;
impl AltFn<super::sig::Ptb11> for Ptb11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch3> for Ptb11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Hreq> for Ptb11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB12: Pin<Ptb12Id, PortbId> = Pin { port: PORTB, index: 12, id: Ptb12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb12Id {}
pub type Ptb12 = Pin<Ptb12Id, PortbId>;
impl AltFn<super::sig::Adc1Se7> for Ptb12Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb12> for Ptb12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch0> for Ptb12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt2> for Ptb12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can2Rx> for Ptb12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTB13: Pin<Ptb13Id, PortbId> = Pin { port: PORTB, index: 13, id: Ptb13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb13Id {}
pub type Ptb13 = Pin<Ptb13Id, PortbId>;
impl AltFn<super::sig::Adc1Se8> for Ptb13Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se8> for Ptb13Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb13> for Ptb13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch1> for Ptb13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt1> for Ptb13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can2Tx> for Ptb13Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTB14: Pin<Ptb14Id, PortbId> = Pin { port: PORTB, index: 14, id: Ptb14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb14Id {}
pub type Ptb14 = Pin<Ptb14Id, PortbId>;
impl AltFn<super::sig::Adc1Se9> for Ptb14Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se9> for Ptb14Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb14> for Ptb14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch2> for Ptb14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sck> for Ptb14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB15: Pin<Ptb15Id, PortbId> = Pin { port: PORTB, index: 15, id: Ptb15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb15Id {}
pub type Ptb15 = Pin<Ptb15Id, PortbId>;
impl AltFn<super::sig::Adc1Se14> for Ptb15Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb15> for Ptb15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch3> for Ptb15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sin> for Ptb15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB16: Pin<Ptb16Id, PortbId> = Pin { port: PORTB, index: 16, id: Ptb16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb16Id {}
pub type Ptb16 = Pin<Ptb16Id, PortbId>;
impl AltFn<super::sig::Adc1Se15> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb16> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch4> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sout> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB17: Pin<Ptb17Id, PortbId> = Pin { port: PORTB, index: 17, id: Ptb17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb17Id {}
pub type Ptb17 = Pin<Ptb17Id, PortbId>;
impl AltFn<super::sig::Ptb17> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch5> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Pcs3> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTC0: Pin<Ptc0Id, PortcId> = Pin { port: PORTC, index: 0, id: Ptc0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc0Id {}
pub type Ptc0 = Pin<Ptc0Id, PortcId>;
impl AltFn<super::sig::Adc0Se8> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc0> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch0> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sin> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm1Ch6> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC1: Pin<Ptc1Id, PortcId> = Pin { port: PORTC, index: 1, id: Ptc1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc1Id {}
pub type Ptc1 = Pin<Ptc1Id, PortcId>;
impl AltFn<super::sig::Adc0Se9> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc1> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch1> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sout> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm1Ch7> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC2: Pin<Ptc2Id, PortcId> = Pin { port: PORTC, index: 2, id: Ptc2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc2Id {}
pub type Ptc2 = Pin<Ptc2Id, PortcId>;
impl AltFn<super::sig::Adc0Se10> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In5> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc2> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch2> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can0Rx> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart0Rx> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC3: Pin<Ptc3Id, PortcId> = Pin { port: PORTC, index: 3, id: Ptc3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc3Id {}
pub type Ptc3 = Pin<Ptc3Id, PortcId>;
impl AltFn<super::sig::Adc0Se11> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In4> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc3> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch3> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can0Tx> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart0Tx> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC4: Pin<Ptc4Id, PortcId> = Pin { port: PORTC, index: 4, id: Ptc4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc4Id {}
pub type Ptc4 = Pin<Ptc4Id, PortcId>;
impl AltFn<super::sig::Cmp0In2> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc4> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch0> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::RtcClkout> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::EwmIn> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::JtagTclk> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::SwdClk> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTC5: Pin<Ptc5Id, PortcId> = Pin { port: PORTC, index: 5, id: Ptc5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc5Id {}
pub type Ptc5 = Pin<Ptc5Id, PortcId>;
impl AltFn<super::sig::Ptc5> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch0> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::RtcClkout> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::JtagTdi> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTC6: Pin<Ptc6Id, PortcId> = Pin { port: PORTC, index: 6, id: Ptc6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc6Id {}
pub type Ptc6 = Pin<Ptc6Id, PortcId>;
impl AltFn<super::sig::Adc1Se4> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc6> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Rx> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can1Rx> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch2> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC7: Pin<Ptc7Id, PortcId> = Pin { port: PORTC, index: 7, id: Ptc7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc7Id {}
pub type Ptc7 = Pin<Ptc7Id, PortcId>;
impl AltFn<super::sig::Adc1Se5> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc7> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Tx> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can1Tx> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch3> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPha> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC8: Pin<Ptc8Id, PortcId> = Pin { port: PORTC, index: 8, id: Ptc8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc8Id {}
pub type Ptc8 = Pin<Ptc8Id, PortcId>;
impl AltFn<super::sig::Ptc8> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Rx> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Flt0> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart0Cts> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC9: Pin<Ptc9Id, PortcId> = Pin { port: PORTC, index: 9, id: Ptc9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc9Id {}
pub type Ptc9 = Pin<Ptc9Id, PortcId>;
impl AltFn<super::sig::Ptc9> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Tx> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Flt1> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart0Rts> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC10: Pin<Ptc10Id, PortcId> = Pin { port: PORTC, index: 10, id: Ptc10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc10Id {}
pub type Ptc10 = Pin<Ptc10Id, PortcId>;
impl AltFn<super::sig::Ptc10> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch4> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TrgmuxIn11> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC11: Pin<Ptc11Id, PortcId> = Pin { port: PORTC, index: 11, id: Ptc11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc11Id {}
pub type Ptc11 = Pin<Ptc11Id, PortcId>;
impl AltFn<super::sig::Ptc11> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch5> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::TrgmuxIn10> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC12: Pin<Ptc12Id, PortcId> = Pin { port: PORTC, index: 12, id: Ptc12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc12Id {}
pub type Ptc12 = Pin<Ptc12Id, PortcId>;
impl AltFn<super::sig::Ptc12> for Ptc12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch6> for Ptc12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Ch6> for Ptc12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart2Cts> for Ptc12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC13: Pin<Ptc13Id, PortcId> = Pin { port: PORTC, index: 13, id: Ptc13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc13Id {}
pub type Ptc13 = Pin<Ptc13Id, PortcId>;
impl AltFn<super::sig::Ptc13> for Ptc13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch7> for Ptc13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Ch7> for Ptc13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart2Rts> for Ptc13Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC14: Pin<Ptc14Id, PortcId> = Pin { port: PORTC, index: 14, id: Ptc14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc14Id {}
pub type Ptc14 = Pin<Ptc14Id, PortcId>;
impl AltFn<super::sig::Adc0Se12> for Ptc14Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc14> for Ptc14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch2> for Ptc14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Pcs0> for Ptc14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TrgmuxIn9> for Ptc14Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC15: Pin<Ptc15Id, PortcId> = Pin { port: PORTC, index: 15, id: Ptc15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc15Id {}
pub type Ptc15 = Pin<Ptc15Id, PortcId>;
impl AltFn<super::sig::Adc0Se13> for Ptc15Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc15> for Ptc15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch3> for Ptc15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sck> for Ptc15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TrgmuxIn8> for Ptc15Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC16: Pin<Ptc16Id, PortcId> = Pin { port: PORTC, index: 16, id: Ptc16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc16Id {}
pub type Ptc16 = Pin<Ptc16Id, PortcId>;
impl AltFn<super::sig::Adc0Se14> for Ptc16Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc16> for Ptc16Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Flt2> for Ptc16Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can2Rx> for Ptc16Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTC17: Pin<Ptc17Id, PortcId> = Pin { port: PORTC, index: 17, id: Ptc17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc17Id {}
pub type Ptc17 = Pin<Ptc17Id, PortcId>;
impl AltFn<super::sig::Adc0Se15> for Ptc17Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc17> for Ptc17Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Flt3> for Ptc17Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Can2Tx> for Ptc17Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTD0: Pin<Ptd0Id, PortdId> = Pin { port: PORTD, index: 0, id: Ptd0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd0Id {}
pub type Ptd0 = Pin<Ptd0Id, PortdId>;
impl AltFn<super::sig::Ptd0> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch2> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sck> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch0> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD0> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut1> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD1: Pin<Ptd1Id, PortdId> = Pin { port: PORTD, index: 1, id: Ptd1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd1Id {}
pub type Ptd1 = Pin<Ptd1Id, PortdId>;
impl AltFn<super::sig::Ptd1> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch3> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sin> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch1> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD1> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut2> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD2: Pin<Ptd2Id, PortdId> = Pin { port: PORTD, index: 2, id: Ptd2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd2Id {}
pub type Ptd2 = Pin<Ptd2Id, PortdId>;
impl AltFn<super::sig::Adc1Se2> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd2> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch4> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Sout> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD4> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD6> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::TrgmuxIn5> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD3: Pin<Ptd3Id, PortdId> = Pin { port: PORTD, index: 3, id: Ptd3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd3Id {}
pub type Ptd3 = Pin<Ptd3Id, PortdId>;
impl AltFn<super::sig::Adc1Se3> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd3> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch5> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi1Pcs0> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FxioD5> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD7> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::TrgmuxIn4> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::NmiB> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD4: Pin<Ptd4Id, PortdId> = Pin { port: PORTD, index: 4, id: Ptd4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd4Id {}
pub type Ptd4 = Pin<Ptd4Id, PortdId>;
impl AltFn<super::sig::Adc1Se6> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd4> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt3> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt3> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTD5: Pin<Ptd5Id, PortdId> = Pin { port: PORTD, index: 5, id: Ptd5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd5Id {}
pub type Ptd5 = Pin<Ptd5Id, PortdId>;
impl AltFn<super::sig::Ptd5> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch3> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lptmr0Alt2> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Flt1> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TrgmuxIn7> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD6: Pin<Ptd6Id, PortdId> = Pin { port: PORTD, index: 6, id: Ptd6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd6Id {}
pub type Ptd6 = Pin<Ptd6Id, PortdId>;
impl AltFn<super::sig::Cmp0In7> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd6> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart2Rx> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Flt2> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTD7: Pin<Ptd7Id, PortdId> = Pin { port: PORTD, index: 7, id: Ptd7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd7Id {}
pub type Ptd7 = Pin<Ptd7Id, PortdId>;
impl AltFn<super::sig::Cmp0In6> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd7> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart2Tx> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Flt3> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTD8: Pin<Ptd8Id, PortdId> = Pin { port: PORTD, index: 8, id: Ptd8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd8Id {}
pub type Ptd8 = Pin<Ptd8Id, PortdId>;
impl AltFn<super::sig::Ptd8> for Ptd8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Flt2> for Ptd8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD1> for Ptd8Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm1Ch4> for Ptd8Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD9: Pin<Ptd9Id, PortdId> = Pin { port: PORTD, index: 9, id: Ptd9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd9Id {}
pub type Ptd9 = Pin<Ptd9Id, PortdId>;
impl AltFn<super::sig::Ptd9> for Ptd9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FxioD0> for Ptd9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Flt3> for Ptd9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1Ch5> for Ptd9Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD10: Pin<Ptd10Id, PortdId> = Pin { port: PORTD, index: 10, id: Ptd10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd10Id {}
pub type Ptd10 = Pin<Ptd10Id, PortdId>;
impl AltFn<super::sig::Ptd10> for Ptd10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch0> for Ptd10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Ptd10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTD11: Pin<Ptd11Id, PortdId> = Pin { port: PORTD, index: 11, id: Ptd11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd11Id {}
pub type Ptd11 = Pin<Ptd11Id, PortdId>;
impl AltFn<super::sig::Ptd11> for Ptd11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch1> for Ptd11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2QdPha> for Ptd11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpuart2Cts> for Ptd11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD12: Pin<Ptd12Id, PortdId> = Pin { port: PORTD, index: 12, id: Ptd12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd12Id {}
pub type Ptd12 = Pin<Ptd12Id, PortdId>;
impl AltFn<super::sig::Ptd12> for Ptd12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch2> for Ptd12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart2Rts> for Ptd12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD13: Pin<Ptd13Id, PortdId> = Pin { port: PORTD, index: 13, id: Ptd13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd13Id {}
pub type Ptd13 = Pin<Ptd13Id, PortdId>;
impl AltFn<super::sig::Ptd13> for Ptd13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch4> for Ptd13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart1Rx> for Ptd13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::RtcClkout> for Ptd13Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD14: Pin<Ptd14Id, PortdId> = Pin { port: PORTD, index: 14, id: Ptd14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd14Id {}
pub type Ptd14 = Pin<Ptd14Id, PortdId>;
impl AltFn<super::sig::Ptd14> for Ptd14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch5> for Ptd14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart1Tx> for Ptd14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Clkout> for Ptd14Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD15: Pin<Ptd15Id, PortdId> = Pin { port: PORTD, index: 15, id: Ptd15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd15Id {}
pub type Ptd15 = Pin<Ptd15Id, PortdId>;
impl AltFn<super::sig::Ptd15> for Ptd15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch0> for Ptd15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sck> for Ptd15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTD16: Pin<Ptd16Id, PortdId> = Pin { port: PORTD, index: 16, id: Ptd16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd16Id {}
pub type Ptd16 = Pin<Ptd16Id, PortdId>;
impl AltFn<super::sig::Ptd16> for Ptd16Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch1> for Ptd16Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi0Sin> for Ptd16Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp0Rrt> for Ptd16Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTD17: Pin<Ptd17Id, PortdId> = Pin { port: PORTD, index: 17, id: Ptd17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd17Id {}
pub type Ptd17 = Pin<Ptd17Id, PortdId>;
impl AltFn<super::sig::Ptd17> for Ptd17Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt2> for Ptd17Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart2Rx> for Ptd17Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTE0: Pin<Pte0Id, PorteId> = Pin { port: PORTE, index: 0, id: Pte0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte0Id {}
pub type Pte0 = Pin<Pte0Id, PorteId>;
impl AltFn<super::sig::Pte0> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi0Sck> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tclk1> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpspi1Sout> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm1Flt2> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE1: Pin<Pte1Id, PorteId> = Pin { port: PORTE, index: 1, id: Pte1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte1Id {}
pub type Pte1 = Pin<Pte1Id, PorteId>;
impl AltFn<super::sig::Pte1> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi0Sin> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpi2c0Hreq> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Lpspi1Pcs0> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm1Flt1> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE2: Pin<Pte2Id, PorteId> = Pin { port: PORTE, index: 2, id: Pte2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte2Id {}
pub type Pte2 = Pin<Pte2Id, PorteId>;
impl AltFn<super::sig::Adc1Se10> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte2> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi0Sout> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lptmr0Alt3> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch6> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lpuart1Cts> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE3: Pin<Pte3Id, PorteId> = Pin { port: PORTE, index: 3, id: Pte3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte3Id {}
pub type Pte3 = Pin<Pte3Id, PorteId>;
impl AltFn<super::sig::Pte3> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt0> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart2Rts> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Flt0> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TrgmuxIn6> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Cmp0Out> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE4: Pin<Pte4Id, PorteId> = Pin { port: PORTE, index: 4, id: Pte4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte4Id {}
pub type Pte4 = Pin<Pte4Id, PorteId>;
impl AltFn<super::sig::Pte4> for Pte4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Pte4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch2> for Pte4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Can0Rx> for Pte4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FxioD6> for Pte4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::EwmOutB> for Pte4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE5: Pin<Pte5Id, PorteId> = Pin { port: PORTE, index: 5, id: Pte5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte5Id {}
pub type Pte5 = Pin<Pte5Id, PorteId>;
impl AltFn<super::sig::Pte5> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tclk2> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2QdPha> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch3> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Can0Tx> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FxioD7> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::EwmIn> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE6: Pin<Pte6Id, PorteId> = Pin { port: PORTE, index: 6, id: Pte6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte6Id {}
pub type Pte6 = Pin<Pte6Id, PorteId>;
impl AltFn<super::sig::Adc1Se11> for Pte6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte6> for Pte6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi0Pcs2> for Pte6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Ch7> for Pte6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lpuart1Rts> for Pte6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE7: Pin<Pte7Id, PorteId> = Pin { port: PORTE, index: 7, id: Pte7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte7Id {}
pub type Pte7 = Pin<Pte7Id, PorteId>;
impl AltFn<super::sig::Pte7> for Pte7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch7> for Pte7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt0> for Pte7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTE8: Pin<Pte8Id, PorteId> = Pin { port: PORTE, index: 8, id: Pte8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte8Id {}
pub type Pte8 = Pin<Pte8Id, PorteId>;
impl AltFn<super::sig::Cmp0In3> for Pte8Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte8> for Pte8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch6> for Pte8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

pub const PTE9: Pin<Pte9Id, PorteId> = Pin { port: PORTE, index: 9, id: Pte9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte9Id {}
pub type Pte9 = Pin<Pte9Id, PorteId>;
impl AltFn<super::sig::Pte9> for Pte9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch7> for Pte9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart2Cts> for Pte9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTE10: Pin<Pte10Id, PorteId> = Pin { port: PORTE, index: 10, id: Pte10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte10Id {}
pub type Pte10 = Pin<Pte10Id, PorteId>;
impl AltFn<super::sig::Pte10> for Pte10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Clkout> for Pte10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Pcs1> for Pte10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch4> for Pte10Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD4> for Pte10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut4> for Pte10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE11: Pin<Pte11Id, PorteId> = Pin { port: PORTE, index: 11, id: Pte11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte11Id {}
pub type Pte11 = Pin<Pte11Id, PorteId>;
impl AltFn<super::sig::Pte11> for Pte11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi2Pcs0> for Pte11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lptmr0Alt1> for Pte11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch5> for Pte11Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD5> for Pte11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut5> for Pte11Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE12: Pin<Pte12Id, PorteId> = Pin { port: PORTE, index: 12, id: Pte12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte12Id {}
pub type Pte12 = Pin<Pte12Id, PorteId>;
impl AltFn<super::sig::Pte12> for Pte12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt3> for Pte12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpuart2Tx> for Pte12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTE13: Pin<Pte13Id, PorteId> = Pin { port: PORTE, index: 13, id: Pte13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte13Id {}
pub type Pte13 = Pin<Pte13Id, PorteId>;
impl AltFn<super::sig::Pte13> for Pte13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpspi2Pcs2> for Pte13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Flt0> for Pte13Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE14: Pin<Pte14Id, PorteId> = Pin { port: PORTE, index: 14, id: Pte14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte14Id {}
pub type Pte14 = Pin<Pte14Id, PorteId>;
impl AltFn<super::sig::Pte14> for Pte14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt1> for Pte14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Flt1> for Pte14Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE15: Pin<Pte15Id, PorteId> = Pin { port: PORTE, index: 15, id: Pte15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte15Id {}
pub type Pte15 = Pin<Pte15Id, PorteId>;
impl AltFn<super::sig::Pte15> for Pte15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Cts> for Pte15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sck> for Pte15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch6> for Pte15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD2> for Pte15Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut6> for Pte15Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE16: Pin<Pte16Id, PorteId> = Pin { port: PORTE, index: 16, id: Pte16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte16Id {}
pub type Pte16 = Pin<Pte16Id, PorteId>;
impl AltFn<super::sig::Pte16> for Pte16Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Lpuart1Rts> for Pte16Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lpspi2Sin> for Pte16Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm2Ch7> for Pte16Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FxioD3> for Pte16Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TrgmuxOut7> for Pte16Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub trait IrqPort<T> {
   fn irq_port(&self) -> super::irq::Irq<T>;
}

pub trait RegisterPortHandler {
   fn register_port_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandlePort>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandlePort {
   fn handle_port(&self);
}

impl IrqPort<super::irq::PortaId> for Porta {
   fn irq_port(&self) -> super::irq::IrqPorta { super::irq::IRQ_PORTA }
}

impl RegisterPortHandler for Porta {
   fn register_port_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandlePort>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandlePort>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_port() }
       }
       super::irq::set_handler(59, Some(wrapper::<F>));
       super::irq::IrqGuard::new(59)
   }
}

impl IrqPort<super::irq::PortbId> for Portb {
   fn irq_port(&self) -> super::irq::IrqPortb { super::irq::IRQ_PORTB }
}

impl RegisterPortHandler for Portb {
   fn register_port_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandlePort>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandlePort>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_port() }
       }
       super::irq::set_handler(60, Some(wrapper::<F>));
       super::irq::IrqGuard::new(60)
   }
}

impl IrqPort<super::irq::PortcId> for Portc {
   fn irq_port(&self) -> super::irq::IrqPortc { super::irq::IRQ_PORTC }
}

impl RegisterPortHandler for Portc {
   fn register_port_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandlePort>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandlePort>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_port() }
       }
       super::irq::set_handler(61, Some(wrapper::<F>));
       super::irq::IrqGuard::new(61)
   }
}

impl IrqPort<super::irq::PortdId> for Portd {
   fn irq_port(&self) -> super::irq::IrqPortd { super::irq::IRQ_PORTD }
}

impl RegisterPortHandler for Portd {
   fn register_port_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandlePort>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandlePort>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_port() }
       }
       super::irq::set_handler(62, Some(wrapper::<F>));
       super::irq::IrqGuard::new(62)
   }
}

