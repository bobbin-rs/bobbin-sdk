#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::ftm::*;

pub const FTM0: Ftm0 = Periph(0x40038000, Ftm0Id {});
pub const FTM1: Ftm1 = Periph(0x40039000, Ftm1Id {});
pub const FTM2: Ftm2 = Periph(0x4003a000, Ftm2Id {});
pub const FTM3: Ftm3 = Periph(0x40026000, Ftm3Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ftm0Id {}
pub type Ftm0 = Periph<Ftm0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ftm1Id {}
pub type Ftm1 = Periph<Ftm1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ftm2Id {}
pub type Ftm2 = Periph<Ftm2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ftm3Id {}
pub type Ftm3 = Periph<Ftm3Id>;

impl super::sig::Signal<super::sig::Ftm0Ch0> for Ftm0Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch0> for Ftm0Ch0 {}
impl super::sig::Signal<super::sig::Ftm0Ch1> for Ftm0Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch1> for Ftm0Ch1 {}
impl super::sig::Signal<super::sig::Ftm0Ch2> for Ftm0Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch2> for Ftm0Ch2 {}
impl super::sig::Signal<super::sig::Ftm0Ch3> for Ftm0Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch3> for Ftm0Ch3 {}
impl super::sig::Signal<super::sig::Ftm0Ch4> for Ftm0Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch4> for Ftm0Ch4 {}
impl super::sig::Signal<super::sig::Ftm0Ch5> for Ftm0Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch5> for Ftm0Ch5 {}
impl super::sig::Signal<super::sig::Ftm0Ch6> for Ftm0Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch6> for Ftm0Ch6 {}
impl super::sig::Signal<super::sig::Ftm0Ch7> for Ftm0Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch7> for Ftm0Ch7 {}

impl super::sig::Signal<super::sig::Ftm1Ch0> for Ftm1Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch0> for Ftm1Ch0 {}
impl super::sig::Signal<super::sig::Ftm1Ch1> for Ftm1Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch1> for Ftm1Ch1 {}
impl super::sig::Signal<super::sig::Ftm1Ch2> for Ftm1Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch2> for Ftm1Ch2 {}
impl super::sig::Signal<super::sig::Ftm1Ch3> for Ftm1Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch3> for Ftm1Ch3 {}
impl super::sig::Signal<super::sig::Ftm1Ch4> for Ftm1Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch4> for Ftm1Ch4 {}
impl super::sig::Signal<super::sig::Ftm1Ch5> for Ftm1Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch5> for Ftm1Ch5 {}
impl super::sig::Signal<super::sig::Ftm1Ch6> for Ftm1Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch6> for Ftm1Ch6 {}
impl super::sig::Signal<super::sig::Ftm1Ch7> for Ftm1Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch7> for Ftm1Ch7 {}

impl super::sig::Signal<super::sig::Ftm2Ch0> for Ftm2Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch0> for Ftm2Ch0 {}
impl super::sig::Signal<super::sig::Ftm2Ch1> for Ftm2Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch1> for Ftm2Ch1 {}
impl super::sig::Signal<super::sig::Ftm2Ch2> for Ftm2Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch2> for Ftm2Ch2 {}
impl super::sig::Signal<super::sig::Ftm2Ch3> for Ftm2Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch3> for Ftm2Ch3 {}
impl super::sig::Signal<super::sig::Ftm2Ch4> for Ftm2Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch4> for Ftm2Ch4 {}
impl super::sig::Signal<super::sig::Ftm2Ch5> for Ftm2Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch5> for Ftm2Ch5 {}
impl super::sig::Signal<super::sig::Ftm2Ch6> for Ftm2Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch6> for Ftm2Ch6 {}
impl super::sig::Signal<super::sig::Ftm2Ch7> for Ftm2Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch7> for Ftm2Ch7 {}

impl super::sig::Signal<super::sig::Ftm3Ch0> for Ftm3Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch0> for Ftm3Ch0 {}
impl super::sig::Signal<super::sig::Ftm3Ch1> for Ftm3Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch1> for Ftm3Ch1 {}
impl super::sig::Signal<super::sig::Ftm3Ch2> for Ftm3Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch2> for Ftm3Ch2 {}
impl super::sig::Signal<super::sig::Ftm3Ch3> for Ftm3Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch3> for Ftm3Ch3 {}
impl super::sig::Signal<super::sig::Ftm3Ch4> for Ftm3Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch4> for Ftm3Ch4 {}
impl super::sig::Signal<super::sig::Ftm3Ch5> for Ftm3Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch5> for Ftm3Ch5 {}
impl super::sig::Signal<super::sig::Ftm3Ch6> for Ftm3Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch6> for Ftm3Ch6 {}
impl super::sig::Signal<super::sig::Ftm3Ch7> for Ftm3Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch7> for Ftm3Ch7 {}


pub const FTM0_CH0: Channel<Ftm0Ch0Id, Ftm0Id> = Channel { periph: FTM0, index: 0, id: Ftm0Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch0Id {}
pub type Ftm0Ch0 = Channel<Ftm0Ch0Id, Ftm0Id>;

pub const FTM0_CH1: Channel<Ftm0Ch1Id, Ftm0Id> = Channel { periph: FTM0, index: 1, id: Ftm0Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch1Id {}
pub type Ftm0Ch1 = Channel<Ftm0Ch1Id, Ftm0Id>;

pub const FTM0_CH2: Channel<Ftm0Ch2Id, Ftm0Id> = Channel { periph: FTM0, index: 2, id: Ftm0Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch2Id {}
pub type Ftm0Ch2 = Channel<Ftm0Ch2Id, Ftm0Id>;

pub const FTM0_CH3: Channel<Ftm0Ch3Id, Ftm0Id> = Channel { periph: FTM0, index: 3, id: Ftm0Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch3Id {}
pub type Ftm0Ch3 = Channel<Ftm0Ch3Id, Ftm0Id>;

pub const FTM0_CH4: Channel<Ftm0Ch4Id, Ftm0Id> = Channel { periph: FTM0, index: 4, id: Ftm0Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch4Id {}
pub type Ftm0Ch4 = Channel<Ftm0Ch4Id, Ftm0Id>;

pub const FTM0_CH5: Channel<Ftm0Ch5Id, Ftm0Id> = Channel { periph: FTM0, index: 5, id: Ftm0Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch5Id {}
pub type Ftm0Ch5 = Channel<Ftm0Ch5Id, Ftm0Id>;

pub const FTM0_CH6: Channel<Ftm0Ch6Id, Ftm0Id> = Channel { periph: FTM0, index: 6, id: Ftm0Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch6Id {}
pub type Ftm0Ch6 = Channel<Ftm0Ch6Id, Ftm0Id>;

pub const FTM0_CH7: Channel<Ftm0Ch7Id, Ftm0Id> = Channel { periph: FTM0, index: 7, id: Ftm0Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch7Id {}
pub type Ftm0Ch7 = Channel<Ftm0Ch7Id, Ftm0Id>;

pub const FTM1_CH0: Channel<Ftm1Ch0Id, Ftm1Id> = Channel { periph: FTM1, index: 0, id: Ftm1Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch0Id {}
pub type Ftm1Ch0 = Channel<Ftm1Ch0Id, Ftm1Id>;

pub const FTM1_CH1: Channel<Ftm1Ch1Id, Ftm1Id> = Channel { periph: FTM1, index: 1, id: Ftm1Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch1Id {}
pub type Ftm1Ch1 = Channel<Ftm1Ch1Id, Ftm1Id>;

pub const FTM1_CH2: Channel<Ftm1Ch2Id, Ftm1Id> = Channel { periph: FTM1, index: 2, id: Ftm1Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch2Id {}
pub type Ftm1Ch2 = Channel<Ftm1Ch2Id, Ftm1Id>;

pub const FTM1_CH3: Channel<Ftm1Ch3Id, Ftm1Id> = Channel { periph: FTM1, index: 3, id: Ftm1Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch3Id {}
pub type Ftm1Ch3 = Channel<Ftm1Ch3Id, Ftm1Id>;

pub const FTM1_CH4: Channel<Ftm1Ch4Id, Ftm1Id> = Channel { periph: FTM1, index: 4, id: Ftm1Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch4Id {}
pub type Ftm1Ch4 = Channel<Ftm1Ch4Id, Ftm1Id>;

pub const FTM1_CH5: Channel<Ftm1Ch5Id, Ftm1Id> = Channel { periph: FTM1, index: 5, id: Ftm1Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch5Id {}
pub type Ftm1Ch5 = Channel<Ftm1Ch5Id, Ftm1Id>;

pub const FTM1_CH6: Channel<Ftm1Ch6Id, Ftm1Id> = Channel { periph: FTM1, index: 6, id: Ftm1Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch6Id {}
pub type Ftm1Ch6 = Channel<Ftm1Ch6Id, Ftm1Id>;

pub const FTM1_CH7: Channel<Ftm1Ch7Id, Ftm1Id> = Channel { periph: FTM1, index: 7, id: Ftm1Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch7Id {}
pub type Ftm1Ch7 = Channel<Ftm1Ch7Id, Ftm1Id>;

pub const FTM2_CH0: Channel<Ftm2Ch0Id, Ftm2Id> = Channel { periph: FTM2, index: 0, id: Ftm2Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch0Id {}
pub type Ftm2Ch0 = Channel<Ftm2Ch0Id, Ftm2Id>;

pub const FTM2_CH1: Channel<Ftm2Ch1Id, Ftm2Id> = Channel { periph: FTM2, index: 1, id: Ftm2Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch1Id {}
pub type Ftm2Ch1 = Channel<Ftm2Ch1Id, Ftm2Id>;

pub const FTM2_CH2: Channel<Ftm2Ch2Id, Ftm2Id> = Channel { periph: FTM2, index: 2, id: Ftm2Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch2Id {}
pub type Ftm2Ch2 = Channel<Ftm2Ch2Id, Ftm2Id>;

pub const FTM2_CH3: Channel<Ftm2Ch3Id, Ftm2Id> = Channel { periph: FTM2, index: 3, id: Ftm2Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch3Id {}
pub type Ftm2Ch3 = Channel<Ftm2Ch3Id, Ftm2Id>;

pub const FTM2_CH4: Channel<Ftm2Ch4Id, Ftm2Id> = Channel { periph: FTM2, index: 4, id: Ftm2Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch4Id {}
pub type Ftm2Ch4 = Channel<Ftm2Ch4Id, Ftm2Id>;

pub const FTM2_CH5: Channel<Ftm2Ch5Id, Ftm2Id> = Channel { periph: FTM2, index: 5, id: Ftm2Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch5Id {}
pub type Ftm2Ch5 = Channel<Ftm2Ch5Id, Ftm2Id>;

pub const FTM2_CH6: Channel<Ftm2Ch6Id, Ftm2Id> = Channel { periph: FTM2, index: 6, id: Ftm2Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch6Id {}
pub type Ftm2Ch6 = Channel<Ftm2Ch6Id, Ftm2Id>;

pub const FTM2_CH7: Channel<Ftm2Ch7Id, Ftm2Id> = Channel { periph: FTM2, index: 7, id: Ftm2Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch7Id {}
pub type Ftm2Ch7 = Channel<Ftm2Ch7Id, Ftm2Id>;

pub const FTM3_CH0: Channel<Ftm3Ch0Id, Ftm3Id> = Channel { periph: FTM3, index: 0, id: Ftm3Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm3Ch0Id {}
pub type Ftm3Ch0 = Channel<Ftm3Ch0Id, Ftm3Id>;

pub const FTM3_CH1: Channel<Ftm3Ch1Id, Ftm3Id> = Channel { periph: FTM3, index: 1, id: Ftm3Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm3Ch1Id {}
pub type Ftm3Ch1 = Channel<Ftm3Ch1Id, Ftm3Id>;

pub const FTM3_CH2: Channel<Ftm3Ch2Id, Ftm3Id> = Channel { periph: FTM3, index: 2, id: Ftm3Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm3Ch2Id {}
pub type Ftm3Ch2 = Channel<Ftm3Ch2Id, Ftm3Id>;

pub const FTM3_CH3: Channel<Ftm3Ch3Id, Ftm3Id> = Channel { periph: FTM3, index: 3, id: Ftm3Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm3Ch3Id {}
pub type Ftm3Ch3 = Channel<Ftm3Ch3Id, Ftm3Id>;

pub const FTM3_CH4: Channel<Ftm3Ch4Id, Ftm3Id> = Channel { periph: FTM3, index: 4, id: Ftm3Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm3Ch4Id {}
pub type Ftm3Ch4 = Channel<Ftm3Ch4Id, Ftm3Id>;

pub const FTM3_CH5: Channel<Ftm3Ch5Id, Ftm3Id> = Channel { periph: FTM3, index: 5, id: Ftm3Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm3Ch5Id {}
pub type Ftm3Ch5 = Channel<Ftm3Ch5Id, Ftm3Id>;

pub const FTM3_CH6: Channel<Ftm3Ch6Id, Ftm3Id> = Channel { periph: FTM3, index: 6, id: Ftm3Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm3Ch6Id {}
pub type Ftm3Ch6 = Channel<Ftm3Ch6Id, Ftm3Id>;

pub const FTM3_CH7: Channel<Ftm3Ch7Id, Ftm3Id> = Channel { periph: FTM3, index: 7, id: Ftm3Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm3Ch7Id {}
pub type Ftm3Ch7 = Channel<Ftm3Ch7Id, Ftm3Id>;

pub trait IrqFtmFault<T> {
   fn irq_ftm_fault(&self) -> super::irq::Irq<T>;
}

pub trait RegisterFtmFaultHandler {
   fn register_ftm_fault_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtmFault>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleFtmFault {
   fn handle_ftm_fault(&self);
}

pub trait IrqFtmOverflow<T> {
   fn irq_ftm_overflow(&self) -> super::irq::Irq<T>;
}

pub trait RegisterFtmOverflowHandler {
   fn register_ftm_overflow_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtmOverflow>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleFtmOverflow {
   fn handle_ftm_overflow(&self);
}

pub trait IrqFtm<T> {
   fn irq_ftm(&self) -> super::irq::Irq<T>;
}

pub trait RegisterFtmHandler {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleFtm {
   fn handle_ftm(&self);
}

impl IrqFtmFault<super::irq::Ftm0FaultId> for Ftm0 {
   fn irq_ftm_fault(&self) -> super::irq::IrqFtm0Fault { super::irq::IRQ_FTM0_FAULT }
}

impl RegisterFtmFaultHandler for Ftm0 {
   fn register_ftm_fault_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtmFault>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtmFault>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm_fault() }
       }
       super::irq::set_handler(103, Some(wrapper::<F>));
       super::irq::IrqGuard::new(103)
   }
}

impl IrqFtmOverflow<super::irq::Ftm0OverflowId> for Ftm0 {
   fn irq_ftm_overflow(&self) -> super::irq::IrqFtm0Overflow { super::irq::IRQ_FTM0_OVERFLOW }
}

impl RegisterFtmOverflowHandler for Ftm0 {
   fn register_ftm_overflow_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtmOverflow>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtmOverflow>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm_overflow() }
       }
       super::irq::set_handler(104, Some(wrapper::<F>));
       super::irq::IrqGuard::new(104)
   }
}

impl IrqFtm<super::irq::Ftm0Ch0Id> for Ftm0Ch0 {
   fn irq_ftm(&self) -> super::irq::IrqFtm0Ch0 { super::irq::IRQ_FTM0_CH0 }
}

impl RegisterFtmHandler for Ftm0Ch0 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(99, Some(wrapper::<F>));
       super::irq::IrqGuard::new(99)
   }
}

impl IrqFtm<super::irq::Ftm0Ch1Id> for Ftm0Ch1 {
   fn irq_ftm(&self) -> super::irq::IrqFtm0Ch1 { super::irq::IRQ_FTM0_CH1 }
}

impl RegisterFtmHandler for Ftm0Ch1 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(99, Some(wrapper::<F>));
       super::irq::IrqGuard::new(99)
   }
}

impl IrqFtm<super::irq::Ftm0Ch2Id> for Ftm0Ch2 {
   fn irq_ftm(&self) -> super::irq::IrqFtm0Ch2 { super::irq::IRQ_FTM0_CH2 }
}

impl RegisterFtmHandler for Ftm0Ch2 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(100, Some(wrapper::<F>));
       super::irq::IrqGuard::new(100)
   }
}

impl IrqFtm<super::irq::Ftm0Ch3Id> for Ftm0Ch3 {
   fn irq_ftm(&self) -> super::irq::IrqFtm0Ch3 { super::irq::IRQ_FTM0_CH3 }
}

impl RegisterFtmHandler for Ftm0Ch3 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(100, Some(wrapper::<F>));
       super::irq::IrqGuard::new(100)
   }
}

impl IrqFtm<super::irq::Ftm0Ch4Id> for Ftm0Ch4 {
   fn irq_ftm(&self) -> super::irq::IrqFtm0Ch4 { super::irq::IRQ_FTM0_CH4 }
}

impl RegisterFtmHandler for Ftm0Ch4 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(101, Some(wrapper::<F>));
       super::irq::IrqGuard::new(101)
   }
}

impl IrqFtm<super::irq::Ftm0Ch5Id> for Ftm0Ch5 {
   fn irq_ftm(&self) -> super::irq::IrqFtm0Ch5 { super::irq::IRQ_FTM0_CH5 }
}

impl RegisterFtmHandler for Ftm0Ch5 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(101, Some(wrapper::<F>));
       super::irq::IrqGuard::new(101)
   }
}

impl IrqFtm<super::irq::Ftm0Ch6Id> for Ftm0Ch6 {
   fn irq_ftm(&self) -> super::irq::IrqFtm0Ch6 { super::irq::IRQ_FTM0_CH6 }
}

impl RegisterFtmHandler for Ftm0Ch6 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(102, Some(wrapper::<F>));
       super::irq::IrqGuard::new(102)
   }
}

impl IrqFtm<super::irq::Ftm0Ch7Id> for Ftm0Ch7 {
   fn irq_ftm(&self) -> super::irq::IrqFtm0Ch7 { super::irq::IRQ_FTM0_CH7 }
}

impl RegisterFtmHandler for Ftm0Ch7 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(102, Some(wrapper::<F>));
       super::irq::IrqGuard::new(102)
   }
}

impl IrqFtmFault<super::irq::Ftm1FaultId> for Ftm1 {
   fn irq_ftm_fault(&self) -> super::irq::IrqFtm1Fault { super::irq::IRQ_FTM1_FAULT }
}

impl RegisterFtmFaultHandler for Ftm1 {
   fn register_ftm_fault_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtmFault>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtmFault>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm_fault() }
       }
       super::irq::set_handler(109, Some(wrapper::<F>));
       super::irq::IrqGuard::new(109)
   }
}

impl IrqFtmOverflow<super::irq::Ftm1OverflowId> for Ftm1 {
   fn irq_ftm_overflow(&self) -> super::irq::IrqFtm1Overflow { super::irq::IRQ_FTM1_OVERFLOW }
}

impl RegisterFtmOverflowHandler for Ftm1 {
   fn register_ftm_overflow_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtmOverflow>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtmOverflow>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm_overflow() }
       }
       super::irq::set_handler(110, Some(wrapper::<F>));
       super::irq::IrqGuard::new(110)
   }
}

impl IrqFtm<super::irq::Ftm1Ch0Id> for Ftm1Ch0 {
   fn irq_ftm(&self) -> super::irq::IrqFtm1Ch0 { super::irq::IRQ_FTM1_CH0 }
}

impl RegisterFtmHandler for Ftm1Ch0 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(105, Some(wrapper::<F>));
       super::irq::IrqGuard::new(105)
   }
}

impl IrqFtm<super::irq::Ftm1Ch1Id> for Ftm1Ch1 {
   fn irq_ftm(&self) -> super::irq::IrqFtm1Ch1 { super::irq::IRQ_FTM1_CH1 }
}

impl RegisterFtmHandler for Ftm1Ch1 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(105, Some(wrapper::<F>));
       super::irq::IrqGuard::new(105)
   }
}

impl IrqFtm<super::irq::Ftm1Ch2Id> for Ftm1Ch2 {
   fn irq_ftm(&self) -> super::irq::IrqFtm1Ch2 { super::irq::IRQ_FTM1_CH2 }
}

impl RegisterFtmHandler for Ftm1Ch2 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(106, Some(wrapper::<F>));
       super::irq::IrqGuard::new(106)
   }
}

impl IrqFtm<super::irq::Ftm1Ch3Id> for Ftm1Ch3 {
   fn irq_ftm(&self) -> super::irq::IrqFtm1Ch3 { super::irq::IRQ_FTM1_CH3 }
}

impl RegisterFtmHandler for Ftm1Ch3 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(106, Some(wrapper::<F>));
       super::irq::IrqGuard::new(106)
   }
}

