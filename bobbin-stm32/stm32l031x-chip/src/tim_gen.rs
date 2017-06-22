pub use stm32_common::chip::tim_gen::*;

pub const TIM2: Tim2 = Periph(0x40000000, Tim2Id {});
pub const TIM21: Tim21 = Periph(0x40010800, Tim21Id {});
pub const TIM22: Tim22 = Periph(0x40011400, Tim22Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim2Id {}
pub type Tim2 = Periph<Tim2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim21Id {}
pub type Tim21 = Periph<Tim21Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim22Id {}
pub type Tim22 = Periph<Tim22Id>;

impl super::sig::Signal<super::sig::Tim2Etr> for Tim2 {}
impl super::sig::SignalEtr<super::sig::Tim2Etr> for Tim2 {}
impl super::sig::Signal<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::SignalTim<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::Signal<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::SignalTim<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::Signal<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::SignalTim<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::Signal<super::sig::Tim2Ch4> for Tim2Ch4 {}
impl super::sig::SignalTim<super::sig::Tim2Ch4> for Tim2Ch4 {}

impl super::sig::Signal<super::sig::Tim21Etr> for Tim21 {}
impl super::sig::SignalEtr<super::sig::Tim21Etr> for Tim21 {}
impl super::sig::Signal<super::sig::Tim21Ch1> for Tim21Ch1 {}
impl super::sig::SignalTim<super::sig::Tim21Ch1> for Tim21Ch1 {}
impl super::sig::Signal<super::sig::Tim21Ch2> for Tim21Ch2 {}
impl super::sig::SignalTim<super::sig::Tim21Ch2> for Tim21Ch2 {}

impl super::sig::Signal<super::sig::Tim22Etr> for Tim22 {}
impl super::sig::SignalEtr<super::sig::Tim22Etr> for Tim22 {}
impl super::sig::Signal<super::sig::Tim22Ch1> for Tim22Ch1 {}
impl super::sig::SignalTim<super::sig::Tim22Ch1> for Tim22Ch1 {}
impl super::sig::Signal<super::sig::Tim22Ch2> for Tim22Ch2 {}
impl super::sig::SignalTim<super::sig::Tim22Ch2> for Tim22Ch2 {}


pub const TIM2_CH1: Channel<Tim2Ch1Id, Tim2Id> = Channel { periph: TIM2, index: 0, id: Tim2Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim2Ch1Id {}
pub type Tim2Ch1 = Channel<Tim2Ch1Id, Tim2Id>;

pub const TIM2_CH2: Channel<Tim2Ch2Id, Tim2Id> = Channel { periph: TIM2, index: 1, id: Tim2Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim2Ch2Id {}
pub type Tim2Ch2 = Channel<Tim2Ch2Id, Tim2Id>;

pub const TIM2_CH3: Channel<Tim2Ch3Id, Tim2Id> = Channel { periph: TIM2, index: 2, id: Tim2Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim2Ch3Id {}
pub type Tim2Ch3 = Channel<Tim2Ch3Id, Tim2Id>;

pub const TIM2_CH4: Channel<Tim2Ch4Id, Tim2Id> = Channel { periph: TIM2, index: 3, id: Tim2Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim2Ch4Id {}
pub type Tim2Ch4 = Channel<Tim2Ch4Id, Tim2Id>;

pub const TIM21_CH1: Channel<Tim21Ch1Id, Tim21Id> = Channel { periph: TIM21, index: 0, id: Tim21Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim21Ch1Id {}
pub type Tim21Ch1 = Channel<Tim21Ch1Id, Tim21Id>;

pub const TIM21_CH2: Channel<Tim21Ch2Id, Tim21Id> = Channel { periph: TIM21, index: 1, id: Tim21Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim21Ch2Id {}
pub type Tim21Ch2 = Channel<Tim21Ch2Id, Tim21Id>;

pub const TIM22_CH1: Channel<Tim22Ch1Id, Tim22Id> = Channel { periph: TIM22, index: 0, id: Tim22Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim22Ch1Id {}
pub type Tim22Ch1 = Channel<Tim22Ch1Id, Tim22Id>;

pub const TIM22_CH2: Channel<Tim22Ch2Id, Tim22Id> = Channel { periph: TIM22, index: 1, id: Tim22Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim22Ch2Id {}
pub type Tim22Ch2 = Channel<Tim22Ch2Id, Tim22Id>;

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
   fn irq_tim(&self) -> super::irq::IrqTim2 { super::irq::IRQ_TIM2 }
}

impl RegisterTimHandler for Tim2 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(15, Some(wrapper::<F>));
       super::irq::IrqGuard::new(15)
   }
}

impl IrqTim<super::irq::Tim21Id> for Tim21 {
   fn irq_tim(&self) -> super::irq::IrqTim21 { super::irq::IRQ_TIM21 }
}

impl RegisterTimHandler for Tim21 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(20, Some(wrapper::<F>));
       super::irq::IrqGuard::new(20)
   }
}

impl IrqTim<super::irq::Tim22Id> for Tim22 {
   fn irq_tim(&self) -> super::irq::IrqTim22 { super::irq::IRQ_TIM22 }
}

impl RegisterTimHandler for Tim22 {
   fn register_tim_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleTim>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleTim>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_tim() }
       }
       super::irq::set_handler(22, Some(wrapper::<F>));
       super::irq::IrqGuard::new(22)
   }
}

