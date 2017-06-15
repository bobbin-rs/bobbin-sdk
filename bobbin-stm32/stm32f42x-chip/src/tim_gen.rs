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

impl super::sig::Signal<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::SignalIo<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::Signal<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::SignalIo<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::Signal<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::SignalIo<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::Signal<super::sig::Tim2Ch4> for Tim2Ch4 {}
impl super::sig::SignalIo<super::sig::Tim2Ch4> for Tim2Ch4 {}

impl super::sig::Signal<super::sig::Tim3Ch1> for Tim3Ch1 {}
impl super::sig::SignalIo<super::sig::Tim3Ch1> for Tim3Ch1 {}
impl super::sig::Signal<super::sig::Tim3Ch2> for Tim3Ch2 {}
impl super::sig::SignalIo<super::sig::Tim3Ch2> for Tim3Ch2 {}
impl super::sig::Signal<super::sig::Tim3Ch3> for Tim3Ch3 {}
impl super::sig::SignalIo<super::sig::Tim3Ch3> for Tim3Ch3 {}
impl super::sig::Signal<super::sig::Tim3Ch4> for Tim3Ch4 {}
impl super::sig::SignalIo<super::sig::Tim3Ch4> for Tim3Ch4 {}

impl super::sig::Signal<super::sig::Tim4Ch1> for Tim4Ch1 {}
impl super::sig::SignalIo<super::sig::Tim4Ch1> for Tim4Ch1 {}
impl super::sig::Signal<super::sig::Tim4Ch2> for Tim4Ch2 {}
impl super::sig::SignalIo<super::sig::Tim4Ch2> for Tim4Ch2 {}
impl super::sig::Signal<super::sig::Tim4Ch3> for Tim4Ch3 {}
impl super::sig::SignalIo<super::sig::Tim4Ch3> for Tim4Ch3 {}
impl super::sig::Signal<super::sig::Tim4Ch4> for Tim4Ch4 {}
impl super::sig::SignalIo<super::sig::Tim4Ch4> for Tim4Ch4 {}

impl super::sig::Signal<super::sig::Tim5Ch1> for Tim5Ch1 {}
impl super::sig::SignalIo<super::sig::Tim5Ch1> for Tim5Ch1 {}
impl super::sig::Signal<super::sig::Tim5Ch2> for Tim5Ch2 {}
impl super::sig::SignalIo<super::sig::Tim5Ch2> for Tim5Ch2 {}
impl super::sig::Signal<super::sig::Tim5Ch3> for Tim5Ch3 {}
impl super::sig::SignalIo<super::sig::Tim5Ch3> for Tim5Ch3 {}
impl super::sig::Signal<super::sig::Tim5Ch4> for Tim5Ch4 {}
impl super::sig::SignalIo<super::sig::Tim5Ch4> for Tim5Ch4 {}

impl super::sig::Signal<super::sig::Tim9Ch1> for Tim9Ch1 {}
impl super::sig::SignalIo<super::sig::Tim9Ch1> for Tim9Ch1 {}
impl super::sig::Signal<super::sig::Tim9Ch2> for Tim9Ch2 {}
impl super::sig::SignalIo<super::sig::Tim9Ch2> for Tim9Ch2 {}

impl super::sig::Signal<super::sig::Tim10Ch1> for Tim10Ch1 {}
impl super::sig::SignalIo<super::sig::Tim10Ch1> for Tim10Ch1 {}

impl super::sig::Signal<super::sig::Tim11Ch1> for Tim11Ch1 {}
impl super::sig::SignalIo<super::sig::Tim11Ch1> for Tim11Ch1 {}

impl super::sig::Signal<super::sig::Tim12Ch1> for Tim12Ch1 {}
impl super::sig::SignalIo<super::sig::Tim12Ch1> for Tim12Ch1 {}
impl super::sig::Signal<super::sig::Tim12Ch2> for Tim12Ch2 {}
impl super::sig::SignalIo<super::sig::Tim12Ch2> for Tim12Ch2 {}

impl super::sig::Signal<super::sig::Tim13Ch1> for Tim13Ch1 {}
impl super::sig::SignalIo<super::sig::Tim13Ch1> for Tim13Ch1 {}

impl super::sig::Signal<super::sig::Tim14Ch1> for Tim14Ch1 {}
impl super::sig::SignalIo<super::sig::Tim14Ch1> for Tim14Ch1 {}


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

