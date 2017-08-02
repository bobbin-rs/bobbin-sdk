pub use kinetis_common::chip::lpit::*;

pub const LPIT0: Lpit0 = Periph(0x40037000, Lpit0Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpit0Id {}
pub type Lpit0 = Periph<Lpit0Id>;



pub const LPIT0_CH0: Channel<Lpit0Ch0Id, Lpit0Id> = Channel { periph: LPIT0, index: 0, id: Lpit0Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Lpit0Ch0Id {}
pub type Lpit0Ch0 = Channel<Lpit0Ch0Id, Lpit0Id>;

pub const LPIT0_CH1: Channel<Lpit0Ch1Id, Lpit0Id> = Channel { periph: LPIT0, index: 1, id: Lpit0Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Lpit0Ch1Id {}
pub type Lpit0Ch1 = Channel<Lpit0Ch1Id, Lpit0Id>;

pub const LPIT0_CH2: Channel<Lpit0Ch2Id, Lpit0Id> = Channel { periph: LPIT0, index: 2, id: Lpit0Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Lpit0Ch2Id {}
pub type Lpit0Ch2 = Channel<Lpit0Ch2Id, Lpit0Id>;

pub const LPIT0_CH3: Channel<Lpit0Ch3Id, Lpit0Id> = Channel { periph: LPIT0, index: 3, id: Lpit0Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Lpit0Ch3Id {}
pub type Lpit0Ch3 = Channel<Lpit0Ch3Id, Lpit0Id>;

pub trait IrqLpit<T> {
   fn irq_lpit(&self) -> super::irq::Irq<T>;
}

pub trait RegisterLpitHandler {
   fn register_lpit_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpit>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleLpit {
   fn handle_lpit(&self);
}

impl IrqLpit<super::irq::Lpit0Ch0Id> for Lpit0Ch0 {
   fn irq_lpit(&self) -> super::irq::IrqLpit0Ch0 { super::irq::IRQ_LPIT0_CH0 }
}

impl RegisterLpitHandler for Lpit0Ch0 {
   fn register_lpit_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpit>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLpit>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lpit() }
       }
       super::irq::set_handler(48, Some(wrapper::<F>));
       super::irq::IrqGuard::new(48)
   }
}

impl IrqLpit<super::irq::Lpit0Ch1Id> for Lpit0Ch1 {
   fn irq_lpit(&self) -> super::irq::IrqLpit0Ch1 { super::irq::IRQ_LPIT0_CH1 }
}

impl RegisterLpitHandler for Lpit0Ch1 {
   fn register_lpit_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpit>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLpit>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lpit() }
       }
       super::irq::set_handler(49, Some(wrapper::<F>));
       super::irq::IrqGuard::new(49)
   }
}

impl IrqLpit<super::irq::Lpit0Ch2Id> for Lpit0Ch2 {
   fn irq_lpit(&self) -> super::irq::IrqLpit0Ch2 { super::irq::IRQ_LPIT0_CH2 }
}

impl RegisterLpitHandler for Lpit0Ch2 {
   fn register_lpit_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpit>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLpit>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lpit() }
       }
       super::irq::set_handler(50, Some(wrapper::<F>));
       super::irq::IrqGuard::new(50)
   }
}

impl IrqLpit<super::irq::Lpit0Ch3Id> for Lpit0Ch3 {
   fn irq_lpit(&self) -> super::irq::IrqLpit0Ch3 { super::irq::IRQ_LPIT0_CH3 }
}

impl RegisterLpitHandler for Lpit0Ch3 {
   fn register_lpit_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpit>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLpit>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lpit() }
       }
       super::irq::set_handler(51, Some(wrapper::<F>));
       super::irq::IrqGuard::new(51)
   }
}

