#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::lptmr::*;

pub const LPTMR0: Lptmr0 = Periph(0x40040000, Lptmr0Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lptmr0Id {}
pub type Lptmr0 = Periph<Lptmr0Id>;



