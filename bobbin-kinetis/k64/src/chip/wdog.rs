#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::wdog::*;

periph!( WDOG, Wdog, _WDOG, WdogPeriph, 0x40052000);



