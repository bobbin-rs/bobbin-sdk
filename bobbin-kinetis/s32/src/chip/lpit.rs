#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::lpit::*;

pub const LPIT0: Lpit0 = Periph(0x40037000, Lpit0Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpit0Id {}
pub type Lpit0 = Periph<Lpit0Id>;



pub const LPIT0_CH0: Channel<Lpit0Ch0Id, Lpit0Id> = Channel { periph: LPIT0, index: 0, id: Lpit0Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Lpit0Ch0Id {}
pub type Lpit0Ch0 = Channel<Lpit0Ch0Id, Lpit0Id>;

pub const LPIT0_CH1: Channel<Lpit0Ch1Id, Lpit0Id> = Channel { periph: LPIT0, index: 1, id: Lpit0Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Lpit0Ch1Id {}
pub type Lpit0Ch1 = Channel<Lpit0Ch1Id, Lpit0Id>;

pub const LPIT0_CH2: Channel<Lpit0Ch2Id, Lpit0Id> = Channel { periph: LPIT0, index: 2, id: Lpit0Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Lpit0Ch2Id {}
pub type Lpit0Ch2 = Channel<Lpit0Ch2Id, Lpit0Id>;

pub const LPIT0_CH3: Channel<Lpit0Ch3Id, Lpit0Id> = Channel { periph: LPIT0, index: 3, id: Lpit0Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Lpit0Ch3Id {}
pub type Lpit0Ch3 = Channel<Lpit0Ch3Id, Lpit0Id>;

