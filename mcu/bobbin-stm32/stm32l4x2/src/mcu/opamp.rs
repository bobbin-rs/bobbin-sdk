#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::opamp::*;

periph!( OPAMP, Opamp, OPAMP_PERIPH, OpampPeriph, 0x40007800, 0x00, 0x35);

