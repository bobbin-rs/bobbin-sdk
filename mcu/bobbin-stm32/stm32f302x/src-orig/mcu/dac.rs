#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::dac::*;

periph!( DAC, Dac, DAC_PERIPH, DacPeriph, 0x40007400, 0x1c);

