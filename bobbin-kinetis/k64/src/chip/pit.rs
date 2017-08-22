#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::pit::*;

pub const PIT: Pit = Periph(0x40037000, PitId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct PitId {}
pub type Pit = Periph<PitId>;



pub const PIT_CH0: Channel<PitCh0Id, PitId> = Channel { periph: PIT, index: 0, id: PitCh0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct PitCh0Id {}
pub type PitCh0 = Channel<PitCh0Id, PitId>;

pub const PIT_CH1: Channel<PitCh1Id, PitId> = Channel { periph: PIT, index: 1, id: PitCh1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct PitCh1Id {}
pub type PitCh1 = Channel<PitCh1Id, PitId>;

pub const PIT_CH2: Channel<PitCh2Id, PitId> = Channel { periph: PIT, index: 2, id: PitCh2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct PitCh2Id {}
pub type PitCh2 = Channel<PitCh2Id, PitId>;

pub const PIT_CH3: Channel<PitCh3Id, PitId> = Channel { periph: PIT, index: 3, id: PitCh3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct PitCh3Id {}
pub type PitCh3 = Channel<PitCh3Id, PitId>;

pub trait IrqPit<T> {
   fn irq_pit(&self) -> super::irq::Irq<T>;
}

pub trait RegisterPitHandler {
   fn register_pit_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandlePit>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandlePit {
   fn handle_pit(&self);
}

impl IrqPit<super::irq::Pit0Id> for PitCh0 {
   fn irq_pit(&self) -> super::irq::IrqPit0 { super::irq::IRQ_PIT0 }
}

impl RegisterPitHandler for PitCh0 {
   fn register_pit_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandlePit>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandlePit>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_pit() }
       }
       super::irq::set_handler(48, Some(wrapper::<F>));
       super::irq::IrqGuard::new(48)
   }
}

impl IrqPit<super::irq::Pit1Id> for PitCh1 {
   fn irq_pit(&self) -> super::irq::IrqPit1 { super::irq::IRQ_PIT1 }
}

impl RegisterPitHandler for PitCh1 {
   fn register_pit_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandlePit>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandlePit>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_pit() }
       }
       super::irq::set_handler(49, Some(wrapper::<F>));
       super::irq::IrqGuard::new(49)
   }
}

impl IrqPit<super::irq::Pit2Id> for PitCh2 {
   fn irq_pit(&self) -> super::irq::IrqPit2 { super::irq::IRQ_PIT2 }
}

impl RegisterPitHandler for PitCh2 {
   fn register_pit_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandlePit>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandlePit>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_pit() }
       }
       super::irq::set_handler(50, Some(wrapper::<F>));
       super::irq::IrqGuard::new(50)
   }
}

impl IrqPit<super::irq::Pit3Id> for PitCh3 {
   fn irq_pit(&self) -> super::irq::IrqPit3 { super::irq::IRQ_PIT3 }
}

impl RegisterPitHandler for PitCh3 {
   fn register_pit_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandlePit>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandlePit>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_pit() }
       }
       super::irq::set_handler(51, Some(wrapper::<F>));
       super::irq::IrqGuard::new(51)
   }
}

