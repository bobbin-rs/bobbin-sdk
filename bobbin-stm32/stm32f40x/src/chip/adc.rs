#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::adc_f24::*;

pub const ADC1: Adc1 = Periph(0x40012000, Adc1Id {});
pub const ADC2: Adc2 = Periph(0x40012100, Adc2Id {});
pub const ADC3: Adc3 = Periph(0x40012200, Adc3Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc1Id {}
pub type Adc1 = Periph<Adc1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc2Id {}
pub type Adc2 = Periph<Adc2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc3Id {}
pub type Adc3 = Periph<Adc3Id>;

impl super::sig::Signal<super::sig::Adc1In0> for Adc1Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc1In0> for Adc1Ch0 {}
impl super::sig::Signal<super::sig::Adc1In1> for Adc1Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc1In1> for Adc1Ch1 {}
impl super::sig::Signal<super::sig::Adc1In2> for Adc1Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc1In2> for Adc1Ch2 {}
impl super::sig::Signal<super::sig::Adc1In3> for Adc1Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc1In3> for Adc1Ch3 {}
impl super::sig::Signal<super::sig::Adc1In4> for Adc1Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc1In4> for Adc1Ch4 {}
impl super::sig::Signal<super::sig::Adc1In5> for Adc1Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc1In5> for Adc1Ch5 {}
impl super::sig::Signal<super::sig::Adc1In6> for Adc1Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc1In6> for Adc1Ch6 {}
impl super::sig::Signal<super::sig::Adc1In7> for Adc1Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc1In7> for Adc1Ch7 {}
impl super::sig::Signal<super::sig::Adc1In8> for Adc1Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc1In8> for Adc1Ch8 {}
impl super::sig::Signal<super::sig::Adc1In9> for Adc1Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc1In9> for Adc1Ch9 {}
impl super::sig::Signal<super::sig::Adc1In10> for Adc1Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc1In10> for Adc1Ch10 {}
impl super::sig::Signal<super::sig::Adc1In11> for Adc1Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc1In11> for Adc1Ch11 {}
impl super::sig::Signal<super::sig::Adc1In12> for Adc1Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc1In12> for Adc1Ch12 {}
impl super::sig::Signal<super::sig::Adc1In13> for Adc1Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc1In13> for Adc1Ch13 {}
impl super::sig::Signal<super::sig::Adc1In14> for Adc1Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc1In14> for Adc1Ch14 {}
impl super::sig::Signal<super::sig::Adc1In15> for Adc1Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc1In15> for Adc1Ch15 {}

impl super::sig::Signal<super::sig::Adc2In0> for Adc2Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc2In0> for Adc2Ch0 {}
impl super::sig::Signal<super::sig::Adc2In1> for Adc2Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc2In1> for Adc2Ch1 {}
impl super::sig::Signal<super::sig::Adc2In2> for Adc2Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc2In2> for Adc2Ch2 {}
impl super::sig::Signal<super::sig::Adc2In3> for Adc2Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc2In3> for Adc2Ch3 {}
impl super::sig::Signal<super::sig::Adc2In4> for Adc2Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc2In4> for Adc2Ch4 {}
impl super::sig::Signal<super::sig::Adc2In5> for Adc2Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc2In5> for Adc2Ch5 {}
impl super::sig::Signal<super::sig::Adc2In6> for Adc2Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc2In6> for Adc2Ch6 {}
impl super::sig::Signal<super::sig::Adc2In7> for Adc2Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc2In7> for Adc2Ch7 {}
impl super::sig::Signal<super::sig::Adc2In8> for Adc2Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc2In8> for Adc2Ch8 {}
impl super::sig::Signal<super::sig::Adc2In9> for Adc2Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc2In9> for Adc2Ch9 {}
impl super::sig::Signal<super::sig::Adc2In10> for Adc2Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc2In10> for Adc2Ch10 {}
impl super::sig::Signal<super::sig::Adc2In11> for Adc2Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc2In11> for Adc2Ch11 {}
impl super::sig::Signal<super::sig::Adc2In12> for Adc2Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc2In12> for Adc2Ch12 {}
impl super::sig::Signal<super::sig::Adc2In13> for Adc2Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc2In13> for Adc2Ch13 {}
impl super::sig::Signal<super::sig::Adc2In14> for Adc2Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc2In14> for Adc2Ch14 {}
impl super::sig::Signal<super::sig::Adc2In15> for Adc2Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc2In15> for Adc2Ch15 {}

impl super::sig::Signal<super::sig::Adc3In0> for Adc3Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc3In0> for Adc3Ch0 {}
impl super::sig::Signal<super::sig::Adc3In1> for Adc3Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc3In1> for Adc3Ch1 {}
impl super::sig::Signal<super::sig::Adc3In2> for Adc3Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc3In2> for Adc3Ch2 {}
impl super::sig::Signal<super::sig::Adc3In3> for Adc3Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc3In3> for Adc3Ch3 {}
impl super::sig::Signal<super::sig::Adc3In4> for Adc3Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc3In4> for Adc3Ch4 {}
impl super::sig::Signal<super::sig::Adc3In5> for Adc3Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc3In5> for Adc3Ch5 {}
impl super::sig::Signal<super::sig::Adc3In6> for Adc3Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc3In6> for Adc3Ch6 {}
impl super::sig::Signal<super::sig::Adc3In7> for Adc3Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc3In7> for Adc3Ch7 {}
impl super::sig::Signal<super::sig::Adc3In8> for Adc3Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc3In8> for Adc3Ch8 {}
impl super::sig::Signal<super::sig::Adc3In9> for Adc3Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc3In9> for Adc3Ch9 {}
impl super::sig::Signal<super::sig::Adc3In10> for Adc3Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc3In10> for Adc3Ch10 {}
impl super::sig::Signal<super::sig::Adc3In11> for Adc3Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc3In11> for Adc3Ch11 {}
impl super::sig::Signal<super::sig::Adc3In12> for Adc3Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc3In12> for Adc3Ch12 {}
impl super::sig::Signal<super::sig::Adc3In13> for Adc3Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc3In13> for Adc3Ch13 {}
impl super::sig::Signal<super::sig::Adc3In14> for Adc3Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc3In14> for Adc3Ch14 {}
impl super::sig::Signal<super::sig::Adc3In15> for Adc3Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc3In15> for Adc3Ch15 {}


pub const ADC1_CH0: Channel<Adc1Ch0Id, Adc1Id> = Channel { periph: ADC1, index: 0, id: Adc1Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch0Id {}
pub type Adc1Ch0 = Channel<Adc1Ch0Id, Adc1Id>;

pub const ADC1_CH1: Channel<Adc1Ch1Id, Adc1Id> = Channel { periph: ADC1, index: 1, id: Adc1Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch1Id {}
pub type Adc1Ch1 = Channel<Adc1Ch1Id, Adc1Id>;

pub const ADC1_CH2: Channel<Adc1Ch2Id, Adc1Id> = Channel { periph: ADC1, index: 2, id: Adc1Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch2Id {}
pub type Adc1Ch2 = Channel<Adc1Ch2Id, Adc1Id>;

pub const ADC1_CH3: Channel<Adc1Ch3Id, Adc1Id> = Channel { periph: ADC1, index: 3, id: Adc1Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch3Id {}
pub type Adc1Ch3 = Channel<Adc1Ch3Id, Adc1Id>;

pub const ADC1_CH4: Channel<Adc1Ch4Id, Adc1Id> = Channel { periph: ADC1, index: 4, id: Adc1Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch4Id {}
pub type Adc1Ch4 = Channel<Adc1Ch4Id, Adc1Id>;

pub const ADC1_CH5: Channel<Adc1Ch5Id, Adc1Id> = Channel { periph: ADC1, index: 5, id: Adc1Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch5Id {}
pub type Adc1Ch5 = Channel<Adc1Ch5Id, Adc1Id>;

pub const ADC1_CH6: Channel<Adc1Ch6Id, Adc1Id> = Channel { periph: ADC1, index: 6, id: Adc1Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch6Id {}
pub type Adc1Ch6 = Channel<Adc1Ch6Id, Adc1Id>;

pub const ADC1_CH7: Channel<Adc1Ch7Id, Adc1Id> = Channel { periph: ADC1, index: 7, id: Adc1Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch7Id {}
pub type Adc1Ch7 = Channel<Adc1Ch7Id, Adc1Id>;

pub const ADC1_CH8: Channel<Adc1Ch8Id, Adc1Id> = Channel { periph: ADC1, index: 8, id: Adc1Ch8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch8Id {}
pub type Adc1Ch8 = Channel<Adc1Ch8Id, Adc1Id>;

pub const ADC1_CH9: Channel<Adc1Ch9Id, Adc1Id> = Channel { periph: ADC1, index: 9, id: Adc1Ch9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch9Id {}
pub type Adc1Ch9 = Channel<Adc1Ch9Id, Adc1Id>;

pub const ADC1_CH10: Channel<Adc1Ch10Id, Adc1Id> = Channel { periph: ADC1, index: 10, id: Adc1Ch10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch10Id {}
pub type Adc1Ch10 = Channel<Adc1Ch10Id, Adc1Id>;

pub const ADC1_CH11: Channel<Adc1Ch11Id, Adc1Id> = Channel { periph: ADC1, index: 11, id: Adc1Ch11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch11Id {}
pub type Adc1Ch11 = Channel<Adc1Ch11Id, Adc1Id>;

pub const ADC1_CH12: Channel<Adc1Ch12Id, Adc1Id> = Channel { periph: ADC1, index: 12, id: Adc1Ch12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch12Id {}
pub type Adc1Ch12 = Channel<Adc1Ch12Id, Adc1Id>;

pub const ADC1_CH13: Channel<Adc1Ch13Id, Adc1Id> = Channel { periph: ADC1, index: 13, id: Adc1Ch13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch13Id {}
pub type Adc1Ch13 = Channel<Adc1Ch13Id, Adc1Id>;

pub const ADC1_CH14: Channel<Adc1Ch14Id, Adc1Id> = Channel { periph: ADC1, index: 14, id: Adc1Ch14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch14Id {}
pub type Adc1Ch14 = Channel<Adc1Ch14Id, Adc1Id>;

pub const ADC1_CH15: Channel<Adc1Ch15Id, Adc1Id> = Channel { periph: ADC1, index: 15, id: Adc1Ch15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch15Id {}
pub type Adc1Ch15 = Channel<Adc1Ch15Id, Adc1Id>;

pub const ADC1_TEMP: Channel<Adc1TempId, Adc1Id> = Channel { periph: ADC1, index: 16, id: Adc1TempId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1TempId {}
pub type Adc1Temp = Channel<Adc1TempId, Adc1Id>;

pub const ADC1_REF: Channel<Adc1RefId, Adc1Id> = Channel { periph: ADC1, index: 17, id: Adc1RefId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1RefId {}
pub type Adc1Ref = Channel<Adc1RefId, Adc1Id>;

pub const ADC1_BAT: Channel<Adc1BatId, Adc1Id> = Channel { periph: ADC1, index: 18, id: Adc1BatId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1BatId {}
pub type Adc1Bat = Channel<Adc1BatId, Adc1Id>;

pub const ADC2_CH0: Channel<Adc2Ch0Id, Adc2Id> = Channel { periph: ADC2, index: 0, id: Adc2Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch0Id {}
pub type Adc2Ch0 = Channel<Adc2Ch0Id, Adc2Id>;

pub const ADC2_CH1: Channel<Adc2Ch1Id, Adc2Id> = Channel { periph: ADC2, index: 1, id: Adc2Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch1Id {}
pub type Adc2Ch1 = Channel<Adc2Ch1Id, Adc2Id>;

pub const ADC2_CH2: Channel<Adc2Ch2Id, Adc2Id> = Channel { periph: ADC2, index: 2, id: Adc2Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch2Id {}
pub type Adc2Ch2 = Channel<Adc2Ch2Id, Adc2Id>;

pub const ADC2_CH3: Channel<Adc2Ch3Id, Adc2Id> = Channel { periph: ADC2, index: 3, id: Adc2Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch3Id {}
pub type Adc2Ch3 = Channel<Adc2Ch3Id, Adc2Id>;

pub const ADC2_CH4: Channel<Adc2Ch4Id, Adc2Id> = Channel { periph: ADC2, index: 4, id: Adc2Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch4Id {}
pub type Adc2Ch4 = Channel<Adc2Ch4Id, Adc2Id>;

pub const ADC2_CH5: Channel<Adc2Ch5Id, Adc2Id> = Channel { periph: ADC2, index: 5, id: Adc2Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch5Id {}
pub type Adc2Ch5 = Channel<Adc2Ch5Id, Adc2Id>;

pub const ADC2_CH6: Channel<Adc2Ch6Id, Adc2Id> = Channel { periph: ADC2, index: 6, id: Adc2Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch6Id {}
pub type Adc2Ch6 = Channel<Adc2Ch6Id, Adc2Id>;

pub const ADC2_CH7: Channel<Adc2Ch7Id, Adc2Id> = Channel { periph: ADC2, index: 7, id: Adc2Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch7Id {}
pub type Adc2Ch7 = Channel<Adc2Ch7Id, Adc2Id>;

pub const ADC2_CH8: Channel<Adc2Ch8Id, Adc2Id> = Channel { periph: ADC2, index: 8, id: Adc2Ch8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch8Id {}
pub type Adc2Ch8 = Channel<Adc2Ch8Id, Adc2Id>;

pub const ADC2_CH9: Channel<Adc2Ch9Id, Adc2Id> = Channel { periph: ADC2, index: 9, id: Adc2Ch9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch9Id {}
pub type Adc2Ch9 = Channel<Adc2Ch9Id, Adc2Id>;

pub const ADC2_CH10: Channel<Adc2Ch10Id, Adc2Id> = Channel { periph: ADC2, index: 10, id: Adc2Ch10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch10Id {}
pub type Adc2Ch10 = Channel<Adc2Ch10Id, Adc2Id>;

pub const ADC2_CH11: Channel<Adc2Ch11Id, Adc2Id> = Channel { periph: ADC2, index: 11, id: Adc2Ch11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch11Id {}
pub type Adc2Ch11 = Channel<Adc2Ch11Id, Adc2Id>;

pub const ADC2_CH12: Channel<Adc2Ch12Id, Adc2Id> = Channel { periph: ADC2, index: 12, id: Adc2Ch12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch12Id {}
pub type Adc2Ch12 = Channel<Adc2Ch12Id, Adc2Id>;

pub const ADC2_CH13: Channel<Adc2Ch13Id, Adc2Id> = Channel { periph: ADC2, index: 13, id: Adc2Ch13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch13Id {}
pub type Adc2Ch13 = Channel<Adc2Ch13Id, Adc2Id>;

pub const ADC2_CH14: Channel<Adc2Ch14Id, Adc2Id> = Channel { periph: ADC2, index: 14, id: Adc2Ch14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch14Id {}
pub type Adc2Ch14 = Channel<Adc2Ch14Id, Adc2Id>;

pub const ADC2_CH15: Channel<Adc2Ch15Id, Adc2Id> = Channel { periph: ADC2, index: 15, id: Adc2Ch15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch15Id {}
pub type Adc2Ch15 = Channel<Adc2Ch15Id, Adc2Id>;

pub const ADC3_CH0: Channel<Adc3Ch0Id, Adc3Id> = Channel { periph: ADC3, index: 0, id: Adc3Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch0Id {}
pub type Adc3Ch0 = Channel<Adc3Ch0Id, Adc3Id>;

pub const ADC3_CH1: Channel<Adc3Ch1Id, Adc3Id> = Channel { periph: ADC3, index: 1, id: Adc3Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch1Id {}
pub type Adc3Ch1 = Channel<Adc3Ch1Id, Adc3Id>;

pub const ADC3_CH2: Channel<Adc3Ch2Id, Adc3Id> = Channel { periph: ADC3, index: 2, id: Adc3Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch2Id {}
pub type Adc3Ch2 = Channel<Adc3Ch2Id, Adc3Id>;

pub const ADC3_CH3: Channel<Adc3Ch3Id, Adc3Id> = Channel { periph: ADC3, index: 3, id: Adc3Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch3Id {}
pub type Adc3Ch3 = Channel<Adc3Ch3Id, Adc3Id>;

pub const ADC3_CH4: Channel<Adc3Ch4Id, Adc3Id> = Channel { periph: ADC3, index: 4, id: Adc3Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch4Id {}
pub type Adc3Ch4 = Channel<Adc3Ch4Id, Adc3Id>;

pub const ADC3_CH5: Channel<Adc3Ch5Id, Adc3Id> = Channel { periph: ADC3, index: 5, id: Adc3Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch5Id {}
pub type Adc3Ch5 = Channel<Adc3Ch5Id, Adc3Id>;

pub const ADC3_CH6: Channel<Adc3Ch6Id, Adc3Id> = Channel { periph: ADC3, index: 6, id: Adc3Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch6Id {}
pub type Adc3Ch6 = Channel<Adc3Ch6Id, Adc3Id>;

pub const ADC3_CH7: Channel<Adc3Ch7Id, Adc3Id> = Channel { periph: ADC3, index: 7, id: Adc3Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch7Id {}
pub type Adc3Ch7 = Channel<Adc3Ch7Id, Adc3Id>;

pub const ADC3_CH8: Channel<Adc3Ch8Id, Adc3Id> = Channel { periph: ADC3, index: 8, id: Adc3Ch8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch8Id {}
pub type Adc3Ch8 = Channel<Adc3Ch8Id, Adc3Id>;

pub const ADC3_CH9: Channel<Adc3Ch9Id, Adc3Id> = Channel { periph: ADC3, index: 9, id: Adc3Ch9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch9Id {}
pub type Adc3Ch9 = Channel<Adc3Ch9Id, Adc3Id>;

pub const ADC3_CH10: Channel<Adc3Ch10Id, Adc3Id> = Channel { periph: ADC3, index: 10, id: Adc3Ch10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch10Id {}
pub type Adc3Ch10 = Channel<Adc3Ch10Id, Adc3Id>;

pub const ADC3_CH11: Channel<Adc3Ch11Id, Adc3Id> = Channel { periph: ADC3, index: 11, id: Adc3Ch11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch11Id {}
pub type Adc3Ch11 = Channel<Adc3Ch11Id, Adc3Id>;

pub const ADC3_CH12: Channel<Adc3Ch12Id, Adc3Id> = Channel { periph: ADC3, index: 12, id: Adc3Ch12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch12Id {}
pub type Adc3Ch12 = Channel<Adc3Ch12Id, Adc3Id>;

pub const ADC3_CH13: Channel<Adc3Ch13Id, Adc3Id> = Channel { periph: ADC3, index: 13, id: Adc3Ch13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch13Id {}
pub type Adc3Ch13 = Channel<Adc3Ch13Id, Adc3Id>;

pub const ADC3_CH14: Channel<Adc3Ch14Id, Adc3Id> = Channel { periph: ADC3, index: 14, id: Adc3Ch14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch14Id {}
pub type Adc3Ch14 = Channel<Adc3Ch14Id, Adc3Id>;

pub const ADC3_CH15: Channel<Adc3Ch15Id, Adc3Id> = Channel { periph: ADC3, index: 15, id: Adc3Ch15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc3Ch15Id {}
pub type Adc3Ch15 = Channel<Adc3Ch15Id, Adc3Id>;

