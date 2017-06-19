pub use kinetis_common::chip::dma::*;

pub const DMA: Dma = Periph(0x40008000, DmaId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DmaId {}
pub type Dma = Periph<DmaId>;



pub const DMA0: Channel<Dma0Id, DmaId> = Channel { periph: DMA, index: 0, id: Dma0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma0Id {}
pub type Dma0 = Channel<Dma0Id, DmaId>;

pub const DMA1: Channel<Dma1Id, DmaId> = Channel { periph: DMA, index: 1, id: Dma1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma1Id {}
pub type Dma1 = Channel<Dma1Id, DmaId>;

pub const DMA2: Channel<Dma2Id, DmaId> = Channel { periph: DMA, index: 2, id: Dma2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma2Id {}
pub type Dma2 = Channel<Dma2Id, DmaId>;

pub const DMA3: Channel<Dma3Id, DmaId> = Channel { periph: DMA, index: 3, id: Dma3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Dma3Id {}
pub type Dma3 = Channel<Dma3Id, DmaId>;

