#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::spi_v1::*;

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





pub trait IrqSpi<T> {
    fn irq_spi(&self) -> T;
}

impl IrqSpi<super::irq::IrqSpi1> for Spi1 {
    fn irq_spi(&self) -> super::irq::IrqSpi1 { super::irq::IRQ_SPI1 }
}

impl IrqSpi<super::irq::IrqSpi2> for Spi2 {
    fn irq_spi(&self) -> super::irq::IrqSpi2 { super::irq::IRQ_SPI2 }
}

impl IrqSpi<super::irq::IrqSpi3> for Spi3 {
    fn irq_spi(&self) -> super::irq::IrqSpi3 { super::irq::IRQ_SPI3 }
}