impl IrqFtm<super::irq::Ftm1Ch4Id> for Ftm1Ch4 {
   fn irq_ftm(&self) -> super::irq::IrqFtm1Ch4 { super::irq::IRQ_FTM1_CH4 }
}

impl RegisterFtmHandler for Ftm1Ch4 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(107, Some(wrapper::<F>));
       super::irq::IrqGuard::new(107)
   }
}

impl IrqFtm<super::irq::Ftm1Ch5Id> for Ftm1Ch5 {
   fn irq_ftm(&self) -> super::irq::IrqFtm1Ch5 { super::irq::IRQ_FTM1_CH5 }
}

impl RegisterFtmHandler for Ftm1Ch5 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(107, Some(wrapper::<F>));
       super::irq::IrqGuard::new(107)
   }
}

impl IrqFtm<super::irq::Ftm1Ch6Id> for Ftm1Ch6 {
   fn irq_ftm(&self) -> super::irq::IrqFtm1Ch6 { super::irq::IRQ_FTM1_CH6 }
}

impl RegisterFtmHandler for Ftm1Ch6 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(108, Some(wrapper::<F>));
       super::irq::IrqGuard::new(108)
   }
}

impl IrqFtm<super::irq::Ftm1Ch7Id> for Ftm1Ch7 {
   fn irq_ftm(&self) -> super::irq::IrqFtm1Ch7 { super::irq::IRQ_FTM1_CH7 }
}

