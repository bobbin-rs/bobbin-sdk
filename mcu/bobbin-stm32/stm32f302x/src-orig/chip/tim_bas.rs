#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_bas::*;

periph!( TIM6, Tim6, _TIM6, TimBasPeriph, 0x40001000);
periph!( TIM7, Tim7, _TIM7, TimBasPeriph, 0x40001400);





