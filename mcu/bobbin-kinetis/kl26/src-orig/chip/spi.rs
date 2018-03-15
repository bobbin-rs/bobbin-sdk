#[allow(unused_imports)] use bobbin_common::*;

periph!( SPI0, Spi0, _SPI0, SpiPeriph, 0x40076000);
periph!( SPI1, Spi1, _SPI1, SpiPeriph, 0x40077000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SPI Peripheral"]
pub struct SpiPeriph(pub usize); 

impl super::sig::Signal<super::sig::Spi0Sck> for Spi0 {}
impl super::sig::SignalSpiSck<super::sig::Spi0Sck> for Spi0 {}
impl super::sig::Signal<super::sig::Spi0Miso> for Spi0 {}
impl super::sig::SignalSpiMiso<super::sig::Spi0Miso> for Spi0 {}
impl super::sig::Signal<super::sig::Spi0Mosi> for Spi0 {}
impl super::sig::SignalSpiMosi<super::sig::Spi0Mosi> for Spi0 {}
impl super::sig::Signal<super::sig::Spi0Pcs0> for Spi0 {}
impl super::sig::SignalSpiPcs0<super::sig::Spi0Pcs0> for Spi0 {}

impl super::sig::Signal<super::sig::Spi1Sck> for Spi1 {}
impl super::sig::SignalSpiSck<super::sig::Spi1Sck> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Sout> for Spi1 {}
impl super::sig::SignalSpiSout<super::sig::Spi1Sout> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Sin> for Spi1 {}
impl super::sig::SignalSpiSin<super::sig::Spi1Sin> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Pcs0> for Spi1 {}
impl super::sig::SignalSpiPcs0<super::sig::Spi1Pcs0> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Pcs1> for Spi1 {}
impl super::sig::SignalSpiPcs1<super::sig::Spi1Pcs1> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Pcs2> for Spi1 {}
impl super::sig::SignalSpiPcs2<super::sig::Spi1Pcs2> for Spi1 {}
impl super::sig::Signal<super::sig::Spi1Pcs3> for Spi1 {}
impl super::sig::SignalSpiPcs3<super::sig::Spi1Pcs3> for Spi1 {}



pub trait IrqSpi<T> {
    fn irq_spi(&self) -> T;
}

impl IrqSpi<super::irq::IrqSpi0> for Spi0 {
    fn irq_spi(&self) -> super::irq::IrqSpi0 { super::irq::IRQ_SPI0 }
}

impl IrqSpi<super::irq::IrqSpi1> for Spi1 {
    fn irq_spi(&self) -> super::irq::IrqSpi1 { super::irq::IRQ_SPI1 }
}