impl RegisterFtmHandler for Ftm1Ch7 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(108, Some(wrapper::<F>));
       super::irq::IrqGuard::new(108)
   }
}

impl IrqFtmFault<super::irq::Ftm2FaultId> for Ftm2 {
   fn irq_ftm_fault(&self) -> super::irq::IrqFtm2Fault { super::irq::IRQ_FTM2_FAULT }
}

impl RegisterFtmFaultHandler for Ftm2 {
   fn register_ftm_fault_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtmFault>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtmFault>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm_fault() }
       }
       super::irq::set_handler(115, Some(wrapper::<F>));
       super::irq::IrqGuard::new(115)
   }
}

impl IrqFtmOverflow<super::irq::Ftm2OverflowId> for Ftm2 {
   fn irq_ftm_overflow(&self) -> super::irq::IrqFtm2Overflow { super::irq::IRQ_FTM2_OVERFLOW }
}

impl RegisterFtmOverflowHandler for Ftm2 {
   fn register_ftm_overflow_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtmOverflow>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtmOverflow>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm_overflow() }
       }
       super::irq::set_handler(116, Some(wrapper::<F>));
       super::irq::IrqGuard::new(116)
   }
}

impl IrqFtm<super::irq::Ftm2Ch0Id> for Ftm2Ch0 {
   fn irq_ftm(&self) -> super::irq::IrqFtm2Ch0 { super::irq::IRQ_FTM2_CH0 }
}

