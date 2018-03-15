#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::exti::*;

periph!( EXTI, Exti, EXTI_PERIPH, ExtiPeriph, 0x40010400, 0x30);

