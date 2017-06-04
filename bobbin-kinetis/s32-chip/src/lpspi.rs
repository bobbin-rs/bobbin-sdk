pub use kinetis_common::chip::lpspi::*;

pub const LPSPI0: Lpspi0 = Periph(0x4002c000, Lpspi0Id {});
pub const LPSPI1: Lpspi1 = Periph(0x4002d000, Lpspi1Id {});
pub const LPSPI2: Lpspi2 = Periph(0x4002e000, Lpspi2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lpspi0Id {}
pub type Lpspi0 = Periph<Lpspi0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lpspi1Id {}
pub type Lpspi1 = Periph<Lpspi1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lpspi2Id {}
pub type Lpspi2 = Periph<Lpspi2Id>;





