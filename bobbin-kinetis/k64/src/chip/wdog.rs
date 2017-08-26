#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::wdog::*;

periph!(WdogPeriph, WDOG, Wdog, 0x40052000);



