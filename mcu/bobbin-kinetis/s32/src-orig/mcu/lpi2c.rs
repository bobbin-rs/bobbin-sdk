#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::lpi2c::*;

periph!( LPI2C0, Lpi2c0, LPI2C0_PERIPH, Lpi2cPeriph, 0x40066000, 0x20);

