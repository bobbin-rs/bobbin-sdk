#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::wwdg::*;

pub const WWDG: Wwdg = Periph(0x40002c00, WwdgId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct WwdgId {}
pub type Wwdg = Periph<WwdgId>;



pub trait IrqWwdg<T> {
   fn irq_wwdg(&self) -> super::irq::Irq<T>;
}

pub trait RegisterWwdgHandler {
   fn register_wwdg_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleWwdg>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleWwdg {
   fn handle_wwdg(&self);
}

impl IrqWwdg<super::irq::WwdgId> for Wwdg {
   fn irq_wwdg(&self) -> super::irq::IrqWwdg { super::irq::IRQ_WWDG }
}

impl RegisterWwdgHandler for Wwdg {
   fn register_wwdg_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleWwdg>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleWwdg>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_wwdg() }
       }
       super::irq::set_handler(0, Some(wrapper::<F>));
       super::irq::IrqGuard::new(0)
   }
}

