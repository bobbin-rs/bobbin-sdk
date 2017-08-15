#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::adc_f124::*;

pub const ADC1: Adc1 = Periph(0x40012400, Adc1Id {});
pub const ADC2: Adc2 = Periph(0x40012800, Adc2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc1Id {}
pub type Adc1 = Periph<Adc1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc2Id {}
pub type Adc2 = Periph<Adc2Id>;




pub const ADC2_CH0: Channel<Adc2Ch0Id, Adc1Id> = Channel { periph: ADC1, index: 0, id: Adc2Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch0Id {}
pub type Adc2Ch0 = Channel<Adc2Ch0Id, Adc1Id>;

pub const ADC2_CH1: Channel<Adc2Ch1Id, Adc1Id> = Channel { periph: ADC1, index: 1, id: Adc2Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch1Id {}
pub type Adc2Ch1 = Channel<Adc2Ch1Id, Adc1Id>;

pub const ADC2_CH2: Channel<Adc2Ch2Id, Adc1Id> = Channel { periph: ADC1, index: 2, id: Adc2Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch2Id {}
pub type Adc2Ch2 = Channel<Adc2Ch2Id, Adc1Id>;

pub const ADC2_CH3: Channel<Adc2Ch3Id, Adc1Id> = Channel { periph: ADC1, index: 3, id: Adc2Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch3Id {}
pub type Adc2Ch3 = Channel<Adc2Ch3Id, Adc1Id>;

pub const ADC2_CH4: Channel<Adc2Ch4Id, Adc1Id> = Channel { periph: ADC1, index: 4, id: Adc2Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch4Id {}
pub type Adc2Ch4 = Channel<Adc2Ch4Id, Adc1Id>;

pub const ADC2_CH5: Channel<Adc2Ch5Id, Adc1Id> = Channel { periph: ADC1, index: 5, id: Adc2Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch5Id {}
pub type Adc2Ch5 = Channel<Adc2Ch5Id, Adc1Id>;

pub const ADC2_CH6: Channel<Adc2Ch6Id, Adc1Id> = Channel { periph: ADC1, index: 6, id: Adc2Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch6Id {}
pub type Adc2Ch6 = Channel<Adc2Ch6Id, Adc1Id>;

pub const ADC2_CH7: Channel<Adc2Ch7Id, Adc1Id> = Channel { periph: ADC1, index: 7, id: Adc2Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch7Id {}
pub type Adc2Ch7 = Channel<Adc2Ch7Id, Adc1Id>;

pub const ADC2_CH8: Channel<Adc2Ch8Id, Adc1Id> = Channel { periph: ADC1, index: 8, id: Adc2Ch8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch8Id {}
pub type Adc2Ch8 = Channel<Adc2Ch8Id, Adc1Id>;

pub const ADC2_CH9: Channel<Adc2Ch9Id, Adc1Id> = Channel { periph: ADC1, index: 9, id: Adc2Ch9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch9Id {}
pub type Adc2Ch9 = Channel<Adc2Ch9Id, Adc1Id>;

pub const ADC2_CH10: Channel<Adc2Ch10Id, Adc1Id> = Channel { periph: ADC1, index: 10, id: Adc2Ch10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch10Id {}
pub type Adc2Ch10 = Channel<Adc2Ch10Id, Adc1Id>;

pub const ADC2_CH11: Channel<Adc2Ch11Id, Adc1Id> = Channel { periph: ADC1, index: 11, id: Adc2Ch11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch11Id {}
pub type Adc2Ch11 = Channel<Adc2Ch11Id, Adc1Id>;

pub const ADC2_CH12: Channel<Adc2Ch12Id, Adc1Id> = Channel { periph: ADC1, index: 12, id: Adc2Ch12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch12Id {}
pub type Adc2Ch12 = Channel<Adc2Ch12Id, Adc1Id>;

pub const ADC2_CH13: Channel<Adc2Ch13Id, Adc1Id> = Channel { periph: ADC1, index: 13, id: Adc2Ch13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch13Id {}
pub type Adc2Ch13 = Channel<Adc2Ch13Id, Adc1Id>;

pub const ADC2_CH14: Channel<Adc2Ch14Id, Adc1Id> = Channel { periph: ADC1, index: 14, id: Adc2Ch14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch14Id {}
pub type Adc2Ch14 = Channel<Adc2Ch14Id, Adc1Id>;

pub const ADC2_CH15: Channel<Adc2Ch15Id, Adc1Id> = Channel { periph: ADC1, index: 15, id: Adc2Ch15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc2Ch15Id {}
pub type Adc2Ch15 = Channel<Adc2Ch15Id, Adc1Id>;

pub const ADC1_CH0: Channel<Adc1Ch0Id, Adc2Id> = Channel { periph: ADC2, index: 0, id: Adc1Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch0Id {}
pub type Adc1Ch0 = Channel<Adc1Ch0Id, Adc2Id>;

pub const ADC1_CH1: Channel<Adc1Ch1Id, Adc2Id> = Channel { periph: ADC2, index: 1, id: Adc1Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch1Id {}
pub type Adc1Ch1 = Channel<Adc1Ch1Id, Adc2Id>;

pub const ADC1_CH2: Channel<Adc1Ch2Id, Adc2Id> = Channel { periph: ADC2, index: 2, id: Adc1Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch2Id {}
pub type Adc1Ch2 = Channel<Adc1Ch2Id, Adc2Id>;

pub const ADC1_CH3: Channel<Adc1Ch3Id, Adc2Id> = Channel { periph: ADC2, index: 3, id: Adc1Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch3Id {}
pub type Adc1Ch3 = Channel<Adc1Ch3Id, Adc2Id>;

pub const ADC1_CH4: Channel<Adc1Ch4Id, Adc2Id> = Channel { periph: ADC2, index: 4, id: Adc1Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch4Id {}
pub type Adc1Ch4 = Channel<Adc1Ch4Id, Adc2Id>;

pub const ADC1_CH5: Channel<Adc1Ch5Id, Adc2Id> = Channel { periph: ADC2, index: 5, id: Adc1Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch5Id {}
pub type Adc1Ch5 = Channel<Adc1Ch5Id, Adc2Id>;

pub const ADC1_CH6: Channel<Adc1Ch6Id, Adc2Id> = Channel { periph: ADC2, index: 6, id: Adc1Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch6Id {}
pub type Adc1Ch6 = Channel<Adc1Ch6Id, Adc2Id>;

pub const ADC1_CH7: Channel<Adc1Ch7Id, Adc2Id> = Channel { periph: ADC2, index: 7, id: Adc1Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch7Id {}
pub type Adc1Ch7 = Channel<Adc1Ch7Id, Adc2Id>;

pub const ADC1_CH8: Channel<Adc1Ch8Id, Adc2Id> = Channel { periph: ADC2, index: 8, id: Adc1Ch8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch8Id {}
pub type Adc1Ch8 = Channel<Adc1Ch8Id, Adc2Id>;

pub const ADC1_CH9: Channel<Adc1Ch9Id, Adc2Id> = Channel { periph: ADC2, index: 9, id: Adc1Ch9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch9Id {}
pub type Adc1Ch9 = Channel<Adc1Ch9Id, Adc2Id>;

pub const ADC1_CH10: Channel<Adc1Ch10Id, Adc2Id> = Channel { periph: ADC2, index: 10, id: Adc1Ch10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch10Id {}
pub type Adc1Ch10 = Channel<Adc1Ch10Id, Adc2Id>;

pub const ADC1_CH11: Channel<Adc1Ch11Id, Adc2Id> = Channel { periph: ADC2, index: 11, id: Adc1Ch11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch11Id {}
pub type Adc1Ch11 = Channel<Adc1Ch11Id, Adc2Id>;

pub const ADC1_CH12: Channel<Adc1Ch12Id, Adc2Id> = Channel { periph: ADC2, index: 12, id: Adc1Ch12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch12Id {}
pub type Adc1Ch12 = Channel<Adc1Ch12Id, Adc2Id>;

pub const ADC1_CH13: Channel<Adc1Ch13Id, Adc2Id> = Channel { periph: ADC2, index: 13, id: Adc1Ch13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch13Id {}
pub type Adc1Ch13 = Channel<Adc1Ch13Id, Adc2Id>;

pub const ADC1_CH14: Channel<Adc1Ch14Id, Adc2Id> = Channel { periph: ADC2, index: 14, id: Adc1Ch14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch14Id {}
pub type Adc1Ch14 = Channel<Adc1Ch14Id, Adc2Id>;

pub const ADC1_CH15: Channel<Adc1Ch15Id, Adc2Id> = Channel { periph: ADC2, index: 15, id: Adc1Ch15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch15Id {}
pub type Adc1Ch15 = Channel<Adc1Ch15Id, Adc2Id>;

