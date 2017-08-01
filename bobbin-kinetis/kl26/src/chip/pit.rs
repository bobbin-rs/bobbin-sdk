pub use kinetis_common::chip::pit::*;

pub const PIT: Pit = Periph(0x40037000, PitId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct PitId {}
pub type Pit = Periph<PitId>;



