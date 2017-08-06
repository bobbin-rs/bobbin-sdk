#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::exti::*;

pub const EXTI: Exti = Periph(0x40010400, ExtiId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct ExtiId {}
pub type Exti = Periph<ExtiId>;



pub const EXTI_LINE0: Channel<ExtiLine0Id, ExtiId> = Channel { periph: EXTI, index: 0, id: ExtiLine0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine0Id {}
pub type ExtiLine0 = Channel<ExtiLine0Id, ExtiId>;

pub const EXTI_LINE1: Channel<ExtiLine1Id, ExtiId> = Channel { periph: EXTI, index: 1, id: ExtiLine1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine1Id {}
pub type ExtiLine1 = Channel<ExtiLine1Id, ExtiId>;

pub const EXTI_LINE2: Channel<ExtiLine2Id, ExtiId> = Channel { periph: EXTI, index: 2, id: ExtiLine2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine2Id {}
pub type ExtiLine2 = Channel<ExtiLine2Id, ExtiId>;

pub const EXTI_LINE3: Channel<ExtiLine3Id, ExtiId> = Channel { periph: EXTI, index: 3, id: ExtiLine3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine3Id {}
pub type ExtiLine3 = Channel<ExtiLine3Id, ExtiId>;

pub const EXTI_LINE4: Channel<ExtiLine4Id, ExtiId> = Channel { periph: EXTI, index: 4, id: ExtiLine4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine4Id {}
pub type ExtiLine4 = Channel<ExtiLine4Id, ExtiId>;

pub const EXTI_LINE5: Channel<ExtiLine5Id, ExtiId> = Channel { periph: EXTI, index: 5, id: ExtiLine5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine5Id {}
pub type ExtiLine5 = Channel<ExtiLine5Id, ExtiId>;

pub const EXTI_LINE6: Channel<ExtiLine6Id, ExtiId> = Channel { periph: EXTI, index: 6, id: ExtiLine6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine6Id {}
pub type ExtiLine6 = Channel<ExtiLine6Id, ExtiId>;

pub const EXTI_LINE7: Channel<ExtiLine7Id, ExtiId> = Channel { periph: EXTI, index: 7, id: ExtiLine7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine7Id {}
pub type ExtiLine7 = Channel<ExtiLine7Id, ExtiId>;

pub const EXTI_LINE8: Channel<ExtiLine8Id, ExtiId> = Channel { periph: EXTI, index: 8, id: ExtiLine8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine8Id {}
pub type ExtiLine8 = Channel<ExtiLine8Id, ExtiId>;

pub const EXTI_LINE9: Channel<ExtiLine9Id, ExtiId> = Channel { periph: EXTI, index: 9, id: ExtiLine9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine9Id {}
pub type ExtiLine9 = Channel<ExtiLine9Id, ExtiId>;

pub const EXTI_LINE10: Channel<ExtiLine10Id, ExtiId> = Channel { periph: EXTI, index: 10, id: ExtiLine10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine10Id {}
pub type ExtiLine10 = Channel<ExtiLine10Id, ExtiId>;

pub const EXTI_LINE11: Channel<ExtiLine11Id, ExtiId> = Channel { periph: EXTI, index: 11, id: ExtiLine11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine11Id {}
pub type ExtiLine11 = Channel<ExtiLine11Id, ExtiId>;

pub const EXTI_LINE12: Channel<ExtiLine12Id, ExtiId> = Channel { periph: EXTI, index: 12, id: ExtiLine12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine12Id {}
pub type ExtiLine12 = Channel<ExtiLine12Id, ExtiId>;

pub const EXTI_LINE13: Channel<ExtiLine13Id, ExtiId> = Channel { periph: EXTI, index: 13, id: ExtiLine13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine13Id {}
pub type ExtiLine13 = Channel<ExtiLine13Id, ExtiId>;

pub const EXTI_LINE14: Channel<ExtiLine14Id, ExtiId> = Channel { periph: EXTI, index: 14, id: ExtiLine14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine14Id {}
pub type ExtiLine14 = Channel<ExtiLine14Id, ExtiId>;

pub const EXTI_LINE15: Channel<ExtiLine15Id, ExtiId> = Channel { periph: EXTI, index: 15, id: ExtiLine15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine15Id {}
pub type ExtiLine15 = Channel<ExtiLine15Id, ExtiId>;

pub const EXTI_LINE16: Channel<ExtiLine16Id, ExtiId> = Channel { periph: EXTI, index: 16, id: ExtiLine16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine16Id {}
pub type ExtiLine16 = Channel<ExtiLine16Id, ExtiId>;

pub const EXTI_LINE17: Channel<ExtiLine17Id, ExtiId> = Channel { periph: EXTI, index: 17, id: ExtiLine17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine17Id {}
pub type ExtiLine17 = Channel<ExtiLine17Id, ExtiId>;

pub const EXTI_LINE18: Channel<ExtiLine18Id, ExtiId> = Channel { periph: EXTI, index: 18, id: ExtiLine18Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine18Id {}
pub type ExtiLine18 = Channel<ExtiLine18Id, ExtiId>;

pub const EXTI_LINE19: Channel<ExtiLine19Id, ExtiId> = Channel { periph: EXTI, index: 19, id: ExtiLine19Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine19Id {}
pub type ExtiLine19 = Channel<ExtiLine19Id, ExtiId>;

pub const EXTI_LINE20: Channel<ExtiLine20Id, ExtiId> = Channel { periph: EXTI, index: 20, id: ExtiLine20Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine20Id {}
pub type ExtiLine20 = Channel<ExtiLine20Id, ExtiId>;

pub const EXTI_LINE21: Channel<ExtiLine21Id, ExtiId> = Channel { periph: EXTI, index: 21, id: ExtiLine21Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine21Id {}
pub type ExtiLine21 = Channel<ExtiLine21Id, ExtiId>;

pub const EXTI_LINE22: Channel<ExtiLine22Id, ExtiId> = Channel { periph: EXTI, index: 22, id: ExtiLine22Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine22Id {}
pub type ExtiLine22 = Channel<ExtiLine22Id, ExtiId>;

pub const EXTI_LINE23: Channel<ExtiLine23Id, ExtiId> = Channel { periph: EXTI, index: 23, id: ExtiLine23Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine23Id {}
pub type ExtiLine23 = Channel<ExtiLine23Id, ExtiId>;

pub const EXTI_LINE24: Channel<ExtiLine24Id, ExtiId> = Channel { periph: EXTI, index: 24, id: ExtiLine24Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine24Id {}
pub type ExtiLine24 = Channel<ExtiLine24Id, ExtiId>;

pub const EXTI_LINE25: Channel<ExtiLine25Id, ExtiId> = Channel { periph: EXTI, index: 25, id: ExtiLine25Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine25Id {}
pub type ExtiLine25 = Channel<ExtiLine25Id, ExtiId>;

pub const EXTI_LINE26: Channel<ExtiLine26Id, ExtiId> = Channel { periph: EXTI, index: 26, id: ExtiLine26Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine26Id {}
pub type ExtiLine26 = Channel<ExtiLine26Id, ExtiId>;

pub const EXTI_LINE28: Channel<ExtiLine28Id, ExtiId> = Channel { periph: EXTI, index: 28, id: ExtiLine28Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine28Id {}
pub type ExtiLine28 = Channel<ExtiLine28Id, ExtiId>;

pub const EXTI_LINE29: Channel<ExtiLine29Id, ExtiId> = Channel { periph: EXTI, index: 29, id: ExtiLine29Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct ExtiLine29Id {}
pub type ExtiLine29 = Channel<ExtiLine29Id, ExtiId>;

pub trait IrqExti<T> {
   fn irq_exti(&self) -> super::irq::Irq<T>;
}

pub trait RegisterExtiHandler {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleExti {
   fn handle_exti(&self);
}

impl IrqExti<super::irq::Exti0Id> for ExtiLine0 {
   fn irq_exti(&self) -> super::irq::IrqExti0 { super::irq::IRQ_EXTI0 }
}

impl RegisterExtiHandler for ExtiLine0 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(5, Some(wrapper::<F>));
       super::irq::IrqGuard::new(5)
   }
}

impl IrqExti<super::irq::Exti1Id> for ExtiLine1 {
   fn irq_exti(&self) -> super::irq::IrqExti1 { super::irq::IRQ_EXTI1 }
}

impl RegisterExtiHandler for ExtiLine1 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(5, Some(wrapper::<F>));
       super::irq::IrqGuard::new(5)
   }
}

impl IrqExti<super::irq::Exti2Id> for ExtiLine2 {
   fn irq_exti(&self) -> super::irq::IrqExti2 { super::irq::IRQ_EXTI2 }
}

impl RegisterExtiHandler for ExtiLine2 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(6, Some(wrapper::<F>));
       super::irq::IrqGuard::new(6)
   }
}

impl IrqExti<super::irq::Exti3Id> for ExtiLine3 {
   fn irq_exti(&self) -> super::irq::IrqExti3 { super::irq::IRQ_EXTI3 }
}

impl RegisterExtiHandler for ExtiLine3 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(6, Some(wrapper::<F>));
       super::irq::IrqGuard::new(6)
   }
}

impl IrqExti<super::irq::Exti4Id> for ExtiLine4 {
   fn irq_exti(&self) -> super::irq::IrqExti4 { super::irq::IRQ_EXTI4 }
}

impl RegisterExtiHandler for ExtiLine4 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti5Id> for ExtiLine5 {
   fn irq_exti(&self) -> super::irq::IrqExti5 { super::irq::IRQ_EXTI5 }
}

impl RegisterExtiHandler for ExtiLine5 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti6Id> for ExtiLine6 {
   fn irq_exti(&self) -> super::irq::IrqExti6 { super::irq::IRQ_EXTI6 }
}

impl RegisterExtiHandler for ExtiLine6 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti7Id> for ExtiLine7 {
   fn irq_exti(&self) -> super::irq::IrqExti7 { super::irq::IRQ_EXTI7 }
}

impl RegisterExtiHandler for ExtiLine7 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti8Id> for ExtiLine8 {
   fn irq_exti(&self) -> super::irq::IrqExti8 { super::irq::IRQ_EXTI8 }
}

impl RegisterExtiHandler for ExtiLine8 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti9Id> for ExtiLine9 {
   fn irq_exti(&self) -> super::irq::IrqExti9 { super::irq::IRQ_EXTI9 }
}

impl RegisterExtiHandler for ExtiLine9 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti10Id> for ExtiLine10 {
   fn irq_exti(&self) -> super::irq::IrqExti10 { super::irq::IRQ_EXTI10 }
}

impl RegisterExtiHandler for ExtiLine10 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti11Id> for ExtiLine11 {
   fn irq_exti(&self) -> super::irq::IrqExti11 { super::irq::IRQ_EXTI11 }
}

impl RegisterExtiHandler for ExtiLine11 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti12Id> for ExtiLine12 {
   fn irq_exti(&self) -> super::irq::IrqExti12 { super::irq::IRQ_EXTI12 }
}

impl RegisterExtiHandler for ExtiLine12 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti13Id> for ExtiLine13 {
   fn irq_exti(&self) -> super::irq::IrqExti13 { super::irq::IRQ_EXTI13 }
}

impl RegisterExtiHandler for ExtiLine13 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti14Id> for ExtiLine14 {
   fn irq_exti(&self) -> super::irq::IrqExti14 { super::irq::IRQ_EXTI14 }
}

impl RegisterExtiHandler for ExtiLine14 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti15Id> for ExtiLine15 {
   fn irq_exti(&self) -> super::irq::IrqExti15 { super::irq::IRQ_EXTI15 }
}

impl RegisterExtiHandler for ExtiLine15 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqExti<super::irq::Exti16Id> for ExtiLine16 {
   fn irq_exti(&self) -> super::irq::IrqExti16 { super::irq::IRQ_EXTI16 }
}

impl RegisterExtiHandler for ExtiLine16 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(1, Some(wrapper::<F>));
       super::irq::IrqGuard::new(1)
   }
}

impl IrqExti<super::irq::Exti17Id> for ExtiLine17 {
   fn irq_exti(&self) -> super::irq::IrqExti17 { super::irq::IRQ_EXTI17 }
}

impl RegisterExtiHandler for ExtiLine17 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(2, Some(wrapper::<F>));
       super::irq::IrqGuard::new(2)
   }
}

impl IrqExti<super::irq::Exti19Id> for ExtiLine19 {
   fn irq_exti(&self) -> super::irq::IrqExti19 { super::irq::IRQ_EXTI19 }
}

impl RegisterExtiHandler for ExtiLine19 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(2, Some(wrapper::<F>));
       super::irq::IrqGuard::new(2)
   }
}

impl IrqExti<super::irq::Exti20Id> for ExtiLine20 {
   fn irq_exti(&self) -> super::irq::IrqExti20 { super::irq::IRQ_EXTI20 }
}

impl RegisterExtiHandler for ExtiLine20 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(2, Some(wrapper::<F>));
       super::irq::IrqGuard::new(2)
   }
}

impl IrqExti<super::irq::Exti21Id> for ExtiLine21 {
   fn irq_exti(&self) -> super::irq::IrqExti21 { super::irq::IRQ_EXTI21 }
}

impl RegisterExtiHandler for ExtiLine21 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(12, Some(wrapper::<F>));
       super::irq::IrqGuard::new(12)
   }
}

impl IrqExti<super::irq::Exti22Id> for ExtiLine22 {
   fn irq_exti(&self) -> super::irq::IrqExti22 { super::irq::IRQ_EXTI22 }
}

impl RegisterExtiHandler for ExtiLine22 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(12, Some(wrapper::<F>));
       super::irq::IrqGuard::new(12)
   }
}

impl IrqExti<super::irq::Exti23Id> for ExtiLine23 {
   fn irq_exti(&self) -> super::irq::IrqExti23 { super::irq::IRQ_EXTI23 }
}

impl RegisterExtiHandler for ExtiLine23 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(23, Some(wrapper::<F>));
       super::irq::IrqGuard::new(23)
   }
}

impl IrqExti<super::irq::Exti24Id> for ExtiLine24 {
   fn irq_exti(&self) -> super::irq::IrqExti24 { super::irq::IRQ_EXTI24 }
}

impl RegisterExtiHandler for ExtiLine24 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(24, Some(wrapper::<F>));
       super::irq::IrqGuard::new(24)
   }
}

impl IrqExti<super::irq::Exti25Id> for ExtiLine25 {
   fn irq_exti(&self) -> super::irq::IrqExti25 { super::irq::IRQ_EXTI25 }
}

impl RegisterExtiHandler for ExtiLine25 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(27, Some(wrapper::<F>));
       super::irq::IrqGuard::new(27)
   }
}

impl IrqExti<super::irq::Exti26Id> for ExtiLine26 {
   fn irq_exti(&self) -> super::irq::IrqExti26 { super::irq::IRQ_EXTI26 }
}

impl RegisterExtiHandler for ExtiLine26 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(28, Some(wrapper::<F>));
       super::irq::IrqGuard::new(28)
   }
}

impl IrqExti<super::irq::Exti28Id> for ExtiLine28 {
   fn irq_exti(&self) -> super::irq::IrqExti28 { super::irq::IRQ_EXTI28 }
}

impl RegisterExtiHandler for ExtiLine28 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(29, Some(wrapper::<F>));
       super::irq::IrqGuard::new(29)
   }
}

impl IrqExti<super::irq::Exti29Id> for ExtiLine29 {
   fn irq_exti(&self) -> super::irq::IrqExti29 { super::irq::IRQ_EXTI29 }
}

impl RegisterExtiHandler for ExtiLine29 {
   fn register_exti_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleExti>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleExti>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_exti() }
       }
       super::irq::set_handler(13, Some(wrapper::<F>));
       super::irq::IrqGuard::new(13)
   }
}

