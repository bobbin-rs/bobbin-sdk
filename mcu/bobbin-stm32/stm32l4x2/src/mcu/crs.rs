#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::crs::*;

periph!( CRS, Crs, CRS_PERIPH, CrsPeriph, 0x40006000, 0x00, 0x36);

