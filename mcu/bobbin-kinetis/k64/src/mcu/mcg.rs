#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::mcg::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( MCG, Mcg, MCG_PERIPH, McgPeriph, 0x40064000, 0x01);


