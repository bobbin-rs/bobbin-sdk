#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::spi::*;

periph!( SPI0, Spi0, SPI0_PERIPH, SpiPeriph, 0x4002c000, 0x0f);
periph!( SPI1, Spi1, SPI1_PERIPH, SpiPeriph, 0x4002d000, 0x10);
periph!( SPI2, Spi2, SPI2_PERIPH, SpiPeriph, 0x400ac000, 0x11);

