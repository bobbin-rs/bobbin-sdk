pub use stm32_common::chip::tim_adv::*;

pub const TIM1: Tim1 = Periph(0x40010000, Tim1Id {});
pub const TIM8: Tim8 = Periph(0x40010400, Tim8Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim1Id {}
pub type Tim1 = Periph<Tim1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim8Id {}
pub type Tim8 = Periph<Tim8Id>;




pub trait IrqBrk<T> {
   fn irq_brk(&self) -> super::irq::Irq<T>;
}

pub trait RegisterBrkHandler {
   fn register_brk_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleBrk>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleBrk {
   fn handle_brk(&self);
}

pub trait IrqUp<T> {
   fn irq_up(&self) -> super::irq::Irq<T>;
}

pub trait RegisterUpHandler {
   fn register_up_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUp>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleUp {
   fn handle_up(&self);
}

pub trait IrqTrgCom<T> {
   fn irq_trg_com(&self) -> super::irq::Irq<T>;
}

pub trait RegisterTrgComHandler {
   fn register_trg_com_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTrgCom>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleTrgCom {
   fn handle_trg_com(&self);
}

pub trait IrqCc<T> {
   fn irq_cc(&self) -> super::irq::Irq<T>;
}

pub trait RegisterCcHandler {
   fn register_cc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCc>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleCc {
   fn handle_cc(&self);
}

impl IrqBrk<super::irq::Tim1BrkTim9Id> for Tim1 {
   fn irq_brk(&self) -> super::irq::Irq<super::irq::Tim1BrkTim9Id> { super::irq::IRQ_TIM1_BRK_TIM9 }
}

impl RegisterBrkHandler for Tim1 {
   fn register_brk_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleBrk>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleBrk>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_brk() }
       }
       super::irq::set_handler(24, Some(wrapper::<F>));
       super::irq::IrqGuard::new(24)
   }
}

impl IrqUp<super::irq::Tim1UpTim10Id> for Tim1 {
   fn irq_up(&self) -> super::irq::Irq<super::irq::Tim1UpTim10Id> { super::irq::IRQ_TIM1_UP_TIM10 }
}

impl RegisterUpHandler for Tim1 {
   fn register_up_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUp>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUp>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_up() }
       }
       super::irq::set_handler(25, Some(wrapper::<F>));
       super::irq::IrqGuard::new(25)
   }
}

impl IrqTrgCom<super::irq::Tim1TrgComTim11Id> for Tim1 {
   fn irq_trg_com(&self) -> super::irq::Irq<super::irq::Tim1TrgComTim11Id> { super::irq::IRQ_TIM1_TRG_COM_TIM11 }
}

impl RegisterTrgComHandler for Tim1 {
   fn register_trg_com_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTrgCom>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTrgCom>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_trg_com() }
       }
       super::irq::set_handler(26, Some(wrapper::<F>));
       super::irq::IrqGuard::new(26)
   }
}

impl IrqCc<super::irq::Tim1CcId> for Tim1 {
   fn irq_cc(&self) -> super::irq::Irq<super::irq::Tim1CcId> { super::irq::IRQ_TIM1_CC }
}

impl RegisterCcHandler for Tim1 {
   fn register_cc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCc>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCc>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_cc() }
       }
       super::irq::set_handler(27, Some(wrapper::<F>));
       super::irq::IrqGuard::new(27)
   }
}

impl IrqBrk<super::irq::Tim8BrkTim12Id> for Tim8 {
   fn irq_brk(&self) -> super::irq::Irq<super::irq::Tim8BrkTim12Id> { super::irq::IRQ_TIM8_BRK_TIM12 }
}

impl RegisterBrkHandler for Tim8 {
   fn register_brk_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleBrk>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleBrk>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_brk() }
       }
       super::irq::set_handler(43, Some(wrapper::<F>));
       super::irq::IrqGuard::new(43)
   }
}

impl IrqUp<super::irq::Tim8UpTim13Id> for Tim8 {
   fn irq_up(&self) -> super::irq::Irq<super::irq::Tim8UpTim13Id> { super::irq::IRQ_TIM8_UP_TIM13 }
}

impl RegisterUpHandler for Tim8 {
   fn register_up_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUp>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUp>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_up() }
       }
       super::irq::set_handler(44, Some(wrapper::<F>));
       super::irq::IrqGuard::new(44)
   }
}

impl IrqTrgCom<super::irq::Tim8TrgComTim14Id> for Tim8 {
   fn irq_trg_com(&self) -> super::irq::Irq<super::irq::Tim8TrgComTim14Id> { super::irq::IRQ_TIM8_TRG_COM_TIM14 }
}

impl RegisterTrgComHandler for Tim8 {
   fn register_trg_com_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTrgCom>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTrgCom>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_trg_com() }
       }
       super::irq::set_handler(45, Some(wrapper::<F>));
       super::irq::IrqGuard::new(45)
   }
}

impl IrqCc<super::irq::Tim8CcId> for Tim8 {
   fn irq_cc(&self) -> super::irq::Irq<super::irq::Tim8CcId> { super::irq::IRQ_TIM8_CC }
}

impl RegisterCcHandler for Tim8 {
   fn register_cc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCc>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCc>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_cc() }
       }
       super::irq::set_handler(46, Some(wrapper::<F>));
       super::irq::IrqGuard::new(46)
   }
}

