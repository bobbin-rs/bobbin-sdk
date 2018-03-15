#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::spi::*;

periph!( SPI0, Spi0, SPI0_PERIPH, SpiPeriph, 0x40076000, 0x0c);
periph!( SPI1, Spi1, SPI1_PERIPH, SpiPeriph, 0x40077000, 0x0d);

