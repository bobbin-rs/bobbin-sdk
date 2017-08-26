#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::dma::*;

pub const DMA1: Dma1 = Periph(0x40026000, Dma1Id {});
pub const DMA2: Dma2 = Periph(0x40026400, Dma2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Dma1Id {}
pub type Dma1 = Periph<Dma1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Dma2Id {}
pub type Dma2 = Periph<Dma2Id>;




pub const DMA1_STREAM0: Channel<Dma1Stream0Id, Dma1Id> = Channel { periph: DMA1, index: 0, id: Dma1Stream0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Stream0Id {}
pub type Dma1Stream0 = Channel<Dma1Stream0Id, Dma1Id>;

pub const DMA1_STREAM1: Channel<Dma1Stream1Id, Dma1Id> = Channel { periph: DMA1, index: 1, id: Dma1Stream1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Stream1Id {}
pub type Dma1Stream1 = Channel<Dma1Stream1Id, Dma1Id>;

pub const DMA1_STREAM2: Channel<Dma1Stream2Id, Dma1Id> = Channel { periph: DMA1, index: 2, id: Dma1Stream2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Stream2Id {}
pub type Dma1Stream2 = Channel<Dma1Stream2Id, Dma1Id>;

pub const DMA1_STREAM3: Channel<Dma1Stream3Id, Dma1Id> = Channel { periph: DMA1, index: 3, id: Dma1Stream3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Stream3Id {}
pub type Dma1Stream3 = Channel<Dma1Stream3Id, Dma1Id>;

pub const DMA1_STREAM4: Channel<Dma1Stream4Id, Dma1Id> = Channel { periph: DMA1, index: 4, id: Dma1Stream4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Stream4Id {}
pub type Dma1Stream4 = Channel<Dma1Stream4Id, Dma1Id>;

pub const DMA1_STREAM5: Channel<Dma1Stream5Id, Dma1Id> = Channel { periph: DMA1, index: 5, id: Dma1Stream5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Stream5Id {}
pub type Dma1Stream5 = Channel<Dma1Stream5Id, Dma1Id>;

pub const DMA1_STREAM6: Channel<Dma1Stream6Id, Dma1Id> = Channel { periph: DMA1, index: 6, id: Dma1Stream6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Stream6Id {}
pub type Dma1Stream6 = Channel<Dma1Stream6Id, Dma1Id>;

pub const DMA1_STREAM7: Channel<Dma1Stream7Id, Dma1Id> = Channel { periph: DMA1, index: 7, id: Dma1Stream7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Stream7Id {}
pub type Dma1Stream7 = Channel<Dma1Stream7Id, Dma1Id>;

pub const DMA2_STREAM0: Channel<Dma2Stream0Id, Dma2Id> = Channel { periph: DMA2, index: 0, id: Dma2Stream0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Stream0Id {}
pub type Dma2Stream0 = Channel<Dma2Stream0Id, Dma2Id>;

pub const DMA2_STREAM1: Channel<Dma2Stream1Id, Dma2Id> = Channel { periph: DMA2, index: 1, id: Dma2Stream1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Stream1Id {}
pub type Dma2Stream1 = Channel<Dma2Stream1Id, Dma2Id>;

pub const DMA2_STREAM2: Channel<Dma2Stream2Id, Dma2Id> = Channel { periph: DMA2, index: 2, id: Dma2Stream2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Stream2Id {}
pub type Dma2Stream2 = Channel<Dma2Stream2Id, Dma2Id>;

pub const DMA2_STREAM3: Channel<Dma2Stream3Id, Dma2Id> = Channel { periph: DMA2, index: 3, id: Dma2Stream3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Stream3Id {}
pub type Dma2Stream3 = Channel<Dma2Stream3Id, Dma2Id>;

pub const DMA2_STREAM4: Channel<Dma2Stream4Id, Dma2Id> = Channel { periph: DMA2, index: 4, id: Dma2Stream4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Stream4Id {}
pub type Dma2Stream4 = Channel<Dma2Stream4Id, Dma2Id>;

pub const DMA2_STREAM5: Channel<Dma2Stream5Id, Dma2Id> = Channel { periph: DMA2, index: 5, id: Dma2Stream5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Stream5Id {}
pub type Dma2Stream5 = Channel<Dma2Stream5Id, Dma2Id>;

pub const DMA2_STREAM6: Channel<Dma2Stream6Id, Dma2Id> = Channel { periph: DMA2, index: 6, id: Dma2Stream6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Stream6Id {}
pub type Dma2Stream6 = Channel<Dma2Stream6Id, Dma2Id>;

pub const DMA2_STREAM7: Channel<Dma2Stream7Id, Dma2Id> = Channel { periph: DMA2, index: 7, id: Dma2Stream7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Stream7Id {}
pub type Dma2Stream7 = Channel<Dma2Stream7Id, Dma2Id>;

