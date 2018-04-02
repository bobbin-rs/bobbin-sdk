#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dbg::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( DBG, Dbg, DBG_PERIPH, DbgPeriph, DBG_OWNED, DBG_REF_COUNT, 0xe0042000, 0x00, 0x04);


