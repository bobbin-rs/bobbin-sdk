#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::pit::*;

pub const PIT: Pit = Periph(0x40037000, PitId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct PitId {}
pub type Pit = Periph<PitId>;



pub const PIT_CH0: Channel<PitCh0Id, PitId> = Channel { periph: PIT, index: 0, id: PitCh0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct PitCh0Id {}
pub type PitCh0 = Channel<PitCh0Id, PitId>;

pub const PIT_CH1: Channel<PitCh1Id, PitId> = Channel { periph: PIT, index: 1, id: PitCh1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct PitCh1Id {}
pub type PitCh1 = Channel<PitCh1Id, PitId>;

pub const PIT_CH2: Channel<PitCh2Id, PitId> = Channel { periph: PIT, index: 2, id: PitCh2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct PitCh2Id {}
pub type PitCh2 = Channel<PitCh2Id, PitId>;

pub const PIT_CH3: Channel<PitCh3Id, PitId> = Channel { periph: PIT, index: 3, id: PitCh3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct PitCh3Id {}
pub type PitCh3 = Channel<PitCh3Id, PitId>;

