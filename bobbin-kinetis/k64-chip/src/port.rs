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
pub struct PortaId {}
pub type Porta = Periph<PortaId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PortbId {}
pub type Portb = Periph<PortbId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PortcId {}
pub type Portc = Periph<PortcId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PortdId {}
pub type Portd = Periph<PortdId>;
#[derive(Clone, Copy, PartialEq, Eq)]
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
pub struct Pta0Id {}
pub type Pta0 = Pin<Pta0Id, PortaId>;
impl AltFn<super::sig::Pta0> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0CtsB> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0ColB> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm0Ch5> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::JtagTclk> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::SwdClk> for Pta0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA1: Pin<Pta1Id, PortaId> = Pin { port: PORTA, index: 1, id: Pta1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta1Id {}
pub type Pta1 = Pin<Pta1Id, PortaId>;
impl AltFn<super::sig::Pta1> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0Rx> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm0Ch6> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::JtagTdi> for Pta1Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA2: Pin<Pta2Id, PortaId> = Pin { port: PORTA, index: 2, id: Pta2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta2Id {}
pub type Pta2 = Pin<Pta2Id, PortaId>;
impl AltFn<super::sig::Pta2> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0Tx> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm0Ch7> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::JtagTdo> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::TraceSwo> for Pta2Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA3: Pin<Pta3Id, PortaId> = Pin { port: PORTA, index: 3, id: Pta3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta3Id {}
pub type Pta3 = Pin<Pta3Id, PortaId>;
impl AltFn<super::sig::Pta3> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart0RtsB> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm0Ch0> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::JtagTms> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::SwdDio> for Pta3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA4: Pin<Pta4Id, PortaId> = Pin { port: PORTA, index: 4, id: Pta4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta4Id {}
pub type Pta4 = Pin<Pta4Id, PortaId>;
impl AltFn<super::sig::Pta4> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch1> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::NmiB> for Pta4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA5: Pin<Pta5Id, PortaId> = Pin { port: PORTA, index: 5, id: Pta5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta5Id {}
pub type Pta5 = Pin<Pta5Id, PortaId>;
impl AltFn<super::sig::Pta5> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::UsbClkin> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm0Ch2> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Rxer> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Rxer> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Cmp2Out> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0TxBclk> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::JtagTrstB> for Pta5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA6: Pin<Pta6Id, PortaId> = Pin { port: PORTA, index: 6, id: Pta6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta6Id {}
pub type Pta6 = Pin<Pta6Id, PortaId>;
impl AltFn<super::sig::Pta6> for Pta6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch3> for Pta6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Clkout> for Pta6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::TraceClkout> for Pta6Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA7: Pin<Pta7Id, PortaId> = Pin { port: PORTA, index: 7, id: Pta7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta7Id {}
pub type Pta7 = Pin<Pta7Id, PortaId>;
impl AltFn<super::sig::Adc0Se10> for Pta7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta7> for Pta7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Ch4> for Pta7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::TraceD3> for Pta7Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA8: Pin<Pta8Id, PortaId> = Pin { port: PORTA, index: 8, id: Pta8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta8Id {}
pub type Pta8 = Pin<Pta8Id, PortaId>;
impl AltFn<super::sig::Adc0Se11> for Pta8Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta8> for Pta8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch0> for Pta8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm1QdPha> for Pta8Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TraceD2> for Pta8Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA9: Pin<Pta9Id, PortaId> = Pin { port: PORTA, index: 9, id: Pta9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta9Id {}
pub type Pta9 = Pin<Pta9Id, PortaId>;
impl AltFn<super::sig::Pta9> for Pta9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Ch1> for Pta9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Mii0Rxd3> for Pta9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Pta9Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TraceD1> for Pta9Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA10: Pin<Pta10Id, PortaId> = Pin { port: PORTA, index: 10, id: Pta10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta10Id {}
pub type Pta10 = Pin<Pta10Id, PortaId>;
impl AltFn<super::sig::Pta10> for Pta10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch0> for Pta10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Mii0Rxd2> for Pta10Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm2QdPha> for Pta10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::TraceD0> for Pta10Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA11: Pin<Pta11Id, PortaId> = Pin { port: PORTA, index: 11, id: Pta11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta11Id {}
pub type Pta11 = Pin<Pta11Id, PortaId>;
impl AltFn<super::sig::Pta11> for Pta11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm2Ch1> for Pta11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Mii0Rxclk> for Pta11Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2c2Sda> for Pta11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Pta11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA12: Pin<Pta12Id, PortaId> = Pin { port: PORTA, index: 12, id: Pta12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta12Id {}
pub type Pta12 = Pin<Pta12Id, PortaId>;
impl AltFn<super::sig::Cmp2In0> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta12> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Can0Tx> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch0> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Rxd1> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Rxd1> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2c2Scl> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0Txd0> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ftm1QdPha> for Pta12Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA13: Pin<Pta13Id, PortaId> = Pin { port: PORTA, index: 13, id: Pta13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta13Id {}
pub type Pta13 = Pin<Pta13Id, PortaId>;
impl AltFn<super::sig::Cmp2In1> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta13> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Can0Rx> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch1> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Rxd0> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Rxd0> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2c2Sda> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0TxFs> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Pta13Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA14: Pin<Pta14Id, PortaId> = Pin { port: PORTA, index: 14, id: Pta14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta14Id {}
pub type Pta14 = Pin<Pta14Id, PortaId>;
impl AltFn<super::sig::Pta14> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs0> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Tx> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0CrsDv> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Rxdv> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2c2Scl> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0RxBclk> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s0Txd1> for Pta14Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA15: Pin<Pta15Id, PortaId> = Pin { port: PORTA, index: 15, id: Pta15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta15Id {}
pub type Pta15 = Pin<Pta15Id, PortaId>;
impl AltFn<super::sig::Pta15> for Pta15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sck> for Pta15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Rx> for Pta15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Txen> for Pta15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Txen> for Pta15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s0Rxd0> for Pta15Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA16: Pin<Pta16Id, PortaId> = Pin { port: PORTA, index: 16, id: Pta16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta16Id {}
pub type Pta16 = Pin<Pta16Id, PortaId>;
impl AltFn<super::sig::Pta16> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sout> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0CtsB> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart0ColB> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Txd0> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Txd0> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s0RxFs> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2s0Rxd1> for Pta16Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTA17: Pin<Pta17Id, PortaId> = Pin { port: PORTA, index: 17, id: Pta17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta17Id {}
pub type Pta17 = Pin<Pta17Id, PortaId>;
impl AltFn<super::sig::Adc1Se17> for Pta17Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta17> for Pta17Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sin> for Pta17Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0RtsB> for Pta17Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Txd1> for Pta17Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Txd1> for Pta17Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2s0Mclk> for Pta17Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA18: Pin<Pta18Id, PortaId> = Pin { port: PORTA, index: 18, id: Pta18Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta18Id {}
pub type Pta18 = Pin<Pta18Id, PortaId>;
impl AltFn<super::sig::Extal0> for Pta18Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta18> for Pta18Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm0Flt2> for Pta18Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FtmClkin0> for Pta18Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

pub const PTA19: Pin<Pta19Id, PortaId> = Pin { port: PORTA, index: 19, id: Pta19Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta19Id {}
pub type Pta19 = Pin<Pta19Id, PortaId>;
impl AltFn<super::sig::Xtal0> for Pta19Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pta19> for Pta19Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm1Flt0> for Pta19Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FtmClkin1> for Pta19Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Lptmr0Alt1> for Pta19Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA24: Pin<Pta24Id, PortaId> = Pin { port: PORTA, index: 24, id: Pta24Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta24Id {}
pub type Pta24 = Pin<Pta24Id, PortaId>;
impl AltFn<super::sig::Pta24> for Pta24Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Txd2> for Pta24Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA29> for Pta24Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA25: Pin<Pta25Id, PortaId> = Pin { port: PORTA, index: 25, id: Pta25Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta25Id {}
pub type Pta25 = Pin<Pta25Id, PortaId>;
impl AltFn<super::sig::Pta25> for Pta25Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Txclk> for Pta25Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA28> for Pta25Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA26: Pin<Pta26Id, PortaId> = Pin { port: PORTA, index: 26, id: Pta26Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta26Id {}
pub type Pta26 = Pin<Pta26Id, PortaId>;
impl AltFn<super::sig::Pta26> for Pta26Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Txd3> for Pta26Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA27> for Pta26Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA27: Pin<Pta27Id, PortaId> = Pin { port: PORTA, index: 27, id: Pta27Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta27Id {}
pub type Pta27 = Pin<Pta27Id, PortaId>;
impl AltFn<super::sig::Pta27> for Pta27Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Crs> for Pta27Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA26> for Pta27Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA28: Pin<Pta28Id, PortaId> = Pin { port: PORTA, index: 28, id: Pta28Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta28Id {}
pub type Pta28 = Pin<Pta28Id, PortaId>;
impl AltFn<super::sig::Pta28> for Pta28Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Txer> for Pta28Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA25> for Pta28Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTA29: Pin<Pta29Id, PortaId> = Pin { port: PORTA, index: 29, id: Pta29Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pta29Id {}
pub type Pta29 = Pin<Pta29Id, PortaId>;
impl AltFn<super::sig::Pta29> for Pta29Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Mii0Col> for Pta29Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA24> for Pta29Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB0: Pin<Ptb0Id, PortbId> = Pin { port: PORTB, index: 0, id: Ptb0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb0Id {}
pub type Ptb0 = Pin<Ptb0Id, PortbId>;
impl AltFn<super::sig::Adc0Se8> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se8> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb0> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Scl> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch0> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Mdio> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Mdio> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPha> for Ptb0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB1: Pin<Ptb1Id, PortbId> = Pin { port: PORTB, index: 1, id: Ptb1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb1Id {}
pub type Ptb1 = Pin<Ptb1Id, PortbId>;
impl AltFn<super::sig::Adc0Se9> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se9> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb1> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Sda> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch1> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Rmii0Mdc> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Mii0Mdc> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Ptb1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB2: Pin<Ptb2Id, PortbId> = Pin { port: PORTB, index: 2, id: Ptb2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb2Id {}
pub type Ptb2 = Pin<Ptb2Id, PortbId>;
impl AltFn<super::sig::Adc0Se12> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb2> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Scl> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0RtsB> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr0> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm0Flt3> for Ptb2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB3: Pin<Ptb3Id, PortbId> = Pin { port: PORTB, index: 3, id: Ptb3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb3Id {}
pub type Ptb3 = Pin<Ptb3Id, PortbId>;
impl AltFn<super::sig::Adc0Se13> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb3> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Sda> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0CtsB> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart0ColB> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr1> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm0Flt0> for Ptb3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB4: Pin<Ptb4Id, PortbId> = Pin { port: PORTB, index: 4, id: Ptb4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb4Id {}
pub type Ptb4 = Pin<Ptb4Id, PortbId>;
impl AltFn<super::sig::Adc1Se10> for Ptb4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb4> for Ptb4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Enet01588Tmr2> for Ptb4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1Flt0> for Ptb4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB5: Pin<Ptb5Id, PortbId> = Pin { port: PORTB, index: 5, id: Ptb5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb5Id {}
pub type Ptb5 = Pin<Ptb5Id, PortbId>;
impl AltFn<super::sig::Adc1Se11> for Ptb5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb5> for Ptb5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Enet01588Tmr3> for Ptb5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm2Flt0> for Ptb5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB6: Pin<Ptb6Id, PortbId> = Pin { port: PORTB, index: 6, id: Ptb6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb6Id {}
pub type Ptb6 = Pin<Ptb6Id, PortbId>;
impl AltFn<super::sig::Adc1Se12> for Ptb6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb6> for Ptb6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FbAd23> for Ptb6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTB7: Pin<Ptb7Id, PortbId> = Pin { port: PORTB, index: 7, id: Ptb7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb7Id {}
pub type Ptb7 = Pin<Ptb7Id, PortbId>;
impl AltFn<super::sig::Adc1Se13> for Ptb7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb7> for Ptb7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::FbAd22> for Ptb7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTB8: Pin<Ptb8Id, PortbId> = Pin { port: PORTB, index: 8, id: Ptb8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb8Id {}
pub type Ptb8 = Pin<Ptb8Id, PortbId>;
impl AltFn<super::sig::Ptb8> for Ptb8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3RtsB> for Ptb8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd21> for Ptb8Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTB9: Pin<Ptb9Id, PortbId> = Pin { port: PORTB, index: 9, id: Ptb9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb9Id {}
pub type Ptb9 = Pin<Ptb9Id, PortbId>;
impl AltFn<super::sig::Ptb9> for Ptb9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Pcs1> for Ptb9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3CtsB> for Ptb9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd20> for Ptb9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTB10: Pin<Ptb10Id, PortbId> = Pin { port: PORTB, index: 10, id: Ptb10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb10Id {}
pub type Ptb10 = Pin<Ptb10Id, PortbId>;
impl AltFn<super::sig::Adc1Se14> for Ptb10Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb10> for Ptb10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Pcs0> for Ptb10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3Rx> for Ptb10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd19> for Ptb10Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm0Flt1> for Ptb10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB11: Pin<Ptb11Id, PortbId> = Pin { port: PORTB, index: 11, id: Ptb11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb11Id {}
pub type Ptb11 = Pin<Ptb11Id, PortbId>;
impl AltFn<super::sig::Adc1Se15> for Ptb11Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptb11> for Ptb11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sck> for Ptb11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3Tx> for Ptb11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd18> for Ptb11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm0Flt2> for Ptb11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB12: Pin<Ptb12Id, PortbId> = Pin { port: PORTB, index: 12, id: Ptb12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb12Id {}
pub type Ptb12 = Pin<Ptb12Id, PortbId>;
impl AltFn<super::sig::Ptb12> for Ptb12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3RtsB> for Ptb12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch0> for Ptb12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch4> for Ptb12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPha> for Ptb12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB13: Pin<Ptb13Id, PortbId> = Pin { port: PORTB, index: 13, id: Ptb13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb13Id {}
pub type Ptb13 = Pin<Ptb13Id, PortbId>;
impl AltFn<super::sig::Ptb13> for Ptb13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3CtsB> for Ptb13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm1Ch1> for Ptb13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch5> for Ptb13Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm1QdPhb> for Ptb13Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB16: Pin<Ptb16Id, PortbId> = Pin { port: PORTB, index: 16, id: Ptb16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb16Id {}
pub type Ptb16 = Pin<Ptb16Id, PortbId>;
impl AltFn<super::sig::Ptb16> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sout> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Rx> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FtmClkin0> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd17> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmIn> for Ptb16Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB17: Pin<Ptb17Id, PortbId> = Pin { port: PORTB, index: 17, id: Ptb17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb17Id {}
pub type Ptb17 = Pin<Ptb17Id, PortbId>;
impl AltFn<super::sig::Ptb17> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sin> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Tx> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FtmClkin1> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd16> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmOutB> for Ptb17Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB18: Pin<Ptb18Id, PortbId> = Pin { port: PORTB, index: 18, id: Ptb18Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb18Id {}
pub type Ptb18 = Pin<Ptb18Id, PortbId>;
impl AltFn<super::sig::Ptb18> for Ptb18Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Can0Tx> for Ptb18Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Ch0> for Ptb18Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxBclk> for Ptb18Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd15> for Ptb18Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm2QdPha> for Ptb18Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB19: Pin<Ptb19Id, PortbId> = Pin { port: PORTB, index: 19, id: Ptb19Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb19Id {}
pub type Ptb19 = Pin<Ptb19Id, PortbId>;
impl AltFn<super::sig::Ptb19> for Ptb19Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Can0Rx> for Ptb19Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm2Ch1> for Ptb19Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxFs> for Ptb19Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbOeB> for Ptb19Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm2QdPhb> for Ptb19Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB20: Pin<Ptb20Id, PortbId> = Pin { port: PORTB, index: 20, id: Ptb20Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb20Id {}
pub type Ptb20 = Pin<Ptb20Id, PortbId>;
impl AltFn<super::sig::Ptb20> for Ptb20Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Pcs0> for Ptb20Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FbAd31> for Ptb20Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Cmp0Out> for Ptb20Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB21: Pin<Ptb21Id, PortbId> = Pin { port: PORTB, index: 21, id: Ptb21Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb21Id {}
pub type Ptb21 = Pin<Ptb21Id, PortbId>;
impl AltFn<super::sig::Ptb21> for Ptb21Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sck> for Ptb21Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FbAd30> for Ptb21Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Cmp1Out> for Ptb21Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB22: Pin<Ptb22Id, PortbId> = Pin { port: PORTB, index: 22, id: Ptb22Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb22Id {}
pub type Ptb22 = Pin<Ptb22Id, PortbId>;
impl AltFn<super::sig::Ptb22> for Ptb22Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sout> for Ptb22Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::FbAd29> for Ptb22Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Cmp2Out> for Ptb22Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTB23: Pin<Ptb23Id, PortbId> = Pin { port: PORTB, index: 23, id: Ptb23Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptb23Id {}
pub type Ptb23 = Pin<Ptb23Id, PortbId>;
impl AltFn<super::sig::Ptb23> for Ptb23Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sin> for Ptb23Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Spi0Pcs5> for Ptb23Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd28> for Ptb23Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC0: Pin<Ptc0Id, PortcId> = Pin { port: PORTC, index: 0, id: Ptc0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc0Id {}
pub type Ptc0 = Pin<Ptc0Id, PortcId>;
impl AltFn<super::sig::Adc0Se14> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc0> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs4> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Pdb0Extrg> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::UsbSofOut> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd14> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0Txd1> for Ptc0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC1: Pin<Ptc1Id, PortcId> = Pin { port: PORTC, index: 1, id: Ptc1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc1Id {}
pub type Ptc1 = Pin<Ptc1Id, PortcId>;
impl AltFn<super::sig::Adc0Se15> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc1> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs3> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1RtsB> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch0> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd13> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0Txd0> for Ptc1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC2: Pin<Ptc2Id, PortcId> = Pin { port: PORTC, index: 2, id: Ptc2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc2Id {}
pub type Ptc2 = Pin<Ptc2Id, PortcId>;
impl AltFn<super::sig::Adc0Se4b> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp1In0> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc2> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs2> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1CtsB> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch1> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd12> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0TxFs> for Ptc2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC3: Pin<Ptc3Id, PortcId> = Pin { port: PORTC, index: 3, id: Ptc3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc3Id {}
pub type Ptc3 = Pin<Ptc3Id, PortcId>;
impl AltFn<super::sig::Cmp1In1> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc3> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs1> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Rx> for Ptc3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch2> for Ptc3Id {
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

impl AltFn<super::sig::Ftm0Ch3> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd11> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Cmp1Out> for Ptc4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC5: Pin<Ptc5Id, PortcId> = Pin { port: PORTC, index: 5, id: Ptc5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::FbAd10> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Cmp0Out> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ftm0Ch2> for Ptc5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTC6: Pin<Ptc6Id, PortcId> = Pin { port: PORTC, index: 6, id: Ptc6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc6Id {}
pub type Ptc6 = Pin<Ptc6Id, PortcId>;
impl AltFn<super::sig::Cmp0In0> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc6> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sout> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Pdb0Extrg> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxBclk> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd9> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2s0Mclk> for Ptc6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC7: Pin<Ptc7Id, PortcId> = Pin { port: PORTC, index: 7, id: Ptc7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc7Id {}
pub type Ptc7 = Pin<Ptc7Id, PortcId>;
impl AltFn<super::sig::Cmp0In1> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc7> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sin> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::UsbSofOut> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxFs> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd8> for Ptc7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC8: Pin<Ptc8Id, PortcId> = Pin { port: PORTC, index: 8, id: Ptc8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc8Id {}
pub type Ptc8 = Pin<Ptc8Id, PortcId>;
impl AltFn<super::sig::Adc1Se4b> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In2> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc8> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch4> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Mclk> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd7> for Ptc8Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC9: Pin<Ptc9Id, PortcId> = Pin { port: PORTC, index: 9, id: Ptc9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc9Id {}
pub type Ptc9 = Pin<Ptc9Id, PortcId>;
impl AltFn<super::sig::Adc1Se5b> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Cmp0In3> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc9> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ftm3Ch5> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxBclk> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd6> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm2Flt0> for Ptc9Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC10: Pin<Ptc10Id, PortcId> = Pin { port: PORTC, index: 10, id: Ptc10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc10Id {}
pub type Ptc10 = Pin<Ptc10Id, PortcId>;
impl AltFn<super::sig::Adc1Se6b> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc10> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Scl> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Ch6> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxFs> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd5> for Ptc10Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC11: Pin<Ptc11Id, PortcId> = Pin { port: PORTC, index: 11, id: Ptc11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc11Id {}
pub type Ptc11 = Pin<Ptc11Id, PortcId>;
impl AltFn<super::sig::Adc1Se7b> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptc11> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1Sda> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Ch7> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Rxd1> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbRwB> for Ptc11Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC12: Pin<Ptc12Id, PortcId> = Pin { port: PORTC, index: 12, id: Ptc12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc12Id {}
pub type Ptc12 = Pin<Ptc12Id, PortcId>;
impl AltFn<super::sig::Ptc12> for Ptc12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4RtsB> for Ptc12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd27> for Ptc12Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm3Flt0> for Ptc12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTC13: Pin<Ptc13Id, PortcId> = Pin { port: PORTC, index: 13, id: Ptc13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc13Id {}
pub type Ptc13 = Pin<Ptc13Id, PortcId>;
impl AltFn<super::sig::Ptc13> for Ptc13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4CtsB> for Ptc13Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd26> for Ptc13Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC14: Pin<Ptc14Id, PortcId> = Pin { port: PORTC, index: 14, id: Ptc14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc14Id {}
pub type Ptc14 = Pin<Ptc14Id, PortcId>;
impl AltFn<super::sig::Ptc14> for Ptc14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4Rx> for Ptc14Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd25> for Ptc14Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC15: Pin<Ptc15Id, PortcId> = Pin { port: PORTC, index: 15, id: Ptc15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc15Id {}
pub type Ptc15 = Pin<Ptc15Id, PortcId>;
impl AltFn<super::sig::Ptc15> for Ptc15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4Tx> for Ptc15Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbAd24> for Ptc15Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC16: Pin<Ptc16Id, PortcId> = Pin { port: PORTC, index: 16, id: Ptc16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc16Id {}
pub type Ptc16 = Pin<Ptc16Id, PortcId>;
impl AltFn<super::sig::Ptc16> for Ptc16Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3Rx> for Ptc16Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr0> for Ptc16Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbCs5B> for Ptc16Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbTsiz1> for Ptc16Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbBe2316Bls158B> for Ptc16Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC17: Pin<Ptc17Id, PortcId> = Pin { port: PORTC, index: 17, id: Ptc17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc17Id {}
pub type Ptc17 = Pin<Ptc17Id, PortcId>;
impl AltFn<super::sig::Ptc17> for Ptc17Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3Tx> for Ptc17Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr1> for Ptc17Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbCs4B> for Ptc17Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbTsiz0> for Ptc17Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbBe3124Bls70B> for Ptc17Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC18: Pin<Ptc18Id, PortcId> = Pin { port: PORTC, index: 18, id: Ptc18Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc18Id {}
pub type Ptc18 = Pin<Ptc18Id, PortcId>;
impl AltFn<super::sig::Ptc18> for Ptc18Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3RtsB> for Ptc18Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr2> for Ptc18Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbTbstB> for Ptc18Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbCs2B> for Ptc18Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbBe158Bls2316B> for Ptc18Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTC19: Pin<Ptc19Id, PortcId> = Pin { port: PORTC, index: 19, id: Ptc19Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptc19Id {}
pub type Ptc19 = Pin<Ptc19Id, PortcId>;
impl AltFn<super::sig::Ptc19> for Ptc19Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3CtsB> for Ptc19Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Enet01588Tmr3> for Ptc19Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbCs3B> for Ptc19Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbBe70Bls3124B> for Ptc19Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbTaB> for Ptc19Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD0: Pin<Ptd0Id, PortdId> = Pin { port: PORTD, index: 0, id: Ptd0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd0Id {}
pub type Ptd0 = Pin<Ptd0Id, PortdId>;
impl AltFn<super::sig::Ptd0> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs0> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2RtsB> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch0> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAle> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbCs1B> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::FbTsB> for Ptd0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTD1: Pin<Ptd1Id, PortdId> = Pin { port: PORTD, index: 1, id: Ptd1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
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

impl AltFn<super::sig::Uart2CtsB> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch1> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbCs0B> for Ptd1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTD2: Pin<Ptd2Id, PortdId> = Pin { port: PORTD, index: 2, id: Ptd2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd2Id {}
pub type Ptd2 = Pin<Ptd2Id, PortdId>;
impl AltFn<super::sig::Ptd2> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sout> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Rx> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch2> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd4> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c0Scl> for Ptd2Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD3: Pin<Ptd3Id, PortdId> = Pin { port: PORTD, index: 3, id: Ptd3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd3Id {}
pub type Ptd3 = Pin<Ptd3Id, PortdId>;
impl AltFn<super::sig::Ptd3> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Sin> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart2Tx> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm3Ch3> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd3> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c0Sda> for Ptd3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD4: Pin<Ptd4Id, PortdId> = Pin { port: PORTD, index: 4, id: Ptd4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd4Id {}
pub type Ptd4 = Pin<Ptd4Id, PortdId>;
impl AltFn<super::sig::Ptd4> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs1> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0RtsB> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch4> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd2> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmIn> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Spi1Pcs0> for Ptd4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD5: Pin<Ptd5Id, PortdId> = Pin { port: PORTD, index: 5, id: Ptd5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd5Id {}
pub type Ptd5 = Pin<Ptd5Id, PortdId>;
impl AltFn<super::sig::Adc0Se6b> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd5> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs2> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0CtsB> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Uart0ColB> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch5> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd1> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmOutB> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Spi1Sck> for Ptd5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD6: Pin<Ptd6Id, PortdId> = Pin { port: PORTD, index: 6, id: Ptd6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd6Id {}
pub type Ptd6 = Pin<Ptd6Id, PortdId>;
impl AltFn<super::sig::Adc0Se7b> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ptd6> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi0Pcs3> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Rx> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch6> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbAd0> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ftm0Flt0> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Spi1Sout> for Ptd6Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD7: Pin<Ptd7Id, PortdId> = Pin { port: PORTD, index: 7, id: Ptd7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd7Id {}
pub type Ptd7 = Pin<Ptd7Id, PortdId>;
impl AltFn<super::sig::Ptd7> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::CmtIro> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart0Tx> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ftm0Ch7> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm0Flt1> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Spi1Sin> for Ptd7Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTD8: Pin<Ptd8Id, PortdId> = Pin { port: PORTD, index: 8, id: Ptd8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd8Id {}
pub type Ptd8 = Pin<Ptd8Id, PortdId>;
impl AltFn<super::sig::Ptd8> for Ptd8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Scl> for Ptd8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5Rx> for Ptd8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbA16> for Ptd8Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD9: Pin<Ptd9Id, PortdId> = Pin { port: PORTD, index: 9, id: Ptd9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd9Id {}
pub type Ptd9 = Pin<Ptd9Id, PortdId>;
impl AltFn<super::sig::Ptd9> for Ptd9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c0Sda> for Ptd9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5Tx> for Ptd9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbA17> for Ptd9Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD10: Pin<Ptd10Id, PortdId> = Pin { port: PORTD, index: 10, id: Ptd10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd10Id {}
pub type Ptd10 = Pin<Ptd10Id, PortdId>;
impl AltFn<super::sig::Ptd10> for Ptd10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart5RtsB> for Ptd10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::FbA18> for Ptd10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD11: Pin<Ptd11Id, PortdId> = Pin { port: PORTD, index: 11, id: Ptd11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd11Id {}
pub type Ptd11 = Pin<Ptd11Id, PortdId>;
impl AltFn<super::sig::Ptd11> for Ptd11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Pcs0> for Ptd11Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5CtsB> for Ptd11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0Clkin> for Ptd11Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA19> for Ptd11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD12: Pin<Ptd12Id, PortdId> = Pin { port: PORTD, index: 12, id: Ptd12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd12Id {}
pub type Ptd12 = Pin<Ptd12Id, PortdId>;
impl AltFn<super::sig::Ptd12> for Ptd12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sck> for Ptd12Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ftm3Flt0> for Ptd12Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0D4> for Ptd12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA20> for Ptd12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD13: Pin<Ptd13Id, PortdId> = Pin { port: PORTD, index: 13, id: Ptd13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd13Id {}
pub type Ptd13 = Pin<Ptd13Id, PortdId>;
impl AltFn<super::sig::Ptd13> for Ptd13Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sout> for Ptd13Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sdhc0D5> for Ptd13Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA21> for Ptd13Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD14: Pin<Ptd14Id, PortdId> = Pin { port: PORTD, index: 14, id: Ptd14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd14Id {}
pub type Ptd14 = Pin<Ptd14Id, PortdId>;
impl AltFn<super::sig::Ptd14> for Ptd14Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Sin> for Ptd14Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sdhc0D6> for Ptd14Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA22> for Ptd14Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTD15: Pin<Ptd15Id, PortdId> = Pin { port: PORTD, index: 15, id: Ptd15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ptd15Id {}
pub type Ptd15 = Pin<Ptd15Id, PortdId>;
impl AltFn<super::sig::Ptd15> for Ptd15Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi2Pcs1> for Ptd15Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Sdhc0D7> for Ptd15Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::FbA23> for Ptd15Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE0: Pin<Pte0Id, PorteId> = Pin { port: PORTE, index: 0, id: Pte0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte0Id {}
pub type Pte0 = Pin<Pte0Id, PorteId>;
impl AltFn<super::sig::Adc1Se4a> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte0> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Pcs1> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Tx> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0D1> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TraceClkout> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c1Sda> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::RtcClkout> for Pte0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE1: Pin<Pte1Id, PorteId> = Pin { port: PORTE, index: 1, id: Pte1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte1Id {}
pub type Pte1 = Pin<Pte1Id, PorteId>;
impl AltFn<super::sig::Adc1Se5a> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte1> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sout> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1Rx> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0D0> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TraceD3> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c1Scl> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Spi1Sin> for Pte1Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE2: Pin<Pte2Id, PorteId> = Pin { port: PORTE, index: 2, id: Pte2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte2Id {}
pub type Pte2 = Pin<Pte2Id, PorteId>;
impl AltFn<super::sig::Adc0Dp2> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se6a> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte2> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sck> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1CtsB> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0Dclk> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TraceD2> for Pte2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTE3: Pin<Pte3Id, PorteId> = Pin { port: PORTE, index: 3, id: Pte3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte3Id {}
pub type Pte3 = Pin<Pte3Id, PorteId>;
impl AltFn<super::sig::Adc0Dm2> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Adc1Se7a> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte3> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Sin> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart1RtsB> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0Cmd> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TraceD1> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Spi1Sout> for Pte3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE4: Pin<Pte4Id, PorteId> = Pin { port: PORTE, index: 4, id: Pte4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte4Id {}
pub type Pte4 = Pin<Pte4Id, PorteId>;
impl AltFn<super::sig::Pte4> for Pte4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Pcs0> for Pte4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3Tx> for Pte4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0D3> for Pte4Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::TraceD0> for Pte4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PTE5: Pin<Pte5Id, PorteId> = Pin { port: PORTE, index: 5, id: Pte5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte5Id {}
pub type Pte5 = Pin<Pte5Id, PorteId>;
impl AltFn<super::sig::Pte5> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Pcs2> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3Rx> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Sdhc0D2> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch0> for Pte5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE6: Pin<Pte6Id, PorteId> = Pin { port: PORTE, index: 6, id: Pte6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte6Id {}
pub type Pte6 = Pin<Pte6Id, PorteId>;
impl AltFn<super::sig::Pte6> for Pte6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Spi1Pcs3> for Pte6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart3CtsB> for Pte6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Mclk> for Pte6Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch1> for Pte6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::UsbSofOut> for Pte6Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE7: Pin<Pte7Id, PorteId> = Pin { port: PORTE, index: 7, id: Pte7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte7Id {}
pub type Pte7 = Pin<Pte7Id, PorteId>;
impl AltFn<super::sig::Pte7> for Pte7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart3RtsB> for Pte7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Rxd0> for Pte7Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch2> for Pte7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE8: Pin<Pte8Id, PorteId> = Pin { port: PORTE, index: 8, id: Pte8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte8Id {}
pub type Pte8 = Pin<Pte8Id, PorteId>;
impl AltFn<super::sig::Pte8> for Pte8Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2s0Rxd1> for Pte8Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5Tx> for Pte8Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxFs> for Pte8Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch3> for Pte8Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE9: Pin<Pte9Id, PorteId> = Pin { port: PORTE, index: 9, id: Pte9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte9Id {}
pub type Pte9 = Pin<Pte9Id, PorteId>;
impl AltFn<super::sig::Pte9> for Pte9Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2s0Txd1> for Pte9Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart5Rx> for Pte9Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0RxBclk> for Pte9Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch4> for Pte9Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE10: Pin<Pte10Id, PorteId> = Pin { port: PORTE, index: 10, id: Pte10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte10Id {}
pub type Pte10 = Pin<Pte10Id, PorteId>;
impl AltFn<super::sig::Pte10> for Pte10Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart5CtsB> for Pte10Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0Txd0> for Pte10Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch5> for Pte10Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE11: Pin<Pte11Id, PorteId> = Pin { port: PORTE, index: 11, id: Pte11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte11Id {}
pub type Pte11 = Pin<Pte11Id, PorteId>;
impl AltFn<super::sig::Pte11> for Pte11Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart5RtsB> for Pte11Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2s0TxFs> for Pte11Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch6> for Pte11Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE12: Pin<Pte12Id, PorteId> = Pin { port: PORTE, index: 12, id: Pte12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte12Id {}
pub type Pte12 = Pin<Pte12Id, PorteId>;
impl AltFn<super::sig::Pte12> for Pte12Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2s0TxBclk> for Pte12Id {
   #[inline] fn alt_fn(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ftm3Ch7> for Pte12Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE24: Pin<Pte24Id, PorteId> = Pin { port: PORTE, index: 24, id: Pte24Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte24Id {}
pub type Pte24 = Pin<Pte24Id, PorteId>;
impl AltFn<super::sig::Adc0Se17> for Pte24Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte24> for Pte24Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4Tx> for Pte24Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c0Scl> for Pte24Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmOutB> for Pte24Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE25: Pin<Pte25Id, PorteId> = Pin { port: PORTE, index: 25, id: Pte25Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte25Id {}
pub type Pte25 = Pin<Pte25Id, PorteId>;
impl AltFn<super::sig::Adc0Se18> for Pte25Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Pte25> for Pte25Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4Rx> for Pte25Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c0Sda> for Pte25Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::EwmIn> for Pte25Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

pub const PTE26: Pin<Pte26Id, PorteId> = Pin { port: PORTE, index: 26, id: Pte26Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte26Id {}
pub type Pte26 = Pin<Pte26Id, PorteId>;
impl AltFn<super::sig::Pte26> for Pte26Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Enet1588Clkin> for Pte26Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Uart4CtsB> for Pte26Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::RtcClkout> for Pte26Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::UsbClkin> for Pte26Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PTE27: Pin<Pte27Id, PorteId> = Pin { port: PORTE, index: 27, id: Pte27Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte27Id {}
pub type Pte27 = Pin<Pte27Id, PorteId>;
impl AltFn<super::sig::Pte27> for Pte27Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Uart4RtsB> for Pte27Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PTE28: Pin<Pte28Id, PorteId> = Pin { port: PORTE, index: 28, id: Pte28Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pte28Id {}
pub type Pte28 = Pin<Pte28Id, PorteId>;
impl AltFn<super::sig::Pte28> for Pte28Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

