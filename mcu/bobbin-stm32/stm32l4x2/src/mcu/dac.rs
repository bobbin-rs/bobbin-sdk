#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::dac::*;

periph!( DAC1, Dac1, DAC1_PERIPH, DacPeriph, 0x40007400, 0x01);

