#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::spi_v1::*;

periph!( SPI1, Spi1, _SPI1, SpiPeriph, 0x40013000);
periph!( SPI2, Spi2, _SPI2, SpiPeriph, 0x40003800);
periph!( SPI3, Spi3, _SPI3, SpiPeriph, 0x40003c00);






