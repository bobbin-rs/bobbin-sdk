pub use kinetis_common::chip::dmamux::*;

pub const DMAMUX: Dmamux = Periph(0x40021000, DmamuxId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DmamuxId {}
pub type Dmamux = Periph<DmamuxId>;



