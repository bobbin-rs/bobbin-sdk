#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::lptim::*;

pub const LPTIM: Lptim = Periph(0x40007c00, LptimId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct LptimId {}
pub type Lptim = Periph<LptimId>;



