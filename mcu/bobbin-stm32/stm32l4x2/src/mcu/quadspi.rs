#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::quadspi::*;

periph!( QUADSPI, Quadspi, QUADSPI_PERIPH, QuadspiPeriph, 0xa0001000, 0x00, 0x3a);

