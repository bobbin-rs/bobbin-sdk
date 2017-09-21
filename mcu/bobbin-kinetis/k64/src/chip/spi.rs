#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::spi::*;

periph!( SPI0, Spi0, _SPI0, SpiPeriph, 0x4002c000);
periph!( SPI1, Spi1, _SPI1, SpiPeriph, 0x4002d000);
periph!( SPI2, Spi2, _SPI2, SpiPeriph, 0x400ac000);

impl super::sig::Signal<super::sig::Spi0Sck> for Spi0 {}
impl super::sig::SignalSpiSck<super::sig::Spi0Sck> for Spi0 {}
impl super::sig::Signal<super::sig::Spi0Sout> for Spi0 {}
impl super::sig::SignalSpiSout<super::sig::Spi0Sout> for Spi0 {}
impl super::sig::Signal<super::sig::Spi0Sin> for Spi0 {}
impl super::sig::SignalSpiSin<super::sig::Spi0Sin> for Spi0 {}
impl super::sig::Signal<super::sig::Spi0Pcs0> for Spi0 {}
impl super::sig::SignalSpiPcs0<super::sig::Spi0Pcs0> for Spi0 {}
impl super::sig::Signal<super::sig::Spi0Pcs1> for Spi0 {}
impl super::sig::SignalSpiPcs1<super::sig::Spi0Pcs1> for Spi0 {}
impl super::sig::Signal<super::sig::Spi0Pcs2> for Spi0 {}
impl super::sig::SignalSpiPcs2<super::sig::Spi0Pcs2> for Spi0 {}
impl super::sig::Signal<super::sig::Spi0Pcs3> for Spi0 {}
impl super::sig::SignalSpiPcs3<super::sig::Spi0Pcs3> for Spi0 {}
impl super::sig::Signal<super::sig::Spi0Pcs4> for Spi0 {}
impl super::sig::SignalSpiPcs4<super::sig::Spi0Pcs4> for Spi0 {}
impl super::sig::Signal<super::sig::Spi0Pcs5> for Spi0 {}
impl super::sig::SignalSpiPcs5<super::sig::Spi0Pcs5> for Spi0 {}

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

impl super::sig::Signal<super::sig::Spi2Sck> for Spi2 {}
impl super::sig::SignalSpiSck<super::sig::Spi2Sck> for Spi2 {}
impl super::sig::Signal<super::sig::Spi2Sout> for Spi2 {}
impl super::sig::SignalSpiSout<super::sig::Spi2Sout> for Spi2 {}
impl super::sig::Signal<super::sig::Spi2Sin> for Spi2 {}
impl super::sig::SignalSpiSin<super::sig::Spi2Sin> for Spi2 {}
impl super::sig::Signal<super::sig::Spi2Pcs0> for Spi2 {}
impl super::sig::SignalSpiPcs0<super::sig::Spi2Pcs0> for Spi2 {}
impl super::sig::Signal<super::sig::Spi2Pcs1> for Spi2 {}
impl super::sig::SignalSpiPcs1<super::sig::Spi2Pcs1> for Spi2 {}



pub trait IrqSpi<T> {
    fn irq_spi(&self) -> T;
}

impl IrqSpi<super::irq::IrqSpi0> for Spi0 {
    fn irq_spi(&self) -> super::irq::IrqSpi0 { super::irq::IRQ_SPI0 }
}

impl IrqSpi<super::irq::IrqSpi1> for Spi1 {
    fn irq_spi(&self) -> super::irq::IrqSpi1 { super::irq::IRQ_SPI1 }
}

impl IrqSpi<super::irq::IrqSpi2> for Spi2 {
    fn irq_spi(&self) -> super::irq::IrqSpi2 { super::irq::IRQ_SPI2 }
}

