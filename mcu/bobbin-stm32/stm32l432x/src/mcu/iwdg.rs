#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::iwdg::*;

periph!( IWDG, Iwdg, IWDG_PERIPH, IwdgPeriph, 0x40003000, 0x00, 0x07);

