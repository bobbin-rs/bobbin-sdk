pub use stm32_common::chip::tim_gen::*;

pub const TIM2: Tim2 = Periph(0x40000000, Tim2Id {});
pub const TIM3: Tim3 = Periph(0x40000400, Tim3Id {});
pub const TIM4: Tim4 = Periph(0x40000800, Tim4Id {});
pub const TIM15: Tim15 = Periph(0x40014000, Tim15Id {});
pub const TIM16: Tim16 = Periph(0x40014400, Tim16Id {});
pub const TIM17: Tim17 = Periph(0x40014800, Tim17Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tim2Id {}
pub type Tim2 = Periph<Tim2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tim3Id {}
pub type Tim3 = Periph<Tim3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tim4Id {}
pub type Tim4 = Periph<Tim4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tim15Id {}
pub type Tim15 = Periph<Tim15Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tim16Id {}
pub type Tim16 = Periph<Tim16Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tim17Id {}
pub type Tim17 = Periph<Tim17Id>;

impl super::sig::Signal<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::SignalTim<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::Signal<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::SignalTim<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::Signal<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::SignalTim<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::Signal<super::sig::Tim2Ch4> for Tim2Ch4 {}
impl super::sig::SignalTim<super::sig::Tim2Ch4> for Tim2Ch4 {}

impl super::sig::Signal<super::sig::Tim3Ch1> for Tim3Ch1 {}
impl super::sig::SignalTim<super::sig::Tim3Ch1> for Tim3Ch1 {}
impl super::sig::Signal<super::sig::Tim3Ch2> for Tim3Ch2 {}
impl super::sig::SignalTim<super::sig::Tim3Ch2> for Tim3Ch2 {}
impl super::sig::Signal<super::sig::Tim3Ch3> for Tim3Ch3 {}
impl super::sig::SignalTim<super::sig::Tim3Ch3> for Tim3Ch3 {}
impl super::sig::Signal<super::sig::Tim3Ch4> for Tim3Ch4 {}
impl super::sig::SignalTim<super::sig::Tim3Ch4> for Tim3Ch4 {}

impl super::sig::Signal<super::sig::Tim4Ch1> for Tim4Ch1 {}
impl super::sig::SignalTim<super::sig::Tim4Ch1> for Tim4Ch1 {}
impl super::sig::Signal<super::sig::Tim4Ch2> for Tim4Ch2 {}
impl super::sig::SignalTim<super::sig::Tim4Ch2> for Tim4Ch2 {}
impl super::sig::Signal<super::sig::Tim4Ch3> for Tim4Ch3 {}
impl super::sig::SignalTim<super::sig::Tim4Ch3> for Tim4Ch3 {}
impl super::sig::Signal<super::sig::Tim4Ch4> for Tim4Ch4 {}
impl super::sig::SignalTim<super::sig::Tim4Ch4> for Tim4Ch4 {}

impl super::sig::Signal<super::sig::Tim15Ch1> for Tim15Ch1 {}
impl super::sig::SignalTim<super::sig::Tim15Ch1> for Tim15Ch1 {}
impl super::sig::Signal<super::sig::Tim15Ch2> for Tim15Ch2 {}
impl super::sig::SignalTim<super::sig::Tim15Ch2> for Tim15Ch2 {}

impl super::sig::Signal<super::sig::Tim16Ch1> for Tim16Ch1 {}
impl super::sig::SignalTim<super::sig::Tim16Ch1> for Tim16Ch1 {}
impl super::sig::Signal<super::sig::Tim16Ch1n> for Tim16Ch1 {}
impl super::sig::SignalTimn<super::sig::Tim16Ch1n> for Tim16Ch1 {}

impl super::sig::Signal<super::sig::Tim17Ch1> for Tim17Ch1 {}
impl super::sig::SignalTim<super::sig::Tim17Ch1> for Tim17Ch1 {}
impl super::sig::Signal<super::sig::Tim17Ch1n> for Tim17Ch1 {}
impl super::sig::SignalTimn<super::sig::Tim17Ch1n> for Tim17Ch1 {}


pub const TIM2_CH1: Channel<Tim2Ch1Id, Tim2Id> = Channel { periph: TIM2, index: 0, id: Tim2Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim2Ch1Id {}
pub type Tim2Ch1 = Channel<Tim2Ch1Id, Tim2Id>;

pub const TIM2_CH2: Channel<Tim2Ch2Id, Tim2Id> = Channel { periph: TIM2, index: 1, id: Tim2Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim2Ch2Id {}
pub type Tim2Ch2 = Channel<Tim2Ch2Id, Tim2Id>;

pub const TIM2_CH3: Channel<Tim2Ch3Id, Tim2Id> = Channel { periph: TIM2, index: 2, id: Tim2Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim2Ch3Id {}
pub type Tim2Ch3 = Channel<Tim2Ch3Id, Tim2Id>;

pub const TIM2_CH4: Channel<Tim2Ch4Id, Tim2Id> = Channel { periph: TIM2, index: 3, id: Tim2Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim2Ch4Id {}
pub type Tim2Ch4 = Channel<Tim2Ch4Id, Tim2Id>;

pub const TIM3_CH1: Channel<Tim3Ch1Id, Tim3Id> = Channel { periph: TIM3, index: 0, id: Tim3Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim3Ch1Id {}
pub type Tim3Ch1 = Channel<Tim3Ch1Id, Tim3Id>;

pub const TIM3_CH2: Channel<Tim3Ch2Id, Tim3Id> = Channel { periph: TIM3, index: 1, id: Tim3Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim3Ch2Id {}
pub type Tim3Ch2 = Channel<Tim3Ch2Id, Tim3Id>;

pub const TIM3_CH3: Channel<Tim3Ch3Id, Tim3Id> = Channel { periph: TIM3, index: 2, id: Tim3Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim3Ch3Id {}
pub type Tim3Ch3 = Channel<Tim3Ch3Id, Tim3Id>;

pub const TIM3_CH4: Channel<Tim3Ch4Id, Tim3Id> = Channel { periph: TIM3, index: 3, id: Tim3Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim3Ch4Id {}
pub type Tim3Ch4 = Channel<Tim3Ch4Id, Tim3Id>;

pub const TIM4_CH1: Channel<Tim4Ch1Id, Tim4Id> = Channel { periph: TIM4, index: 0, id: Tim4Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim4Ch1Id {}
pub type Tim4Ch1 = Channel<Tim4Ch1Id, Tim4Id>;

pub const TIM4_CH2: Channel<Tim4Ch2Id, Tim4Id> = Channel { periph: TIM4, index: 1, id: Tim4Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim4Ch2Id {}
pub type Tim4Ch2 = Channel<Tim4Ch2Id, Tim4Id>;

pub const TIM4_CH3: Channel<Tim4Ch3Id, Tim4Id> = Channel { periph: TIM4, index: 2, id: Tim4Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim4Ch3Id {}
pub type Tim4Ch3 = Channel<Tim4Ch3Id, Tim4Id>;

pub const TIM4_CH4: Channel<Tim4Ch4Id, Tim4Id> = Channel { periph: TIM4, index: 3, id: Tim4Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim4Ch4Id {}
pub type Tim4Ch4 = Channel<Tim4Ch4Id, Tim4Id>;

pub const TIM15_CH1: Channel<Tim15Ch1Id, Tim15Id> = Channel { periph: TIM15, index: 0, id: Tim15Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim15Ch1Id {}
pub type Tim15Ch1 = Channel<Tim15Ch1Id, Tim15Id>;

pub const TIM15_CH2: Channel<Tim15Ch2Id, Tim15Id> = Channel { periph: TIM15, index: 1, id: Tim15Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim15Ch2Id {}
pub type Tim15Ch2 = Channel<Tim15Ch2Id, Tim15Id>;

pub const TIM16_CH1: Channel<Tim16Ch1Id, Tim16Id> = Channel { periph: TIM16, index: 0, id: Tim16Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim16Ch1Id {}
pub type Tim16Ch1 = Channel<Tim16Ch1Id, Tim16Id>;

pub const TIM17_CH1: Channel<Tim17Ch1Id, Tim17Id> = Channel { periph: TIM17, index: 0, id: Tim17Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Tim17Ch1Id {}
pub type Tim17Ch1 = Channel<Tim17Ch1Id, Tim17Id>;

