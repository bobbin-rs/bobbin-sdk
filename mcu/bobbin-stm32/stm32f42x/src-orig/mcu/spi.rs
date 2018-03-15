#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::spi::*;

periph!( SPI1, Spi1, SPI1_PERIPH, SpiPeriph, 0x40013000, 0x1f);
periph!( SPI2, Spi2, SPI2_PERIPH, SpiPeriph, 0x40003800, 0x20);
periph!( SPI3, Spi3, SPI3_PERIPH, SpiPeriph, 0x40003c00, 0x21);
periph!( I2S2EXT, I2s2ext, I2S2EXT_PERIPH, SpiPeriph, 0x40003400, 0x22);
periph!( I2S3EXT, I2s3ext, I2S3EXT_PERIPH, SpiPeriph, 0x40004000, 0x23);
periph!( SPI4, Spi4, SPI4_PERIPH, SpiPeriph, 0x40013400, 0x24);
periph!( SPI5, Spi5, SPI5_PERIPH, SpiPeriph, 0x40015000, 0x25);
periph!( SPI6, Spi6, SPI6_PERIPH, SpiPeriph, 0x40015400, 0x26);

