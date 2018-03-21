#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::dbgmcu::*;

periph!( DBGMCU, Dbgmcu, DBGMCU_PERIPH, DbgmcuPeriph, 0xe0042000, 0x00, 0x3b);

