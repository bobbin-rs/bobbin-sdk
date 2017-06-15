pub use stm32_common::chip::tim_adv::*;

pub const TIM1: Tim1 = Periph(0x40010000, Tim1Id {});
pub const TIM8: Tim8 = Periph(0x40010400, Tim8Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim1Id {}
pub type Tim1 = Periph<Tim1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim8Id {}
pub type Tim8 = Periph<Tim8Id>;

impl super::sig::Signal<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::SignalTim<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::Signal<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::SignalTim<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::Signal<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::SignalTim<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::Signal<super::sig::Tim1Ch4> for Tim1Ch4 {}
impl super::sig::SignalTim<super::sig::Tim1Ch4> for Tim1Ch4 {}

impl super::sig::Signal<super::sig::Tim8Ch1> for Tim8Ch1 {}
impl super::sig::SignalTim<super::sig::Tim8Ch1> for Tim8Ch1 {}
impl super::sig::Signal<super::sig::Tim8Ch2> for Tim8Ch2 {}
impl super::sig::SignalTim<super::sig::Tim8Ch2> for Tim8Ch2 {}
impl super::sig::Signal<super::sig::Tim8Ch3> for Tim8Ch3 {}
impl super::sig::SignalTim<super::sig::Tim8Ch3> for Tim8Ch3 {}
impl super::sig::Signal<super::sig::Tim8Ch4> for Tim8Ch4 {}
impl super::sig::SignalTim<super::sig::Tim8Ch4> for Tim8Ch4 {}


pub const TIM1_CH1: Channel<Tim1Ch1Id, Tim1Id> = Channel { periph: TIM1, index: 0, id: Tim1Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim1Ch1Id {}
pub type Tim1Ch1 = Channel<Tim1Ch1Id, Tim1Id>;

pub const TIM1_CH2: Channel<Tim1Ch2Id, Tim1Id> = Channel { periph: TIM1, index: 1, id: Tim1Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim1Ch2Id {}
pub type Tim1Ch2 = Channel<Tim1Ch2Id, Tim1Id>;

pub const TIM1_CH3: Channel<Tim1Ch3Id, Tim1Id> = Channel { periph: TIM1, index: 2, id: Tim1Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim1Ch3Id {}
pub type Tim1Ch3 = Channel<Tim1Ch3Id, Tim1Id>;

pub const TIM1_CH4: Channel<Tim1Ch4Id, Tim1Id> = Channel { periph: TIM1, index: 3, id: Tim1Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim1Ch4Id {}
pub type Tim1Ch4 = Channel<Tim1Ch4Id, Tim1Id>;

pub const TIM8_CH1: Channel<Tim8Ch1Id, Tim8Id> = Channel { periph: TIM8, index: 0, id: Tim8Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim8Ch1Id {}
pub type Tim8Ch1 = Channel<Tim8Ch1Id, Tim8Id>;

pub const TIM8_CH2: Channel<Tim8Ch2Id, Tim8Id> = Channel { periph: TIM8, index: 1, id: Tim8Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim8Ch2Id {}
pub type Tim8Ch2 = Channel<Tim8Ch2Id, Tim8Id>;

pub const TIM8_CH3: Channel<Tim8Ch3Id, Tim8Id> = Channel { periph: TIM8, index: 2, id: Tim8Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim8Ch3Id {}
pub type Tim8Ch3 = Channel<Tim8Ch3Id, Tim8Id>;

pub const TIM8_CH4: Channel<Tim8Ch4Id, Tim8Id> = Channel { periph: TIM8, index: 3, id: Tim8Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tim8Ch4Id {}
pub type Tim8Ch4 = Channel<Tim8Ch4Id, Tim8Id>;

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

impl IrqBrk<super::irq::Tim1BrkId> for Tim1 {
   fn irq_brk(&self) -> super::irq::IrqTim1Brk { super::irq::IRQ_TIM1_BRK }
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

impl IrqUp<super::irq::Tim1UpId> for Tim1 {
   fn irq_up(&self) -> super::irq::IrqTim1Up { super::irq::IRQ_TIM1_UP }
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

impl IrqTrgCom<super::irq::Tim1TrgComId> for Tim1 {
   fn irq_trg_com(&self) -> super::irq::IrqTim1TrgCom { super::irq::IRQ_TIM1_TRG_COM }
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
   fn irq_cc(&self) -> super::irq::IrqTim1Cc { super::irq::IRQ_TIM1_CC }
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

impl IrqBrk<super::irq::Tim8BrkId> for Tim8 {
   fn irq_brk(&self) -> super::irq::IrqTim8Brk { super::irq::IRQ_TIM8_BRK }
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

impl IrqUp<super::irq::Tim8UpId> for Tim8 {
   fn irq_up(&self) -> super::irq::IrqTim8Up { super::irq::IRQ_TIM8_UP }
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

impl IrqTrgCom<super::irq::Tim8TrgComId> for Tim8 {
   fn irq_trg_com(&self) -> super::irq::IrqTim8TrgCom { super::irq::IRQ_TIM8_TRG_COM }
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
   fn irq_cc(&self) -> super::irq::IrqTim8Cc { super::irq::IRQ_TIM8_CC }
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

