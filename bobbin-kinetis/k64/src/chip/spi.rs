#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::spi::*;

periph!( SPI0, Spi0, _SPI0, SpiPeriph, 0x4002c000);
periph!( SPI1, Spi1, _SPI1, SpiPeriph, 0x4002d000);
periph!( SPI2, Spi2, _SPI2, SpiPeriph, 0x400ac000);






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

