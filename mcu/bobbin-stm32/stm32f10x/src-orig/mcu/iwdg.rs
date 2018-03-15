#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::iwdg::*;

periph!( IWDG, Iwdg, IWDG_PERIPH, IwdgPeriph, 0x40003000, 0x09);

