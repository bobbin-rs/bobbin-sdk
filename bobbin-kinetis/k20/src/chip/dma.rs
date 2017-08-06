#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::edma::*;

pub const DMA: Dma = Periph(0x40008000, DmaId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct DmaId {}
pub type Dma = Periph<DmaId>;



pub const DMA0: Channel<Dma0Id, DmaId> = Channel { periph: DMA, index: 0, id: Dma0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma0Id {}
pub type Dma0 = Channel<Dma0Id, DmaId>;

pub const DMA1: Channel<Dma1Id, DmaId> = Channel { periph: DMA, index: 1, id: Dma1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Id {}
pub type Dma1 = Channel<Dma1Id, DmaId>;

pub const DMA2: Channel<Dma2Id, DmaId> = Channel { periph: DMA, index: 2, id: Dma2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Id {}
pub type Dma2 = Channel<Dma2Id, DmaId>;

pub const DMA3: Channel<Dma3Id, DmaId> = Channel { periph: DMA, index: 3, id: Dma3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma3Id {}
pub type Dma3 = Channel<Dma3Id, DmaId>;

pub const DMA4: Channel<Dma4Id, DmaId> = Channel { periph: DMA, index: 4, id: Dma4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma4Id {}
pub type Dma4 = Channel<Dma4Id, DmaId>;

pub const DMA5: Channel<Dma5Id, DmaId> = Channel { periph: DMA, index: 5, id: Dma5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma5Id {}
pub type Dma5 = Channel<Dma5Id, DmaId>;

pub const DMA6: Channel<Dma6Id, DmaId> = Channel { periph: DMA, index: 6, id: Dma6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma6Id {}
pub type Dma6 = Channel<Dma6Id, DmaId>;

pub const DMA7: Channel<Dma7Id, DmaId> = Channel { periph: DMA, index: 7, id: Dma7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma7Id {}
pub type Dma7 = Channel<Dma7Id, DmaId>;

pub const DMA8: Channel<Dma8Id, DmaId> = Channel { periph: DMA, index: 8, id: Dma8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma8Id {}
pub type Dma8 = Channel<Dma8Id, DmaId>;

pub const DMA9: Channel<Dma9Id, DmaId> = Channel { periph: DMA, index: 9, id: Dma9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma9Id {}
pub type Dma9 = Channel<Dma9Id, DmaId>;

pub const DMA10: Channel<Dma10Id, DmaId> = Channel { periph: DMA, index: 10, id: Dma10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma10Id {}
pub type Dma10 = Channel<Dma10Id, DmaId>;

pub const DMA11: Channel<Dma11Id, DmaId> = Channel { periph: DMA, index: 11, id: Dma11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma11Id {}
pub type Dma11 = Channel<Dma11Id, DmaId>;

pub const DMA12: Channel<Dma12Id, DmaId> = Channel { periph: DMA, index: 12, id: Dma12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma12Id {}
pub type Dma12 = Channel<Dma12Id, DmaId>;

pub const DMA13: Channel<Dma13Id, DmaId> = Channel { periph: DMA, index: 13, id: Dma13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma13Id {}
pub type Dma13 = Channel<Dma13Id, DmaId>;

pub const DMA14: Channel<Dma14Id, DmaId> = Channel { periph: DMA, index: 14, id: Dma14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma14Id {}
pub type Dma14 = Channel<Dma14Id, DmaId>;

pub const DMA15: Channel<Dma15Id, DmaId> = Channel { periph: DMA, index: 15, id: Dma15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma15Id {}
pub type Dma15 = Channel<Dma15Id, DmaId>;