impl RegisterFtmHandler for Ftm2Ch0 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(111, Some(wrapper::<F>));
       super::irq::IrqGuard::new(111)
   }
}

impl IrqFtm<super::irq::Ftm2Ch1Id> for Ftm2Ch1 {
   fn irq_ftm(&self) -> super::irq::IrqFtm2Ch1 { super::irq::IRQ_FTM2_CH1 }
}

impl RegisterFtmHandler for Ftm2Ch1 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(111, Some(wrapper::<F>));
       super::irq::IrqGuard::new(111)
   }
}

impl IrqFtm<super::irq::Ftm2Ch2Id> for Ftm2Ch2 {
   fn irq_ftm(&self) -> super::irq::IrqFtm2Ch2 { super::irq::IRQ_FTM2_CH2 }
}

impl RegisterFtmHandler for Ftm2Ch2 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(112, Some(wrapper::<F>));
       super::irq::IrqGuard::new(112)
   }
}

impl IrqFtm<super::irq::Ftm2Ch3Id> for Ftm2Ch3 {
   fn irq_ftm(&self) -> super::irq::IrqFtm2Ch3 { super::irq::IRQ_FTM2_CH3 }
}

impl RegisterFtmHandler for Ftm2Ch3 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(112, Some(wrapper::<F>));
       super::irq::IrqGuard::new(112)
   }
}

