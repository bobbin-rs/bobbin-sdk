#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::quadspi::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( QUADSPI, Quadspi, QUADSPI_PERIPH, QuadspiPeriph, QUADSPI_OWNED, 0xa0001000, 0x00, 0x0a);


