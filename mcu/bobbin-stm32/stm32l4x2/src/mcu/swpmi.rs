#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::swpmi::*;

periph!( SWPMI1, Swpmi1, SWPMI1_PERIPH, SwpmiPeriph, 0x40008800, 0x00, 0x34);

