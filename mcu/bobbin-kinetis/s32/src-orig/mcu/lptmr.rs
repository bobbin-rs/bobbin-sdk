#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::lptmr::*;

periph!( LPTMR0, Lptmr0, LPTMR0_PERIPH, LptmrPeriph, 0x40040000, 0x0f);

