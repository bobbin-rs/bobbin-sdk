pub use ::stm32_common::spi_v1::*;

::bobbin_mcu::periph!( SPI1, Spi1, SPI1_PERIPH, SpiPeriph, SPI1_OWNED, SPI1_REF_COUNT, 0x40013000, 0x00, 0x1f);
::bobbin_mcu::periph!( SPI2, Spi2, SPI2_PERIPH, SpiPeriph, SPI2_OWNED, SPI2_REF_COUNT, 0x40003800, 0x01, 0x20);
::bobbin_mcu::periph!( SPI3, Spi3, SPI3_PERIPH, SpiPeriph, SPI3_OWNED, SPI3_REF_COUNT, 0x40003c00, 0x02, 0x21);
::bobbin_mcu::periph!( I2S2EXT, I2s2ext, I2S2EXT_PERIPH, SpiPeriph, I2S2EXT_OWNED, I2S2EXT_REF_COUNT, 0x40003400, 0x03, 0x22);
::bobbin_mcu::periph!( I2S3EXT, I2s3ext, I2S3EXT_PERIPH, SpiPeriph, I2S3EXT_OWNED, I2S3EXT_REF_COUNT, 0x40004000, 0x04, 0x23);
::bobbin_mcu::periph!( SPI4, Spi4, SPI4_PERIPH, SpiPeriph, SPI4_OWNED, SPI4_REF_COUNT, 0x40013400, 0x05, 0x24);
::bobbin_mcu::periph!( SPI5, Spi5, SPI5_PERIPH, SpiPeriph, SPI5_OWNED, SPI5_REF_COUNT, 0x40015000, 0x06, 0x25);
::bobbin_mcu::periph!( SPI6, Spi6, SPI6_PERIPH, SpiPeriph, SPI6_OWNED, SPI6_REF_COUNT, 0x40015400, 0x07, 0x26);

