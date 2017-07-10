pub use stm32_common::chip::lptim::*;

pub const LPTIM: Lptim = Periph(0x40007c00, LptimId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LptimId {}
pub type Lptim = Periph<LptimId>;



pub trait IrqLptim<T> {
   fn irq_lptim(&self) -> super::irq::Irq<T>;
}

pub trait RegisterLptimHandler {
   fn register_lptim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLptim>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleLptim {
   fn handle_lptim(&self);
}

impl IrqLptim<super::irq::Lptim1Id> for Lptim {
   fn irq_lptim(&self) -> super::irq::IrqLptim1 { super::irq::IRQ_LPTIM1 }
}

impl RegisterLptimHandler for Lptim {
   fn register_lptim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLptim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLptim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lptim() }
       }
       super::irq::set_handler(13, Some(wrapper::<F>));
       super::irq::IrqGuard::new(13)
   }
}

