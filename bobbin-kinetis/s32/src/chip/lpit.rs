pub use kinetis_common::chip::lpit::*;

pub const LPIT0: Lpit0 = Periph(0x40037000, Lpit0Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpit0Id {}
pub type Lpit0 = Periph<Lpit0Id>;



