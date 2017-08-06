#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::flexcan::*;

pub const CAN0: Can0 = Periph(0x40024000, Can0Id {});
pub const CAN1: Can1 = Periph(0x40025000, Can1Id {});
pub const CAN2: Can2 = Periph(0x4002b000, Can2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Can0Id {}
pub type Can0 = Periph<Can0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Can1Id {}
pub type Can1 = Periph<Can1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Can2Id {}
pub type Can2 = Periph<Can2Id>;





