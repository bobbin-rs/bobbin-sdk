pub use kinetis_common::chip::wdog::*;

pub const WDOG: Wdog = Periph(0x40052000, WdogId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WdogId {}
pub type Wdog = Periph<WdogId>;



