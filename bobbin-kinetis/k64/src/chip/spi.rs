#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::spi::*;

pub const SPI0: Spi0 = Periph(0x4002c000, Spi0Id {});
pub const SPI1: Spi1 = Periph(0x4002d000, Spi1Id {});
pub const SPI2: Spi2 = Periph(0x400ac000, Spi2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Spi0Id {}
pub type Spi0 = Periph<Spi0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Spi1Id {}
pub type Spi1 = Periph<Spi1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Spi2Id {}
pub type Spi2 = Periph<Spi2Id>;





pub trait IrqSpi<T> {
   fn irq_spi(&self) -> super::irq::Irq<T>;
}

pub trait RegisterSpiHandler {
   fn register_spi_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleSpi>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleSpi {
   fn handle_spi(&self);
}

impl IrqSpi<super::irq::Spi0Id> for Spi0 {
   fn irq_spi(&self) -> super::irq::IrqSpi0 { super::irq::IRQ_SPI0 }
}

impl RegisterSpiHandler for Spi0 {
   fn register_spi_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleSpi>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleSpi>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_spi() }
       }
       super::irq::set_handler(26, Some(wrapper::<F>));
       super::irq::IrqGuard::new(26)
   }
}

impl IrqSpi<super::irq::Spi1Id> for Spi1 {
   fn irq_spi(&self) -> super::irq::IrqSpi1 { super::irq::IRQ_SPI1 }
}

impl RegisterSpiHandler for Spi1 {
   fn register_spi_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleSpi>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleSpi>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_spi() }
       }
       super::irq::set_handler(27, Some(wrapper::<F>));
       super::irq::IrqGuard::new(27)
   }
}

impl IrqSpi<super::irq::Spi2Id> for Spi2 {
   fn irq_spi(&self) -> super::irq::IrqSpi2 { super::irq::IRQ_SPI2 }
}

impl RegisterSpiHandler for Spi2 {
   fn register_spi_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleSpi>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleSpi>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_spi() }
       }
       super::irq::set_handler(65, Some(wrapper::<F>));
       super::irq::IrqGuard::new(65)
   }
}