pub const TIM3_CH1: Channel<Tim3Ch1Id, Tim3Id> = Channel { periph: TIM3, index: 0, id: Tim3Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim3Ch1Id {}
pub type Tim3Ch1 = Channel<Tim3Ch1Id, Tim3Id>;

pub const TIM3_CH2: Channel<Tim3Ch2Id, Tim3Id> = Channel { periph: TIM3, index: 1, id: Tim3Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim3Ch2Id {}
pub type Tim3Ch2 = Channel<Tim3Ch2Id, Tim3Id>;

pub const TIM3_CH3: Channel<Tim3Ch3Id, Tim3Id> = Channel { periph: TIM3, index: 2, id: Tim3Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim3Ch3Id {}
pub type Tim3Ch3 = Channel<Tim3Ch3Id, Tim3Id>;

pub const TIM3_CH4: Channel<Tim3Ch4Id, Tim3Id> = Channel { periph: TIM3, index: 3, id: Tim3Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim3Ch4Id {}
pub type Tim3Ch4 = Channel<Tim3Ch4Id, Tim3Id>;

pub const TIM4_CH1: Channel<Tim4Ch1Id, Tim4Id> = Channel { periph: TIM4, index: 0, id: Tim4Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim4Ch1Id {}
pub type Tim4Ch1 = Channel<Tim4Ch1Id, Tim4Id>;

pub const TIM4_CH2: Channel<Tim4Ch2Id, Tim4Id> = Channel { periph: TIM4, index: 1, id: Tim4Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim4Ch2Id {}
pub type Tim4Ch2 = Channel<Tim4Ch2Id, Tim4Id>;

pub const TIM4_CH3: Channel<Tim4Ch3Id, Tim4Id> = Channel { periph: TIM4, index: 2, id: Tim4Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim4Ch3Id {}
pub type Tim4Ch3 = Channel<Tim4Ch3Id, Tim4Id>;

pub const TIM4_CH4: Channel<Tim4Ch4Id, Tim4Id> = Channel { periph: TIM4, index: 3, id: Tim4Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim4Ch4Id {}
pub type Tim4Ch4 = Channel<Tim4Ch4Id, Tim4Id>;

pub const TIM5_CH1: Channel<Tim5Ch1Id, Tim5Id> = Channel { periph: TIM5, index: 0, id: Tim5Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim5Ch1Id {}
pub type Tim5Ch1 = Channel<Tim5Ch1Id, Tim5Id>;

pub const TIM5_CH2: Channel<Tim5Ch2Id, Tim5Id> = Channel { periph: TIM5, index: 1, id: Tim5Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim5Ch2Id {}
pub type Tim5Ch2 = Channel<Tim5Ch2Id, Tim5Id>;

pub const TIM5_CH3: Channel<Tim5Ch3Id, Tim5Id> = Channel { periph: TIM5, index: 2, id: Tim5Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim5Ch3Id {}
pub type Tim5Ch3 = Channel<Tim5Ch3Id, Tim5Id>;

pub const TIM5_CH4: Channel<Tim5Ch4Id, Tim5Id> = Channel { periph: TIM5, index: 3, id: Tim5Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim5Ch4Id {}
pub type Tim5Ch4 = Channel<Tim5Ch4Id, Tim5Id>;

pub const TIM9_CH1: Channel<Tim9Ch1Id, Tim9Id> = Channel { periph: TIM9, index: 0, id: Tim9Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim9Ch1Id {}
pub type Tim9Ch1 = Channel<Tim9Ch1Id, Tim9Id>;

pub const TIM9_CH2: Channel<Tim9Ch2Id, Tim9Id> = Channel { periph: TIM9, index: 1, id: Tim9Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim9Ch2Id {}
pub type Tim9Ch2 = Channel<Tim9Ch2Id, Tim9Id>;

pub const TIM10_CH1: Channel<Tim10Ch1Id, Tim10Id> = Channel { periph: TIM10, index: 0, id: Tim10Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim10Ch1Id {}
pub type Tim10Ch1 = Channel<Tim10Ch1Id, Tim10Id>;

pub const TIM11_CH1: Channel<Tim11Ch1Id, Tim11Id> = Channel { periph: TIM11, index: 0, id: Tim11Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim11Ch1Id {}
pub type Tim11Ch1 = Channel<Tim11Ch1Id, Tim11Id>;

pub const TIM12_CH1: Channel<Tim12Ch1Id, Tim12Id> = Channel { periph: TIM12, index: 0, id: Tim12Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim12Ch1Id {}
pub type Tim12Ch1 = Channel<Tim12Ch1Id, Tim12Id>;

pub const TIM12_CH2: Channel<Tim12Ch2Id, Tim12Id> = Channel { periph: TIM12, index: 1, id: Tim12Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim12Ch2Id {}
pub type Tim12Ch2 = Channel<Tim12Ch2Id, Tim12Id>;

pub const TIM13_CH1: Channel<Tim13Ch1Id, Tim13Id> = Channel { periph: TIM13, index: 0, id: Tim13Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim13Ch1Id {}
pub type Tim13Ch1 = Channel<Tim13Ch1Id, Tim13Id>;

pub const TIM14_CH1: Channel<Tim14Ch1Id, Tim14Id> = Channel { periph: TIM14, index: 0, id: Tim14Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim14Ch1Id {}
pub type Tim14Ch1 = Channel<Tim14Ch1Id, Tim14Id>;

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
       super::irq::set_handler(28, Some(wrapper::<F>));
       super::irq::IrqGuard::new(28)
   }
}

impl IrqTim<super::irq::Tim3Id> for Tim3 {
   fn irq_tim(&self) -> super::irq::IrqTim3 { super::irq::IRQ_TIM3 }
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
   fn irq_tim(&self) -> super::irq::IrqTim4 { super::irq::IRQ_TIM4 }
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
   fn irq_tim(&self) -> super::irq::IrqTim5 { super::irq::IRQ_TIM5 }
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

impl IrqTim<super::irq::Tim9Id> for Tim9 {
   fn irq_tim(&self) -> super::irq::IrqTim9 { super::irq::IRQ_TIM9 }
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

impl IrqTim<super::irq::Tim10Id> for Tim10 {
   fn irq_tim(&self) -> super::irq::IrqTim10 { super::irq::IRQ_TIM10 }
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

impl IrqTim<super::irq::Tim11Id> for Tim11 {
   fn irq_tim(&self) -> super::irq::IrqTim11 { super::irq::IRQ_TIM11 }
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

impl IrqTim<super::irq::Tim12Id> for Tim12 {
   fn irq_tim(&self) -> super::irq::IrqTim12 { super::irq::IRQ_TIM12 }
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

impl IrqTim<super::irq::Tim13Id> for Tim13 {
   fn irq_tim(&self) -> super::irq::IrqTim13 { super::irq::IRQ_TIM13 }
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

impl IrqTim<super::irq::Tim14Id> for Tim14 {
   fn irq_tim(&self) -> super::irq::IrqTim14 { super::irq::IRQ_TIM14 }
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

