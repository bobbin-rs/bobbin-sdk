#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::tim_adv::*;

periph!( TIM1, Tim1, TIM1_PERIPH, TimAdvPeriph, 0x40010000, 0x1a);
periph!( TIM8, Tim8, TIM8_PERIPH, TimAdvPeriph, 0x40010400, 0x1b);

