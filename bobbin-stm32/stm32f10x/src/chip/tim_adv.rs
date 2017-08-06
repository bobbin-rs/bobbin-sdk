#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::tim_adv::*;

pub const TIM1: Tim1 = Periph(0x40012c00, Tim1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tim1Id {}
pub type Tim1 = Periph<Tim1Id>;



