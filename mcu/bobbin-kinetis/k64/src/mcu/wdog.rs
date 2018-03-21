#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::wdog::*;

periph!( WDOG, Wdog, WDOG_PERIPH, WdogPeriph, 0x40052000, 0x00, 0x07);

