#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::adc_f3::*;

pub const ADC1: Adc1 = Periph(0x50000000, Adc1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc1Id {}
pub type Adc1 = Periph<Adc1Id>;

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

pub const ADC1_REFINT: Channel<Adc1RefintId, Adc1Id> = Channel { periph: ADC1, index: 18, id: Adc1RefintId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1RefintId {}
pub type Adc1Refint = Channel<Adc1RefintId, Adc1Id>;

pub const ADC1_BAT: Channel<Adc1BatId, Adc1Id> = Channel { periph: ADC1, index: 17, id: Adc1BatId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1BatId {}
pub type Adc1Bat = Channel<Adc1BatId, Adc1Id>;

