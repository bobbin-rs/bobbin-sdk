#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::spi::*;

periph!(SpiPeriph, SPI0, Spi0, 0x4002c000);
periph!(SpiPeriph, SPI1, Spi1, 0x4002d000);
periph!(SpiPeriph, SPI2, Spi2, 0x400ac000);





