pub use stm32_common::chip::tim_gen::*;

pub const TIM2: Tim2 = Periph(0x40000000, Tim2Id {});
pub const TIM3: Tim3 = Periph(0x40000400, Tim3Id {});
pub const TIM4: Tim4 = Periph(0x40000800, Tim4Id {});
pub const TIM5: Tim5 = Periph(0x40000c00, Tim5Id {});
pub const TIM9: Tim9 = Periph(0x40014000, Tim9Id {});
pub const TIM10: Tim10 = Periph(0x40014400, Tim10Id {});
pub const TIM11: Tim11 = Periph(0x40014800, Tim11Id {});
pub const TIM12: Tim12 = Periph(0x40001800, Tim12Id {});
pub const TIM13: Tim13 = Periph(0x40001c00, Tim13Id {});
pub const TIM14: Tim14 = Periph(0x40002000, Tim14Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim2Id {}
pub type Tim2 = Periph<Tim2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim3Id {}
pub type Tim3 = Periph<Tim3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim4Id {}
pub type Tim4 = Periph<Tim4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim5Id {}
pub type Tim5 = Periph<Tim5Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim9Id {}
pub type Tim9 = Periph<Tim9Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim10Id {}
pub type Tim10 = Periph<Tim10Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim11Id {}
pub type Tim11 = Periph<Tim11Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim12Id {}
pub type Tim12 = Periph<Tim12Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim13Id {}
pub type Tim13 = Periph<Tim13Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim14Id {}
pub type Tim14 = Periph<Tim14Id>;












pub trait IrqTim<T> {
   fn irq_tim(&self) -> super::irq::Irq<T>;
}

pub trait RegisterTimHandler {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleTim {
   fn handle_tim(&self);
}

impl IrqTim<super::irq::Tim2Id> for Tim2 {
   fn irq_tim(&self) -> super::irq::Irq<super::irq::Tim2Id> { super::irq::IRQ_TIM2 }
}

impl RegisterTimHandler for Tim2 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(28, Some(wrapper::<F>));
       super::irq::IrqGuard::new(28)
   }
}

impl IrqTim<super::irq::Tim3Id> for Tim3 {
   fn irq_tim(&self) -> super::irq::Irq<super::irq::Tim3Id> { super::irq::IRQ_TIM3 }
}

impl RegisterTimHandler for Tim3 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(29, Some(wrapper::<F>));
       super::irq::IrqGuard::new(29)
   }
}

impl IrqTim<super::irq::Tim4Id> for Tim4 {
   fn irq_tim(&self) -> super::irq::Irq<super::irq::Tim4Id> { super::irq::IRQ_TIM4 }
}

impl RegisterTimHandler for Tim4 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(30, Some(wrapper::<F>));
       super::irq::IrqGuard::new(30)
   }
}

impl IrqTim<super::irq::Tim5Id> for Tim5 {
   fn irq_tim(&self) -> super::irq::Irq<super::irq::Tim5Id> { super::irq::IRQ_TIM5 }
}

impl RegisterTimHandler for Tim5 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(50, Some(wrapper::<F>));
       super::irq::IrqGuard::new(50)
   }
}

impl IrqTim<super::irq::Tim1BrkTim9Id> for Tim9 {
   fn irq_tim(&self) -> super::irq::Irq<super::irq::Tim1BrkTim9Id> { super::irq::IRQ_TIM1_BRK_TIM9 }
}

impl RegisterTimHandler for Tim9 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(24, Some(wrapper::<F>));
       super::irq::IrqGuard::new(24)
   }
}

impl IrqTim<super::irq::Tim1UpTim10Id> for Tim10 {
   fn irq_tim(&self) -> super::irq::Irq<super::irq::Tim1UpTim10Id> { super::irq::IRQ_TIM1_UP_TIM10 }
}

impl RegisterTimHandler for Tim10 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(25, Some(wrapper::<F>));
       super::irq::IrqGuard::new(25)
   }
}

impl IrqTim<super::irq::Tim1TrgComTim11Id> for Tim11 {
   fn irq_tim(&self) -> super::irq::Irq<super::irq::Tim1TrgComTim11Id> { super::irq::IRQ_TIM1_TRG_COM_TIM11 }
}

impl RegisterTimHandler for Tim11 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(26, Some(wrapper::<F>));
       super::irq::IrqGuard::new(26)
   }
}

impl IrqTim<super::irq::Tim8BrkTim12Id> for Tim12 {
   fn irq_tim(&self) -> super::irq::Irq<super::irq::Tim8BrkTim12Id> { super::irq::IRQ_TIM8_BRK_TIM12 }
}

impl RegisterTimHandler for Tim12 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(43, Some(wrapper::<F>));
       super::irq::IrqGuard::new(43)
   }
}

impl IrqTim<super::irq::Tim8UpTim13Id> for Tim13 {
   fn irq_tim(&self) -> super::irq::Irq<super::irq::Tim8UpTim13Id> { super::irq::IRQ_TIM8_UP_TIM13 }
}

impl RegisterTimHandler for Tim13 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(44, Some(wrapper::<F>));
       super::irq::IrqGuard::new(44)
   }
}

impl IrqTim<super::irq::Tim8TrgComTim14Id> for Tim14 {
   fn irq_tim(&self) -> super::irq::Irq<super::irq::Tim8TrgComTim14Id> { super::irq::IRQ_TIM8_TRG_COM_TIM14 }
}

impl RegisterTimHandler for Tim14 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(45, Some(wrapper::<F>));
       super::irq::IrqGuard::new(45)
   }
}

