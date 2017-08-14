#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::adc::*;

pub const ADC0: Adc0 = Periph(0x4003b000, Adc0Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc0Id {}
pub type Adc0 = Periph<Adc0Id>;

impl super::sig::Signal<super::sig::Adc0Dp0> for Adc0Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc0Dp0> for Adc0Ch0 {}
impl super::sig::Signal<super::sig::Adc0Dm0> for Adc0Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc0Dm0> for Adc0Ch0 {}
impl super::sig::Signal<super::sig::Adc0Dp1> for Adc0Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc0Dp1> for Adc0Ch1 {}
impl super::sig::Signal<super::sig::Adc0Dm1> for Adc0Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc0Dm1> for Adc0Ch1 {}
impl super::sig::Signal<super::sig::Adc0Dp2> for Adc0Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc0Dp2> for Adc0Ch2 {}
impl super::sig::Signal<super::sig::Adc0Dm2> for Adc0Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc0Dm2> for Adc0Ch2 {}
impl super::sig::Signal<super::sig::Adc0Dp3> for Adc0Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc0Dp3> for Adc0Ch3 {}
impl super::sig::Signal<super::sig::Adc0Dm3> for Adc0Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc0Dm3> for Adc0Ch3 {}
impl super::sig::Signal<super::sig::Adc0Se4a> for Adc0Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc0Se4a> for Adc0Ch4 {}
impl super::sig::Signal<super::sig::Adc0Se4b> for Adc0Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc0Se4b> for Adc0Ch4 {}
impl super::sig::Signal<super::sig::Adc0Se5a> for Adc0Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc0Se5a> for Adc0Ch5 {}
impl super::sig::Signal<super::sig::Adc0Se5b> for Adc0Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc0Se5b> for Adc0Ch5 {}
impl super::sig::Signal<super::sig::Adc0Se6a> for Adc0Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc0Se6a> for Adc0Ch6 {}
impl super::sig::Signal<super::sig::Adc0Se6b> for Adc0Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc0Se6b> for Adc0Ch6 {}
impl super::sig::Signal<super::sig::Adc0Se7a> for Adc0Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc0Se7a> for Adc0Ch7 {}
impl super::sig::Signal<super::sig::Adc0Se7b> for Adc0Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc0Se7b> for Adc0Ch7 {}
impl super::sig::Signal<super::sig::Adc0Se8> for Adc0Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc0Se8> for Adc0Ch8 {}
impl super::sig::Signal<super::sig::Adc0Se9> for Adc0Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc0Se9> for Adc0Ch9 {}
impl super::sig::Signal<super::sig::Adc0Se10> for Adc0Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc0Se10> for Adc0Ch10 {}
impl super::sig::Signal<super::sig::Adc0Se11> for Adc0Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc0Se11> for Adc0Ch11 {}
impl super::sig::Signal<super::sig::Adc0Se12> for Adc0Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc0Se12> for Adc0Ch12 {}
impl super::sig::Signal<super::sig::Adc0Se13> for Adc0Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc0Se13> for Adc0Ch13 {}
impl super::sig::Signal<super::sig::Adc0Se14> for Adc0Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc0Se14> for Adc0Ch14 {}
impl super::sig::Signal<super::sig::Adc0Se15> for Adc0Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc0Se15> for Adc0Ch15 {}
impl super::sig::Signal<super::sig::Adc0Se16> for Adc0Ch16 {}
impl super::sig::SignalAdc<super::sig::Adc0Se16> for Adc0Ch16 {}
impl super::sig::Signal<super::sig::Adc0Se17> for Adc0Ch17 {}
impl super::sig::SignalAdc<super::sig::Adc0Se17> for Adc0Ch17 {}
impl super::sig::Signal<super::sig::Adc0Se18> for Adc0Ch18 {}
impl super::sig::SignalAdc<super::sig::Adc0Se18> for Adc0Ch18 {}
impl super::sig::Signal<super::sig::Adc0Se19> for Adc0Ch19 {}
impl super::sig::SignalAdc<super::sig::Adc0Se19> for Adc0Ch19 {}
impl super::sig::Signal<super::sig::Adc0Se20> for Adc0Ch20 {}
impl super::sig::SignalAdc<super::sig::Adc0Se20> for Adc0Ch20 {}
impl super::sig::Signal<super::sig::Adc0Se21> for Adc0Ch21 {}
impl super::sig::SignalAdc<super::sig::Adc0Se21> for Adc0Ch21 {}
impl super::sig::Signal<super::sig::Adc0Se22> for Adc0Ch22 {}
impl super::sig::SignalAdc<super::sig::Adc0Se22> for Adc0Ch22 {}
impl super::sig::Signal<super::sig::Adc0Se23> for Adc0Ch23 {}
impl super::sig::SignalAdc<super::sig::Adc0Se23> for Adc0Ch23 {}


pub const ADC0_CH0: Channel<Adc0Ch0Id, Adc0Id> = Channel { periph: ADC0, index: 0, id: Adc0Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch0Id {}
pub type Adc0Ch0 = Channel<Adc0Ch0Id, Adc0Id>;

pub const ADC0_CH1: Channel<Adc0Ch1Id, Adc0Id> = Channel { periph: ADC0, index: 1, id: Adc0Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch1Id {}
pub type Adc0Ch1 = Channel<Adc0Ch1Id, Adc0Id>;

pub const ADC0_CH2: Channel<Adc0Ch2Id, Adc0Id> = Channel { periph: ADC0, index: 2, id: Adc0Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch2Id {}
pub type Adc0Ch2 = Channel<Adc0Ch2Id, Adc0Id>;

pub const ADC0_CH3: Channel<Adc0Ch3Id, Adc0Id> = Channel { periph: ADC0, index: 3, id: Adc0Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch3Id {}
pub type Adc0Ch3 = Channel<Adc0Ch3Id, Adc0Id>;

pub const ADC0_CH4: Channel<Adc0Ch4Id, Adc0Id> = Channel { periph: ADC0, index: 4, id: Adc0Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch4Id {}
pub type Adc0Ch4 = Channel<Adc0Ch4Id, Adc0Id>;

pub const ADC0_CH5: Channel<Adc0Ch5Id, Adc0Id> = Channel { periph: ADC0, index: 5, id: Adc0Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch5Id {}
pub type Adc0Ch5 = Channel<Adc0Ch5Id, Adc0Id>;

pub const ADC0_CH6: Channel<Adc0Ch6Id, Adc0Id> = Channel { periph: ADC0, index: 6, id: Adc0Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch6Id {}
pub type Adc0Ch6 = Channel<Adc0Ch6Id, Adc0Id>;

pub const ADC0_CH7: Channel<Adc0Ch7Id, Adc0Id> = Channel { periph: ADC0, index: 7, id: Adc0Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch7Id {}
pub type Adc0Ch7 = Channel<Adc0Ch7Id, Adc0Id>;

pub const ADC0_CH8: Channel<Adc0Ch8Id, Adc0Id> = Channel { periph: ADC0, index: 8, id: Adc0Ch8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch8Id {}
pub type Adc0Ch8 = Channel<Adc0Ch8Id, Adc0Id>;

pub const ADC0_CH9: Channel<Adc0Ch9Id, Adc0Id> = Channel { periph: ADC0, index: 9, id: Adc0Ch9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch9Id {}
pub type Adc0Ch9 = Channel<Adc0Ch9Id, Adc0Id>;

pub const ADC0_CH10: Channel<Adc0Ch10Id, Adc0Id> = Channel { periph: ADC0, index: 10, id: Adc0Ch10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch10Id {}
pub type Adc0Ch10 = Channel<Adc0Ch10Id, Adc0Id>;

pub const ADC0_CH11: Channel<Adc0Ch11Id, Adc0Id> = Channel { periph: ADC0, index: 11, id: Adc0Ch11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch11Id {}
pub type Adc0Ch11 = Channel<Adc0Ch11Id, Adc0Id>;

pub const ADC0_CH12: Channel<Adc0Ch12Id, Adc0Id> = Channel { periph: ADC0, index: 12, id: Adc0Ch12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch12Id {}
pub type Adc0Ch12 = Channel<Adc0Ch12Id, Adc0Id>;

pub const ADC0_CH13: Channel<Adc0Ch13Id, Adc0Id> = Channel { periph: ADC0, index: 13, id: Adc0Ch13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch13Id {}
pub type Adc0Ch13 = Channel<Adc0Ch13Id, Adc0Id>;

pub const ADC0_CH14: Channel<Adc0Ch14Id, Adc0Id> = Channel { periph: ADC0, index: 14, id: Adc0Ch14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch14Id {}
pub type Adc0Ch14 = Channel<Adc0Ch14Id, Adc0Id>;

pub const ADC0_CH15: Channel<Adc0Ch15Id, Adc0Id> = Channel { periph: ADC0, index: 15, id: Adc0Ch15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch15Id {}
pub type Adc0Ch15 = Channel<Adc0Ch15Id, Adc0Id>;

pub const ADC0_CH16: Channel<Adc0Ch16Id, Adc0Id> = Channel { periph: ADC0, index: 16, id: Adc0Ch16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch16Id {}
pub type Adc0Ch16 = Channel<Adc0Ch16Id, Adc0Id>;

pub const ADC0_CH17: Channel<Adc0Ch17Id, Adc0Id> = Channel { periph: ADC0, index: 17, id: Adc0Ch17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch17Id {}
pub type Adc0Ch17 = Channel<Adc0Ch17Id, Adc0Id>;

pub const ADC0_CH18: Channel<Adc0Ch18Id, Adc0Id> = Channel { periph: ADC0, index: 18, id: Adc0Ch18Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch18Id {}
pub type Adc0Ch18 = Channel<Adc0Ch18Id, Adc0Id>;

pub const ADC0_CH19: Channel<Adc0Ch19Id, Adc0Id> = Channel { periph: ADC0, index: 19, id: Adc0Ch19Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch19Id {}
pub type Adc0Ch19 = Channel<Adc0Ch19Id, Adc0Id>;

pub const ADC0_CH20: Channel<Adc0Ch20Id, Adc0Id> = Channel { periph: ADC0, index: 20, id: Adc0Ch20Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch20Id {}
pub type Adc0Ch20 = Channel<Adc0Ch20Id, Adc0Id>;

pub const ADC0_CH21: Channel<Adc0Ch21Id, Adc0Id> = Channel { periph: ADC0, index: 21, id: Adc0Ch21Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch21Id {}
pub type Adc0Ch21 = Channel<Adc0Ch21Id, Adc0Id>;

pub const ADC0_CH22: Channel<Adc0Ch22Id, Adc0Id> = Channel { periph: ADC0, index: 22, id: Adc0Ch22Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch22Id {}
pub type Adc0Ch22 = Channel<Adc0Ch22Id, Adc0Id>;

pub const ADC0_CH23: Channel<Adc0Ch23Id, Adc0Id> = Channel { periph: ADC0, index: 23, id: Adc0Ch23Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch23Id {}
pub type Adc0Ch23 = Channel<Adc0Ch23Id, Adc0Id>;

pub const ADC0_TEMP: Channel<Adc0TempId, Adc0Id> = Channel { periph: ADC0, index: 26, id: Adc0TempId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0TempId {}
pub type Adc0Temp = Channel<Adc0TempId, Adc0Id>;

pub const ADC0_BANDGAP: Channel<Adc0BandgapId, Adc0Id> = Channel { periph: ADC0, index: 27, id: Adc0BandgapId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0BandgapId {}
pub type Adc0Bandgap = Channel<Adc0BandgapId, Adc0Id>;

pub const ADC0_REFSH: Channel<Adc0RefshId, Adc0Id> = Channel { periph: ADC0, index: 29, id: Adc0RefshId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0RefshId {}
pub type Adc0Refsh = Channel<Adc0RefshId, Adc0Id>;

pub const ADC0_REFSL: Channel<Adc0RefslId, Adc0Id> = Channel { periph: ADC0, index: 30, id: Adc0RefslId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0RefslId {}
pub type Adc0Refsl = Channel<Adc0RefslId, Adc0Id>;

