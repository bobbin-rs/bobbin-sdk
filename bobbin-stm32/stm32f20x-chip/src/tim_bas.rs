pub use stm32_common::chip::tim_bas::*;

pub const TIM6: Tim6 = Periph(0x40001000, Tim6Id {});
pub const TIM7: Tim7 = Periph(0x40001400, Tim7Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim6Id {}
pub type Tim6 = Periph<Tim6Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim7Id {}
pub type Tim7 = Periph<Tim7Id>;




