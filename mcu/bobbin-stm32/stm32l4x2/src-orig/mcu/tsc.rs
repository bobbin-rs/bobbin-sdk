#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::tsc::*;

periph!( TSC, Tsc, TSC_PERIPH, TscPeriph, 0x40024000, 0x06);

