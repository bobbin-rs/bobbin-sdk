#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::wdog::*;

pub const WDOG: Wdog = Periph(0x40052000, WdogId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct WdogId {}
pub type Wdog = Periph<WdogId>;



pub trait IrqWdog<T> {
   fn irq_wdog(&self) -> super::irq::Irq<T>;
}

pub trait RegisterWdogHandler {
   fn register_wdog_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleWdog>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleWdog {
   fn handle_wdog(&self);
}

impl IrqWdog<super::irq::WdogId> for Wdog {
   fn irq_wdog(&self) -> super::irq::IrqWdog { super::irq::IRQ_WDOG }
}

impl RegisterWdogHandler for Wdog {
   fn register_wdog_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleWdog>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleWdog>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_wdog() }
       }
       super::irq::set_handler(22, Some(wrapper::<F>));
       super::irq::IrqGuard::new(22)
   }
}

