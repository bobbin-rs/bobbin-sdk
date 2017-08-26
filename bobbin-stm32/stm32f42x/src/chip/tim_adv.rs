#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::tim_adv::*;

pub const TIM1: Tim1 = Periph(0x40010000, Tim1Id {});
pub const TIM8: Tim8 = Periph(0x40010400, Tim8Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tim1Id {}
pub type Tim1 = Periph<Tim1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tim8Id {}
pub type Tim8 = Periph<Tim8Id>;

impl super::sig::Signal<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::SignalTim<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::Signal<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::SignalTim<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::Signal<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::SignalTim<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::Signal<super::sig::Tim1Ch4> for Tim1Ch4 {}
impl super::sig::SignalTim<super::sig::Tim1Ch4> for Tim1Ch4 {}

impl super::sig::Signal<super::sig::Tim8Ch1> for Tim8Ch1 {}
impl super::sig::SignalTim<super::sig::Tim8Ch1> for Tim8Ch1 {}
impl super::sig::Signal<super::sig::Tim8Ch2> for Tim8Ch2 {}
impl super::sig::SignalTim<super::sig::Tim8Ch2> for Tim8Ch2 {}
impl super::sig::Signal<super::sig::Tim8Ch3> for Tim8Ch3 {}
impl super::sig::SignalTim<super::sig::Tim8Ch3> for Tim8Ch3 {}
impl super::sig::Signal<super::sig::Tim8Ch4> for Tim8Ch4 {}
impl super::sig::SignalTim<super::sig::Tim8Ch4> for Tim8Ch4 {}


pub const TIM1_CH1: Channel<Tim1Ch1Id, Tim1Id> = Channel { periph: TIM1, index: 0, id: Tim1Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim1Ch1Id {}
pub type Tim1Ch1 = Channel<Tim1Ch1Id, Tim1Id>;

pub const TIM1_CH2: Channel<Tim1Ch2Id, Tim1Id> = Channel { periph: TIM1, index: 1, id: Tim1Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim1Ch2Id {}
pub type Tim1Ch2 = Channel<Tim1Ch2Id, Tim1Id>;

pub const TIM1_CH3: Channel<Tim1Ch3Id, Tim1Id> = Channel { periph: TIM1, index: 2, id: Tim1Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim1Ch3Id {}
pub type Tim1Ch3 = Channel<Tim1Ch3Id, Tim1Id>;

pub const TIM1_CH4: Channel<Tim1Ch4Id, Tim1Id> = Channel { periph: TIM1, index: 3, id: Tim1Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim1Ch4Id {}
pub type Tim1Ch4 = Channel<Tim1Ch4Id, Tim1Id>;

pub const TIM8_CH1: Channel<Tim8Ch1Id, Tim8Id> = Channel { periph: TIM8, index: 0, id: Tim8Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim8Ch1Id {}
pub type Tim8Ch1 = Channel<Tim8Ch1Id, Tim8Id>;

pub const TIM8_CH2: Channel<Tim8Ch2Id, Tim8Id> = Channel { periph: TIM8, index: 1, id: Tim8Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim8Ch2Id {}
pub type Tim8Ch2 = Channel<Tim8Ch2Id, Tim8Id>;

pub const TIM8_CH3: Channel<Tim8Ch3Id, Tim8Id> = Channel { periph: TIM8, index: 2, id: Tim8Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim8Ch3Id {}
pub type Tim8Ch3 = Channel<Tim8Ch3Id, Tim8Id>;

pub const TIM8_CH4: Channel<Tim8Ch4Id, Tim8Id> = Channel { periph: TIM8, index: 3, id: Tim8Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim8Ch4Id {}
pub type Tim8Ch4 = Channel<Tim8Ch4Id, Tim8Id>;

