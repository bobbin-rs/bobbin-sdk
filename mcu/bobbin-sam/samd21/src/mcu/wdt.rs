#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::wdt::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( WDT, Wdt, WDT_PERIPH, WdtPeriph, 0x40001000, 0x04);


