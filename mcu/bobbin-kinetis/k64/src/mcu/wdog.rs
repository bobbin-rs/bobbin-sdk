#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::wdog::*;

periph!( WDOG, Wdog, WDOG_PERIPH, WdogPeriph, 0x40052000, 0x07);

