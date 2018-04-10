#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::spi::*;

periph!( SPI1, Spi1, SPI1_PERIPH, SpiPeriph, SPI1_OWNED, SPI1_REF_COUNT, 0x40013000, 0x00, 0x19);
periph!( SPI2, Spi2, SPI2_PERIPH, SpiPeriph, SPI2_OWNED, SPI2_REF_COUNT, 0x40003800, 0x01, 0x1a);
periph!( SPI3, Spi3, SPI3_PERIPH, SpiPeriph, SPI3_OWNED, SPI3_REF_COUNT, 0x40003c00, 0x02, 0x1b);

