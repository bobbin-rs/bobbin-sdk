#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::rcm::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( RCM, Rcm, RCM_PERIPH, RcmPeriph, RCM_OWNED, RCM_REF_COUNT, 0x4007f000, 0x00, 0x04);


