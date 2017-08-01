pub use stm32_common::chip::tim_bas::*;

pub const TIM6: Tim6 = Periph(0x40001000, Tim6Id {});
pub const TIM7: Tim7 = Periph(0x40001400, Tim7Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tim6Id {}
pub type Tim6 = Periph<Tim6Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Tim7Id {}
pub type Tim7 = Periph<Tim7Id>;




pub trait IrqTim<T> {
   fn irq_tim(&self) -> super::irq::Irq<T>;
}

pub trait RegisterTimHandler {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleTim {
   fn handle_tim(&self);
}

impl IrqTim<super::irq::Tim6Id> for Tim6 {
   fn irq_tim(&self) -> super::irq::IrqTim6 { super::irq::IRQ_TIM6 }
}

impl RegisterTimHandler for Tim6 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(54, Some(wrapper::<F>));
       super::irq::IrqGuard::new(54)
   }
}

impl IrqTim<super::irq::Tim7Id> for Tim7 {
   fn irq_tim(&self) -> super::irq::IrqTim7 { super::irq::IRQ_TIM7 }
}

impl RegisterTimHandler for Tim7 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(55, Some(wrapper::<F>));
       super::irq::IrqGuard::new(55)
   }
}

