#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::lpspi::*;

pub const LPSPI0: Lpspi0 = Periph(0x4002c000, Lpspi0Id {});
pub const LPSPI1: Lpspi1 = Periph(0x4002d000, Lpspi1Id {});
pub const LPSPI2: Lpspi2 = Periph(0x4002e000, Lpspi2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpspi0Id {}
pub type Lpspi0 = Periph<Lpspi0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpspi1Id {}
pub type Lpspi1 = Periph<Lpspi1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpspi2Id {}
pub type Lpspi2 = Periph<Lpspi2Id>;

impl super::sig::Signal<super::sig::Lpspi0Sck> for Lpspi0 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi0Sck> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Sout> for Lpspi0 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi0Sout> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Sin> for Lpspi0 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi0Sin> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs0> for Lpspi0 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi0Pcs0> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs1> for Lpspi0 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi0Pcs1> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs2> for Lpspi0 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi0Pcs2> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs3> for Lpspi0 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi0Pcs3> for Lpspi0 {}

impl super::sig::Signal<super::sig::Lpspi1Sck> for Lpspi1 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi1Sck> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Sout> for Lpspi1 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi1Sout> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Sin> for Lpspi1 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi1Sin> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs0> for Lpspi1 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi1Pcs0> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs1> for Lpspi1 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi1Pcs1> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs2> for Lpspi1 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi1Pcs2> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs3> for Lpspi1 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi1Pcs3> for Lpspi1 {}

impl super::sig::Signal<super::sig::Lpspi2Sck> for Lpspi2 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi2Sck> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Sout> for Lpspi2 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi2Sout> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Sin> for Lpspi2 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi2Sin> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs0> for Lpspi2 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi2Pcs0> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs1> for Lpspi2 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi2Pcs1> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs2> for Lpspi2 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi2Pcs2> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs3> for Lpspi2 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi2Pcs3> for Lpspi2 {}


pub trait IrqLpspi<T> {
   fn irq_lpspi(&self) -> super::irq::Irq<T>;
}

pub trait RegisterLpspiHandler {
   fn register_lpspi_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpspi>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleLpspi {
   fn handle_lpspi(&self);
}

impl IrqLpspi<super::irq::Lpspi0Id> for Lpspi0 {
   fn irq_lpspi(&self) -> super::irq::IrqLpspi0 { super::irq::IRQ_LPSPI0 }
}

impl RegisterLpspiHandler for Lpspi0 {
   fn register_lpspi_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpspi>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLpspi>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lpspi() }
       }
       super::irq::set_handler(26, Some(wrapper::<F>));
       super::irq::IrqGuard::new(26)
   }
}

impl IrqLpspi<super::irq::Lpspi1Id> for Lpspi1 {
   fn irq_lpspi(&self) -> super::irq::IrqLpspi1 { super::irq::IRQ_LPSPI1 }
}

impl RegisterLpspiHandler for Lpspi1 {
   fn register_lpspi_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpspi>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLpspi>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lpspi() }
       }
       super::irq::set_handler(27, Some(wrapper::<F>));
       super::irq::IrqGuard::new(27)
   }
}

impl IrqLpspi<super::irq::Lpspi2Id> for Lpspi2 {
   fn irq_lpspi(&self) -> super::irq::IrqLpspi2 { super::irq::IRQ_LPSPI2 }
}

impl RegisterLpspiHandler for Lpspi2 {
   fn register_lpspi_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpspi>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLpspi>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lpspi() }
       }
       super::irq::set_handler(28, Some(wrapper::<F>));
       super::irq::IrqGuard::new(28)
   }
}

