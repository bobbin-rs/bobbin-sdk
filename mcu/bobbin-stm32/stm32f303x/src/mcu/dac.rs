#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dac::*;

periph!( DAC, Dac, DAC_PERIPH, DacPeriph, DAC_OWNED, DAC_REF_COUNT, 0x40007400, 0x00, 0x22);

