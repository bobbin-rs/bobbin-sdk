#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_gen::*;

periph!( TIM2, Tim2, _TIM2, TimGenPeriph, 0x40000000);
periph!( TIM3, Tim3, _TIM3, TimGenPeriph, 0x40000400);
periph!( TIM15, Tim15, _TIM15, TimGenPeriph, 0x40014000);
periph!( TIM16, Tim16, _TIM16, TimGenPeriph, 0x40014400);







