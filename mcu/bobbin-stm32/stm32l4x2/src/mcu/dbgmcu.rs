#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::dbgmcu::*;

periph!( DBGMCU, Dbgmcu, DBGMCU_PERIPH, DbgmcuPeriph, 0xe0042000, 0x3b);

