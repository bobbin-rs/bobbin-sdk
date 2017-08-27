#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::spi::*;

periph!(_SPI0, SpiPeriph, SPI0, Spi0, 0x4002c000);
periph!(_SPI1, SpiPeriph, SPI1, Spi1, 0x4002d000);
periph!(_SPI2, SpiPeriph, SPI2, Spi2, 0x400ac000);





