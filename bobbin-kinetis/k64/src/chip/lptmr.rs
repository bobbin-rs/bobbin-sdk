#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::lptmr::*;

pub const LPTMR0: Lptmr0 = Periph(0x40040000, Lptmr0Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lptmr0Id {}
pub type Lptmr0 = Periph<Lptmr0Id>;



pub trait IrqLptmr<T> {
   fn irq_lptmr(&self) -> super::irq::Irq<T>;
}

pub trait RegisterLptmrHandler {
   fn register_lptmr_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLptmr>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleLptmr {
   fn handle_lptmr(&self);
}

impl IrqLptmr<super::irq::Lptmr0Id> for Lptmr0 {
   fn irq_lptmr(&self) -> super::irq::IrqLptmr0 { super::irq::IRQ_LPTMR0 }
}

impl RegisterLptmrHandler for Lptmr0 {
   fn register_lptmr_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLptmr>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLptmr>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lptmr() }
       }
       super::irq::set_handler(58, Some(wrapper::<F>));
       super::irq::IrqGuard::new(58)
   }
}

