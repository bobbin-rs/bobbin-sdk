#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_adv::*;

periph!(TimAdvPeriph, TIM1, Tim1, 0x40010000);
periph!(TimAdvPeriph, TIM8, Tim8, 0x40010400);




