#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::lptim::*;

periph!( LPTIM, Lptim, LPTIM_PERIPH, LptimPeriph, 0x40007c00, 0x08);

