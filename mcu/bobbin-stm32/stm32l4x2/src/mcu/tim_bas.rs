#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::tim_bas::*;

periph!( TIM6, Tim6, TIM6_PERIPH, TimBasPeriph, 0x40001000, 0x00, 0x23);
periph!( TIM7, Tim7, TIM7_PERIPH, TimBasPeriph, 0x40001400, 0x01, 0x24);

