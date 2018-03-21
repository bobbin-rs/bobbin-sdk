#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::dac::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( DAC, Dac, DAC_PERIPH, DacPeriph, 0x40007400, 0x00, 0x09);


