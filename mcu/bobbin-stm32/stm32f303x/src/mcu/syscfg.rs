#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::syscfg::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( SYSCFG, Syscfg, SYSCFG_PERIPH, SyscfgPeriph, SYSCFG_OWNED, SYSCFG_REF_COUNT, 0x40010000, 0x00, 0x01);


