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
impl AltFn<super::sig::Tsi0Ch1> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta0> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch5> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::SwdClk> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA1: Pin<Pta1Id, PortaId> = Pin { port: PORTA, index: 1, id: Pta1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta1Id {}
pub type Pta1 = Pin<Pta1Id, PortaId>;
impl AltFn<super::sig::Tsi0Ch2> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta1> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0Rx> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm2Ch0> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTA2: Pin<Pta2Id, PortaId> = Pin { port: PORTA, index: 2, id: Pta2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta2Id {}
pub type Pta2 = Pin<Pta2Id, PortaId>;
impl AltFn<super::sig::Tsi0Ch3> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta2> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0Tx> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm2Ch1> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTA3: Pin<Pta3Id, PortaId> = Pin { port: PORTA, index: 3, id: Pta3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta3Id {}
pub type Pta3 = Pin<Pta3Id, PortaId>;
impl AltFn<super::sig::Tsi0Ch4> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta3> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Scl> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch0> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::SwdDio> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA4: Pin<Pta4Id, PortaId> = Pin { port: PORTA, index: 4, id: Pta4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta4Id {}
pub type Pta4 = Pin<Pta4Id, PortaId>;
impl AltFn<super::sig::Tsi0Ch5> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta4> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Sda> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch1> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::NmiB> for Pta4Id {
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

impl AltFn<super::sig::UsbClkin> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch2> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxBclk> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA12: Pin<Pta12Id, PortaId> = Pin { port: PORTA, index: 12, id: Pta12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta12Id {}
pub type Pta12 = Pin<Pta12Id, PortaId>;
impl AltFn<super::sig::Pta12> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm1Ch0> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Txd0> for Pta12Id {
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

impl AltFn<super::sig::Tpm1Ch1> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxFs> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA18: Pin<Pta18Id, PortaId> = Pin { port: PORTA, index: 18, id: Pta18Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta18Id {}
pub type Pta18 = Pin<Pta18Id, PortaId>;
impl AltFn<super::sig::Extal0> for Pta18Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta18> for Pta18Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart1Rx> for Pta18Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin0> for Pta18Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTA19: Pin<Pta19Id, PortaId> = Pin { port: PORTA, index: 19, id: Pta19Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta19Id {}
pub type Pta19 = Pin<Pta19Id, PortaId>;
impl AltFn<super::sig::Xtal0> for Pta19Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta19> for Pta19Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart1Tx> for Pta19Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin1> for Pta19Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lptmr0Alt1> for Pta19Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA20: Pin<Pta20Id, PortaId> = Pin { port: PORTA, index: 20, id: Pta20Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pta20Id {}
pub type Pta20 = Pin<Pta20Id, PortaId>;
impl AltFn<super::sig::Pta20> for Pta20Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::ResetB> for Pta20Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTB0: Pin<Ptb0Id, PortbId> = Pin { port: PORTB, index: 0, id: Ptb0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb0Id {}
pub type Ptb0 = Pin<Ptb0Id, PortbId>;
impl AltFn<super::sig::Adc0Se8> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tsi0Ch0> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb0> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Scl> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm1Ch0> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB1: Pin<Ptb1Id, PortbId> = Pin { port: PORTB, index: 1, id: Ptb1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb1Id {}
pub type Ptb1 = Pin<Ptb1Id, PortbId>;
impl AltFn<super::sig::Adc0Se9> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tsi0Ch6> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb1> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Sda> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm1Ch1> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB2: Pin<Ptb2Id, PortbId> = Pin { port: PORTB, index: 2, id: Ptb2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb2Id {}
pub type Ptb2 = Pin<Ptb2Id, PortbId>;
impl AltFn<super::sig::Adc0Se12> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tsi0Ch7> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb2> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Scl> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm2Ch0> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB3: Pin<Ptb3Id, PortbId> = Pin { port: PORTB, index: 3, id: Ptb3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb3Id {}
pub type Ptb3 = Pin<Ptb3Id, PortbId>;
impl AltFn<super::sig::Adc0Se13> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tsi0Ch8> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb3> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Sda> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm2Ch1> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTB16: Pin<Ptb16Id, PortbId> = Pin { port: PORTB, index: 16, id: Ptb16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb16Id {}
pub type Ptb16 = Pin<Ptb16Id, PortbId>;
impl AltFn<super::sig::Tsi0Ch9> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb16> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Mosi> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Rx> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin0> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi1Miso> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTB17: Pin<Ptb17Id, PortbId> = Pin { port: PORTB, index: 17, id: Ptb17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb17Id {}
pub type Ptb17 = Pin<Ptb17Id, PortbId>;
impl AltFn<super::sig::Tsi0Ch10> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb17> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Miso> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Tx> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin1> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi1Mosi> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTB18: Pin<Ptb18Id, PortbId> = Pin { port: PORTB, index: 18, id: Ptb18Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb18Id {}
pub type Ptb18 = Pin<Ptb18Id, PortbId>;
impl AltFn<super::sig::Tsi0Ch11> for Ptb18Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb18> for Ptb18Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm2Ch0> for Ptb18Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxBclk> for Ptb18Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTB19: Pin<Ptb19Id, PortbId> = Pin { port: PORTB, index: 19, id: Ptb19Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptb19Id {}
pub type Ptb19 = Pin<Ptb19Id, PortbId>;
impl AltFn<super::sig::Tsi0Ch12> for Ptb19Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb19> for Ptb19Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm2Ch1> for Ptb19Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxFs> for Ptb19Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC0: Pin<Ptc0Id, PortcId> = Pin { port: PORTC, index: 0, id: Ptc0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc0Id {}
pub type Ptc0 = Pin<Ptc0Id, PortcId>;
impl AltFn<super::sig::Adc0Se14> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tsi0Ch13> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc0> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::ExtrgIn> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::AudiousbSofOut> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp0Out> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0Txd0> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC1: Pin<Ptc1Id, PortcId> = Pin { port: PORTC, index: 1, id: Ptc1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc1Id {}
pub type Ptc1 = Pin<Ptc1Id, PortcId>;
impl AltFn<super::sig::Adc0Se15> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tsi0Ch14> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc1> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Scl> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch0> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s0Txd0> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC2: Pin<Ptc2Id, PortcId> = Pin { port: PORTC, index: 2, id: Ptc2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc2Id {}
pub type Ptc2 = Pin<Ptc2Id, PortcId>;
impl AltFn<super::sig::Adc0Se11> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tsi0Ch15> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc2> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Sda> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch1> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s0TxFs> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC3: Pin<Ptc3Id, PortcId> = Pin { port: PORTC, index: 3, id: Ptc3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc3Id {}
pub type Ptc3 = Pin<Ptc3Id, PortcId>;
impl AltFn<super::sig::Ptc3> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart1Rx> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tpm0Ch2> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Clkout> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0TxBclk> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC4: Pin<Ptc4Id, PortcId> = Pin { port: PORTC, index: 4, id: Ptc4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc4Id {}
pub type Ptc4 = Pin<Ptc4Id, PortcId>;
impl AltFn<super::sig::Ptc4> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs0> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Tx> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tpm0Ch3> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s0Mclk> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC5: Pin<Ptc5Id, PortcId> = Pin { port: PORTC, index: 5, id: Ptc5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc5Id {}
pub type Ptc5 = Pin<Ptc5Id, PortcId>;
impl AltFn<super::sig::Ptc5> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sck> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Lptmr0Alt2> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Rxd0> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp0Out> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC6: Pin<Ptc6Id, PortcId> = Pin { port: PORTC, index: 6, id: Ptc6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc6Id {}
pub type Ptc6 = Pin<Ptc6Id, PortcId>;
impl AltFn<super::sig::Cmp0In0> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc6> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Mosi> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::ExtrgIn> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxBclk> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi0Miso> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0Mclk> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC7: Pin<Ptc7Id, PortcId> = Pin { port: PORTC, index: 7, id: Ptc7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc7Id {}
pub type Ptc7 = Pin<Ptc7Id, PortcId>;
impl AltFn<super::sig::Cmp0In1> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc7> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Miso> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::AudiousbSofOut> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxFs> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi0Mosi> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC8: Pin<Ptc8Id, PortcId> = Pin { port: PORTC, index: 8, id: Ptc8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc8Id {}
pub type Ptc8 = Pin<Ptc8Id, PortcId>;
impl AltFn<super::sig::Cmp0In2> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc8> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Scl> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch4> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Mclk> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC9: Pin<Ptc9Id, PortcId> = Pin { port: PORTC, index: 9, id: Ptc9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc9Id {}
pub type Ptc9 = Pin<Ptc9Id, PortcId>;
impl AltFn<super::sig::Cmp0In3> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc9> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Sda> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch5> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxBclk> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC10: Pin<Ptc10Id, PortcId> = Pin { port: PORTC, index: 10, id: Ptc10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc10Id {}
pub type Ptc10 = Pin<Ptc10Id, PortcId>;
impl AltFn<super::sig::Ptc10> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Scl> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2s0RxFs> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTC11: Pin<Ptc11Id, PortcId> = Pin { port: PORTC, index: 11, id: Ptc11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptc11Id {}
pub type Ptc11 = Pin<Ptc11Id, PortcId>;
impl AltFn<super::sig::Ptc11> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Sda> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2s0Rxd0> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTD0: Pin<Ptd0Id, PortdId> = Pin { port: PORTD, index: 0, id: Ptd0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd0Id {}
pub type Ptd0 = Pin<Ptd0Id, PortdId>;
impl AltFn<super::sig::Ptd0> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs0> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch0> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTD1: Pin<Ptd1Id, PortdId> = Pin { port: PORTD, index: 1, id: Ptd1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd1Id {}
pub type Ptd1 = Pin<Ptd1Id, PortdId>;
impl AltFn<super::sig::Adc0Se5b> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd1> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sck> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tpm0Ch1> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTD2: Pin<Ptd2Id, PortdId> = Pin { port: PORTD, index: 2, id: Ptd2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd2Id {}
pub type Ptd2 = Pin<Ptd2Id, PortdId>;
impl AltFn<super::sig::Ptd2> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Mosi> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Rx> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tpm0Ch2> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi0Miso> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTD3: Pin<Ptd3Id, PortdId> = Pin { port: PORTD, index: 3, id: Ptd3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd3Id {}
pub type Ptd3 = Pin<Ptd3Id, PortdId>;
impl AltFn<super::sig::Ptd3> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Miso> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Tx> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tpm0Ch3> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Spi0Mosi> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTD4: Pin<Ptd4Id, PortdId> = Pin { port: PORTD, index: 4, id: Ptd4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd4Id {}
pub type Ptd4 = Pin<Ptd4Id, PortdId>;
impl AltFn<super::sig::Ptd4> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Pcs0> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Rx> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tpm0Ch4> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTD5: Pin<Ptd5Id, PortdId> = Pin { port: PORTD, index: 5, id: Ptd5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd5Id {}
pub type Ptd5 = Pin<Ptd5Id, PortdId>;
impl AltFn<super::sig::Adc0Se6b> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd5> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sck> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Tx> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tpm0Ch5> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTD6: Pin<Ptd6Id, PortdId> = Pin { port: PORTD, index: 6, id: Ptd6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd6Id {}
pub type Ptd6 = Pin<Ptd6Id, PortdId>;
impl AltFn<super::sig::Adc0Se7b> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd6> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Mosi> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Rx> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Miso> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTD7: Pin<Ptd7Id, PortdId> = Pin { port: PORTD, index: 7, id: Ptd7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ptd7Id {}
pub type Ptd7 = Pin<Ptd7Id, PortdId>;
impl AltFn<super::sig::Ptd7> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Miso> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Tx> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Mosi> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTE0: Pin<Pte0Id, PorteId> = Pin { port: PORTE, index: 0, id: Pte0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte0Id {}
pub type Pte0 = Pin<Pte0Id, PorteId>;
impl AltFn<super::sig::Pte0> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Miso> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Tx> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::RtcClkout> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp0Out> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c1Sda> for Pte0Id {
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

impl AltFn<super::sig::Spi1Mosi> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Rx> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Spi1Miso> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c1Scl> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE20: Pin<Pte20Id, PorteId> = Pin { port: PORTE, index: 20, id: Pte20Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte20Id {}
pub type Pte20 = Pin<Pte20Id, PorteId>;
impl AltFn<super::sig::Adc0Dp0> for Pte20Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se0> for Pte20Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte20> for Pte20Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm1Ch0> for Pte20Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart0Tx> for Pte20Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE21: Pin<Pte21Id, PorteId> = Pin { port: PORTE, index: 21, id: Pte21Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte21Id {}
pub type Pte21 = Pin<Pte21Id, PorteId>;
impl AltFn<super::sig::Adc0Dm0> for Pte21Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se4a> for Pte21Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte21> for Pte21Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm1Ch1> for Pte21Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart0Rx> for Pte21Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE22: Pin<Pte22Id, PorteId> = Pin { port: PORTE, index: 22, id: Pte22Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte22Id {}
pub type Pte22 = Pin<Pte22Id, PorteId>;
impl AltFn<super::sig::Adc0Dp3> for Pte22Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se3> for Pte22Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte22> for Pte22Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm2Ch0> for Pte22Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart2Tx> for Pte22Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE23: Pin<Pte23Id, PorteId> = Pin { port: PORTE, index: 23, id: Pte23Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte23Id {}
pub type Pte23 = Pin<Pte23Id, PorteId>;
impl AltFn<super::sig::Adc0Dm3> for Pte23Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se7a> for Pte23Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte23> for Pte23Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm2Ch1> for Pte23Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart2Rx> for Pte23Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE24: Pin<Pte24Id, PorteId> = Pin { port: PORTE, index: 24, id: Pte24Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte24Id {}
pub type Pte24 = Pin<Pte24Id, PorteId>;
impl AltFn<super::sig::Pte24> for Pte24Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch0> for Pte24Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c0Scl> for Pte24Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTE25: Pin<Pte25Id, PorteId> = Pin { port: PORTE, index: 25, id: Pte25Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte25Id {}
pub type Pte25 = Pin<Pte25Id, PorteId>;
impl AltFn<super::sig::Pte25> for Pte25Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch1> for Pte25Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c0Sda> for Pte25Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTE29: Pin<Pte29Id, PorteId> = Pin { port: PORTE, index: 29, id: Pte29Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte29Id {}
pub type Pte29 = Pin<Pte29Id, PorteId>;
impl AltFn<super::sig::Cmp0In5> for Pte29Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se4b> for Pte29Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte29> for Pte29Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch2> for Pte29Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin0> for Pte29Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE30: Pin<Pte30Id, PorteId> = Pin { port: PORTE, index: 30, id: Pte30Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte30Id {}
pub type Pte30 = Pin<Pte30Id, PorteId>;
impl AltFn<super::sig::Dac0Out> for Pte30Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc0Se23> for Pte30Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In4> for Pte30Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte30> for Pte30Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch3> for Pte30Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TpmClkin1> for Pte30Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTE31: Pin<Pte31Id, PorteId> = Pin { port: PORTE, index: 31, id: Pte31Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pte31Id {}
pub type Pte31 = Pin<Pte31Id, PorteId>;
impl AltFn<super::sig::Pte31> for Pte31Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tpm0Ch4> for Pte31Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