impl IrqFtm<super::irq::Ftm2Ch4Id> for Ftm2Ch4 {
   fn irq_ftm(&self) -> super::irq::IrqFtm2Ch4 { super::irq::IRQ_FTM2_CH4 }
}

impl RegisterFtmHandler for Ftm2Ch4 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(113, Some(wrapper::<F>));
       super::irq::IrqGuard::new(113)
   }
}

impl IrqFtm<super::irq::Ftm2Ch5Id> for Ftm2Ch5 {
   fn irq_ftm(&self) -> super::irq::IrqFtm2Ch5 { super::irq::IRQ_FTM2_CH5 }
}

impl RegisterFtmHandler for Ftm2Ch5 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(113, Some(wrapper::<F>));
       super::irq::IrqGuard::new(113)
   }
}

impl IrqFtm<super::irq::Ftm2Ch6Id> for Ftm2Ch6 {
   fn irq_ftm(&self) -> super::irq::IrqFtm2Ch6 { super::irq::IRQ_FTM2_CH6 }
}

impl RegisterFtmHandler for Ftm2Ch6 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(114, Some(wrapper::<F>));
       super::irq::IrqGuard::new(114)
   }
}

impl IrqFtm<super::irq::Ftm2Ch7Id> for Ftm2Ch7 {
   fn irq_ftm(&self) -> super::irq::IrqFtm2Ch7 { super::irq::IRQ_FTM2_CH7 }
}

