#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::lptmr::*;

periph!( LPTMR0, Lptmr0, LPTMR0_PERIPH, LptmrPeriph, LPTMR0_OWNED, LPTMR0_REF_COUNT, 0x40040000, 0x00, 0x0b);

