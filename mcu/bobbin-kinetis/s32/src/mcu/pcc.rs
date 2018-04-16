#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::pcc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( PCC, Pcc, PCC_PERIPH, PccPeriph, PCC_OWNED, PCC_REF_COUNT, 0x40065000, 0x00, 0x02);


