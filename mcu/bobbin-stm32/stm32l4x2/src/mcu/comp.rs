#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::comp::*;

periph!( COMP, Comp, COMP_PERIPH, CompPeriph, 0x40010200, 0x00, 0x09);

