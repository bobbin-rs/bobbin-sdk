#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::tim_adv::*;

periph!( TIM1, Tim1, TIM1_PERIPH, TimAdvPeriph, 0x40012c00, 0x10);

