#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::smc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( SMC, Smc, SMC_PERIPH, SmcPeriph, SMC_OWNED, SMC_REF_COUNT, 0x4007e000, 0x00, 0x06);


