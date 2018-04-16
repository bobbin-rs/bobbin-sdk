#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::spi::*;

periph!( SPI0, Spi0, SPI0_PERIPH, SpiPeriph, SPI0_OWNED, SPI0_REF_COUNT, 0x40076000, 0x00, 0x0c);
periph!( SPI1, Spi1, SPI1_PERIPH, SpiPeriph, SPI1_OWNED, SPI1_REF_COUNT, 0x40077000, 0x01, 0x0d);

