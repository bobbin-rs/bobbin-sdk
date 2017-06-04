pub use stm32_common::chip::tim_gen::*;

pub const TIM2: Tim2 = Periph(0x40000000, Tim2Id {});
pub const TIM3: Tim3 = Periph(0x40000400, Tim3Id {});
pub const TIM4: Tim4 = Periph(0x40000800, Tim4Id {});
pub const TIM15: Tim15 = Periph(0x40014000, Tim15Id {});
pub const TIM16: Tim16 = Periph(0x40014400, Tim16Id {});
pub const TIM17: Tim17 = Periph(0x40014800, Tim17Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim2Id {}
pub type Tim2 = Periph<Tim2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim3Id {}
pub type Tim3 = Periph<Tim3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim4Id {}
pub type Tim4 = Periph<Tim4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim15Id {}
pub type Tim15 = Periph<Tim15Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim16Id {}
pub type Tim16 = Periph<Tim16Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim17Id {}
pub type Tim17 = Periph<Tim17Id>;








