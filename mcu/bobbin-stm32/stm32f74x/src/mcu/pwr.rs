#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::pwr::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( PWR, Pwr, PWR_PERIPH, PwrPeriph, PWR_OWNED, PWR_REF_COUNT, 0x40007000, 0x00, 0x02);


