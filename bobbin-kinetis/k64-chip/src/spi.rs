pub use kinetis_common::chip::spi::*;

pub const SPI0: Spi0 = Periph(0x4002c000, Spi0Id {});
pub const SPI1: Spi1 = Periph(0x4002d000, Spi1Id {});
pub const SPI2: Spi2 = Periph(0x400ac000, Spi2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Spi0Id {}
pub type Spi0 = Periph<Spi0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Spi1Id {}
pub type Spi1 = Periph<Spi1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Spi2Id {}
pub type Spi2 = Periph<Spi2Id>;





