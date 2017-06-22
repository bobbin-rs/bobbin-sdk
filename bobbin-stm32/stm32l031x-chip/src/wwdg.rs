pub use stm32_common::chip::wwdg::*;

pub const WWDG: Wwdg = Periph(0x40002c00, WwdgId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WwdgId {}
pub type Wwdg = Periph<WwdgId>;



