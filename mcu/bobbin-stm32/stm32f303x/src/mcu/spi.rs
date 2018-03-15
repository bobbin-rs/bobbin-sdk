#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::spi::*;

periph!( SPI1, Spi1, SPI1_PERIPH, SpiPeriph, 0x40013000, 0x19);
periph!( SPI2, Spi2, SPI2_PERIPH, SpiPeriph, 0x40003800, 0x1a);
periph!( SPI3, Spi3, SPI3_PERIPH, SpiPeriph, 0x40003c00, 0x1b);

