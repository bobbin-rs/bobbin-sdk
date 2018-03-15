#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::exti::*;

periph!( EXTI, Exti, _EXTI, ExtiPeriph, 0x40010400);




