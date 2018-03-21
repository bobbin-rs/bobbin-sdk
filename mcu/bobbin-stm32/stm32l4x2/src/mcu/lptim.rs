#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::lptim::*;

periph!( LPTIM1, Lptim1, LPTIM1_PERIPH, LptimPeriph, 0x40007c00, 0x00, 0x25);
periph!( LPTIM2, Lptim2, LPTIM2_PERIPH, LptimPeriph, 0x40009400, 0x01, 0x26);

