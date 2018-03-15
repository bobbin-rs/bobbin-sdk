#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::swpmi::*;

periph!( SWPMI1, Swpmi1, SWPMI1_PERIPH, SwpmiPeriph, 0x40008800, 0x34);

