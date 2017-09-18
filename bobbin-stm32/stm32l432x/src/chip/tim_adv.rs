#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_adv::*;

periph!( TIM1, Tim1, _TIM1, TimAdvPeriph, 0x40012c00);
periph!( TIM8, Tim8, _TIM8, TimAdvPeriph, 0x40013400);





