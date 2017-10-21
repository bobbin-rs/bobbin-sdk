#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::spi_v1::*;

periph!( SPI1, Spi1, _SPI1, SpiPeriph, 0x40013000);
periph!( SPI2, Spi2, _SPI2, SpiPeriph, 0x40003800);
periph!( SPI3, Spi3, _SPI3, SpiPeriph, 0x40003c00);
periph!( I2S2EXT, I2s2ext, _I2S2EXT, SpiPeriph, 0x40003400);
periph!( I2S3EXT, I2s3ext, _I2S3EXT, SpiPeriph, 0x40004000);
periph!( SPI4, Spi4, _SPI4, SpiPeriph, 0x40013400);
periph!( SPI5, Spi5, _SPI5, SpiPeriph, 0x40015000);
periph!( SPI6, Spi6, _SPI6, SpiPeriph, 0x40015400);

impl super::sig::Signal<super::sig::Spi1Nss> for Spi1 {}
impl super::sig::SignalSpiNss<super::sig::Spi1Nss> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Miso> for Spi1 {}
impl super::sig::SignalSpiMiso<super::sig::Spi1Miso> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Mosi> for Spi1 {}
impl super::sig::SignalSpiMosi<super::sig::Spi1Mosi> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Sck> for Spi1 {}
impl super::sig::SignalSpiSck<super::sig::Spi1Sck> for Spi1 {}

impl super::sig::Signal<super::sig::Spi2Nss> for Spi2 {}
impl super::sig::SignalSpiNss<super::sig::Spi2Nss> for Spi2 {}
impl super::sig::Signal<super::sig::Spi2Miso> for Spi2 {}
impl super::sig::SignalSpiMiso<super::sig::Spi2Miso> for Spi2 {}
impl super::sig::Signal<super::sig::Spi2Mosi> for Spi2 {}
impl super::sig::SignalSpiMosi<super::sig::Spi2Mosi> for Spi2 {}
impl super::sig::Signal<super::sig::Spi2Sck> for Spi2 {}
impl super::sig::SignalSpiSck<super::sig::Spi2Sck> for Spi2 {}

impl super::sig::Signal<super::sig::Spi3Nss> for Spi3 {}
impl super::sig::SignalSpiNss<super::sig::Spi3Nss> for Spi3 {}
impl super::sig::Signal<super::sig::Spi3Miso> for Spi3 {}
impl super::sig::SignalSpiMiso<super::sig::Spi3Miso> for Spi3 {}
impl super::sig::Signal<super::sig::Spi3Mosi> for Spi3 {}
impl super::sig::SignalSpiMosi<super::sig::Spi3Mosi> for Spi3 {}
impl super::sig::Signal<super::sig::Spi3Sck> for Spi3 {}
impl super::sig::SignalSpiSck<super::sig::Spi3Sck> for Spi3 {}



impl super::sig::Signal<super::sig::Spi4Nss> for Spi4 {}
impl super::sig::SignalSpiNss<super::sig::Spi4Nss> for Spi4 {}
impl super::sig::Signal<super::sig::Spi4Miso> for Spi4 {}
impl super::sig::SignalSpiMiso<super::sig::Spi4Miso> for Spi4 {}
impl super::sig::Signal<super::sig::Spi4Mosi> for Spi4 {}
impl super::sig::SignalSpiMosi<super::sig::Spi4Mosi> for Spi4 {}
impl super::sig::Signal<super::sig::Spi4Sck> for Spi4 {}
impl super::sig::SignalSpiSck<super::sig::Spi4Sck> for Spi4 {}

impl super::sig::Signal<super::sig::Spi5Nss> for Spi5 {}
impl super::sig::SignalSpiNss<super::sig::Spi5Nss> for Spi5 {}
impl super::sig::Signal<super::sig::Spi5Miso> for Spi5 {}
impl super::sig::SignalSpiMiso<super::sig::Spi5Miso> for Spi5 {}
impl super::sig::Signal<super::sig::Spi5Mosi> for Spi5 {}
impl super::sig::SignalSpiMosi<super::sig::Spi5Mosi> for Spi5 {}
impl super::sig::Signal<super::sig::Spi5Sck> for Spi5 {}
impl super::sig::SignalSpiSck<super::sig::Spi5Sck> for Spi5 {}

impl super::sig::Signal<super::sig::Spi6Nss> for Spi6 {}
impl super::sig::SignalSpiNss<super::sig::Spi6Nss> for Spi6 {}
impl super::sig::Signal<super::sig::Spi6Miso> for Spi6 {}
impl super::sig::SignalSpiMiso<super::sig::Spi6Miso> for Spi6 {}
impl super::sig::Signal<super::sig::Spi6Mosi> for Spi6 {}
impl super::sig::SignalSpiMosi<super::sig::Spi6Mosi> for Spi6 {}
impl super::sig::Signal<super::sig::Spi6Sck> for Spi6 {}
impl super::sig::SignalSpiSck<super::sig::Spi6Sck> for Spi6 {}



