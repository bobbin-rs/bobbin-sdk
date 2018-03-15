#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::quadspi::*;

periph!( QUADSPI, Quadspi, QUADSPI_PERIPH, QuadspiPeriph, 0xa0001000, 0x3a);

