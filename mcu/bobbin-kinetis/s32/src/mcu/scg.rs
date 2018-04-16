#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::scg::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( SCG, Scg, SCG_PERIPH, ScgPeriph, SCG_OWNED, SCG_REF_COUNT, 0x40064000, 0x00, 0x01);


