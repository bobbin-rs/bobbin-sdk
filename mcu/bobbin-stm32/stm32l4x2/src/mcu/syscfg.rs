#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::syscfg::*;

periph!( SYSCFG, Syscfg, SYSCFG_PERIPH, SyscfgPeriph, 0x40010000, 0x00, 0x11);

