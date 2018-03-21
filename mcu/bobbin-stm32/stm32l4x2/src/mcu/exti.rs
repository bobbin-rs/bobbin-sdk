#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::exti::*;

periph!( EXTI, Exti, EXTI_PERIPH, ExtiPeriph, 0x40010400, 0x00, 0x30);

