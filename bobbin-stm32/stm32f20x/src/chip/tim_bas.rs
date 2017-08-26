#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_bas::*;

periph!(TimBasPeriph, TIM6, Tim6, 0x40001000);
periph!(TimBasPeriph, TIM7, Tim7, 0x40001400);