impl RegisterFtmHandler for Ftm2Ch7 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(114, Some(wrapper::<F>));
       super::irq::IrqGuard::new(114)
   }
}

impl IrqFtmFault<super::irq::Ftm3FaultId> for Ftm3 {
   fn irq_ftm_fault(&self) -> super::irq::IrqFtm3Fault { super::irq::IRQ_FTM3_FAULT }
}

impl RegisterFtmFaultHandler for Ftm3 {
   fn register_ftm_fault_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtmFault>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtmFault>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm_fault() }
       }
       super::irq::set_handler(121, Some(wrapper::<F>));
       super::irq::IrqGuard::new(121)
   }
}

impl IrqFtmOverflow<super::irq::Ftm3OverflowId> for Ftm3 {
   fn irq_ftm_overflow(&self) -> super::irq::IrqFtm3Overflow { super::irq::IRQ_FTM3_OVERFLOW }
}

impl RegisterFtmOverflowHandler for Ftm3 {
   fn register_ftm_overflow_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtmOverflow>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtmOverflow>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm_overflow() }
       }
       super::irq::set_handler(122, Some(wrapper::<F>));
       super::irq::IrqGuard::new(122)
   }
}

impl IrqFtm<super::irq::Ftm3Ch0Id> for Ftm3Ch0 {
   fn irq_ftm(&self) -> super::irq::IrqFtm3Ch0 { super::irq::IRQ_FTM3_CH0 }
}

