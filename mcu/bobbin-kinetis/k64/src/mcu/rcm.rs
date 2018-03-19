#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::rcm::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( RCM, Rcm, RCM_PERIPH, RcmPeriph, 0x4007f000, 0x04);


