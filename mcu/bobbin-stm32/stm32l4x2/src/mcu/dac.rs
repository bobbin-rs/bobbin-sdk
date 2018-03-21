#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::dac::*;

periph!( DAC1, Dac1, DAC1_PERIPH, DacPeriph, 0x40007400, 0x00, 0x01);

