#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::spi::*;

periph!( SPI0, Spi0, SPI0_PERIPH, SpiPeriph, SPI0_OWNED, SPI0_REF_COUNT, 0x4002c000, 0x00, 0x0a);
periph!( SPI1, Spi1, SPI1_PERIPH, SpiPeriph, SPI1_OWNED, SPI1_REF_COUNT, 0x4002d000, 0x01, 0x0b);
periph!( SPI2, Spi2, SPI2_PERIPH, SpiPeriph, SPI2_OWNED, SPI2_REF_COUNT, 0x400ac000, 0x02, 0x0c);

