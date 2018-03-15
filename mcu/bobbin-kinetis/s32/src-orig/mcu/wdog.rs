#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::wdog::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( WDOG, Wdog, WDOG_PERIPH, WdogPeriph, 0x40052000, 0x04);


