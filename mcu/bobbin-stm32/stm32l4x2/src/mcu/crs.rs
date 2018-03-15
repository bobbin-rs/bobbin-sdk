#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::crs::*;

periph!( CRS, Crs, CRS_PERIPH, CrsPeriph, 0x40006000, 0x36);

