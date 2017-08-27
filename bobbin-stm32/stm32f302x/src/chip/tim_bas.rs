#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_bas::*;

periph!(_TIM6, TimBasPeriph, TIM6, Tim6, 0x40001000);
periph!(_TIM7, TimBasPeriph, TIM7, Tim7, 0x40001400);




