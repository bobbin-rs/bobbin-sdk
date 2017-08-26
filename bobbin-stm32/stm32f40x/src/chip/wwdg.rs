#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::wwdg::*;

pub const WWDG: Wwdg = Periph(0x40002c00, WwdgId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct WwdgId {}
pub type Wwdg = Periph<WwdgId>;