impl RegisterFtmHandler for Ftm3Ch0 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(117, Some(wrapper::<F>));
       super::irq::IrqGuard::new(117)
   }
}

impl IrqFtm<super::irq::Ftm3Ch1Id> for Ftm3Ch1 {
   fn irq_ftm(&self) -> super::irq::IrqFtm3Ch1 { super::irq::IRQ_FTM3_CH1 }
}

impl RegisterFtmHandler for Ftm3Ch1 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(117, Some(wrapper::<F>));
       super::irq::IrqGuard::new(117)
   }
}

impl IrqFtm<super::irq::Ftm3Ch2Id> for Ftm3Ch2 {
   fn irq_ftm(&self) -> super::irq::IrqFtm3Ch2 { super::irq::IRQ_FTM3_CH2 }
}

impl RegisterFtmHandler for Ftm3Ch2 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(118, Some(wrapper::<F>));
       super::irq::IrqGuard::new(118)
   }
}

impl IrqFtm<super::irq::Ftm3Ch3Id> for Ftm3Ch3 {
   fn irq_ftm(&self) -> super::irq::IrqFtm3Ch3 { super::irq::IRQ_FTM3_CH3 }
}

impl RegisterFtmHandler for Ftm3Ch3 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(118, Some(wrapper::<F>));
       super::irq::IrqGuard::new(118)
   }
}

