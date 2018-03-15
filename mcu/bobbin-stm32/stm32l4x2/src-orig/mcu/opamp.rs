#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::opamp::*;

periph!( OPAMP, Opamp, OPAMP_PERIPH, OpampPeriph, 0x40007800, 0x35);

