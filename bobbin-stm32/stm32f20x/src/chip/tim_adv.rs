#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_adv::*;

periph!(_TIM1, TimAdvPeriph, TIM1, Tim1, 0x40010000);
periph!(_TIM8, TimAdvPeriph, TIM8, Tim8, 0x40010400);