impl IrqFtm<super::irq::Ftm3Ch4Id> for Ftm3Ch4 {
   fn irq_ftm(&self) -> super::irq::IrqFtm3Ch4 { super::irq::IRQ_FTM3_CH4 }
}

impl RegisterFtmHandler for Ftm3Ch4 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(119, Some(wrapper::<F>));
       super::irq::IrqGuard::new(119)
   }
}

impl IrqFtm<super::irq::Ftm3Ch5Id> for Ftm3Ch5 {
   fn irq_ftm(&self) -> super::irq::IrqFtm3Ch5 { super::irq::IRQ_FTM3_CH5 }
}

impl RegisterFtmHandler for Ftm3Ch5 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(119, Some(wrapper::<F>));
       super::irq::IrqGuard::new(119)
   }
}

impl IrqFtm<super::irq::Ftm3Ch6Id> for Ftm3Ch6 {
   fn irq_ftm(&self) -> super::irq::IrqFtm3Ch6 { super::irq::IRQ_FTM3_CH6 }
}

impl RegisterFtmHandler for Ftm3Ch6 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(120, Some(wrapper::<F>));
       super::irq::IrqGuard::new(120)
   }
}

impl IrqFtm<super::irq::Ftm3Ch7Id> for Ftm3Ch7 {
   fn irq_ftm(&self) -> super::irq::IrqFtm3Ch7 { super::irq::IRQ_FTM3_CH7 }
}

impl RegisterFtmHandler for Ftm3Ch7 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(120, Some(wrapper::<F>));
       super::irq::IrqGuard::new(120)
   }
}

