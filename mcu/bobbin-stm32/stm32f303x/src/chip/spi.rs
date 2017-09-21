#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::spi_v2::*;

periph!( SPI1, Spi1, _SPI1, SpiPeriph, 0x40013000);
periph!( SPI2, Spi2, _SPI2, SpiPeriph, 0x40003800);
periph!( SPI3, Spi3, _SPI3, SpiPeriph, 0x40003c00);

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



