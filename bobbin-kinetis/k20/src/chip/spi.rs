#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::spi::*;

periph!( SPI0, Spi0, _SPI0, SpiPeriph, 0x4002c000);
periph!( SPI1, Spi1, _SPI1, SpiPeriph, 0x4002d000);
periph!( SPI2, Spi2, _SPI2, SpiPeriph, 0x400ac000);






