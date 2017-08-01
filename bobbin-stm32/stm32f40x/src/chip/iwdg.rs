pub use stm32_common::chip::iwdg::*;

pub const IWDG: Iwdg = Periph(0x40003000, IwdgId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct IwdgId {}
pub type Iwdg = Periph<IwdgId>;



