#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::tim_bas::*;

periph!( TIM6, Tim6, TIM6_PERIPH, TimBasPeriph, 0x40001000, 0x14);
periph!( TIM7, Tim7, TIM7_PERIPH, TimBasPeriph, 0x40001400, 0x15);

