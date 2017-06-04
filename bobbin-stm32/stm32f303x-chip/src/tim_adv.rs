pub use stm32_common::chip::tim_adv::*;

pub const TIM1: Tim1 = Periph(0x40012c00, Tim1Id {});
pub const TIM8: Tim8 = Periph(0x40013400, Tim8Id {});
pub const TIM20: Tim20 = Periph(0x40015000, Tim20Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim1Id {}
pub type Tim1 = Periph<Tim1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim8Id {}
pub type Tim8 = Periph<Tim8Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim20Id {}
pub type Tim20 = Periph<Tim20Id>;





