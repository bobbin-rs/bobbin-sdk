#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::syscfg::*;

periph!( SYSCFG, Syscfg, SYSCFG_PERIPH, SyscfgPeriph, 0x40010000, 0x11);

