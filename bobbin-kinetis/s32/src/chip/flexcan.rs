#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::flexcan::*;

pub const CAN0: Can0 = Periph(0x40024000, Can0Id {});
pub const CAN1: Can1 = Periph(0x40025000, Can1Id {});
pub const CAN2: Can2 = Periph(0x4002b000, Can2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Can0Id {}
pub type Can0 = Periph<Can0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Can1Id {}
pub type Can1 = Periph<Can1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Can2Id {}
pub type Can2 = Periph<Can2Id>;





pub trait IrqCan<T> {
   fn irq_can(&self) -> super::irq::Irq<T>;
}

pub trait RegisterCanHandler {
   fn register_can_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCan>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleCan {
   fn handle_can(&self);
}

pub trait IrqCanError<T> {
   fn irq_can_error(&self) -> super::irq::Irq<T>;
}

pub trait RegisterCanErrorHandler {
   fn register_can_error_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCanError>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleCanError {
   fn handle_can_error(&self);
}

pub trait IrqCanWakeUp<T> {
   fn irq_can_wake_up(&self) -> super::irq::Irq<T>;
}

pub trait RegisterCanWakeUpHandler {
   fn register_can_wake_up_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCanWakeUp>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleCanWakeUp {
   fn handle_can_wake_up(&self);
}

pub trait IrqCan015<T> {
   fn irq_can_0_15(&self) -> super::irq::Irq<T>;
}

pub trait RegisterCan015Handler {
   fn register_can_0_15_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCan015>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleCan015 {
   fn handle_can_0_15(&self);
}

pub trait IrqCan1631<T> {
   fn irq_can_16_31(&self) -> super::irq::Irq<T>;
}

pub trait RegisterCan1631Handler {
   fn register_can_16_31_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCan1631>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleCan1631 {
   fn handle_can_16_31(&self);
}

impl IrqCan<super::irq::Can0OredId> for Can0 {
   fn irq_can(&self) -> super::irq::IrqCan0Ored { super::irq::IRQ_CAN0_ORED }
}

impl RegisterCanHandler for Can0 {
   fn register_can_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCan>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCan>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_can() }
       }
       super::irq::set_handler(78, Some(wrapper::<F>));
       super::irq::IrqGuard::new(78)
   }
}

impl IrqCanError<super::irq::Can0ErrorId> for Can0 {
   fn irq_can_error(&self) -> super::irq::IrqCan0Error { super::irq::IRQ_CAN0_ERROR }
}

impl RegisterCanErrorHandler for Can0 {
   fn register_can_error_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCanError>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCanError>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_can_error() }
       }
       super::irq::set_handler(79, Some(wrapper::<F>));
       super::irq::IrqGuard::new(79)
   }
}

impl IrqCanWakeUp<super::irq::Can0WakeUpId> for Can0 {
   fn irq_can_wake_up(&self) -> super::irq::IrqCan0WakeUp { super::irq::IRQ_CAN0_WAKE_UP }
}

impl RegisterCanWakeUpHandler for Can0 {
   fn register_can_wake_up_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCanWakeUp>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCanWakeUp>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_can_wake_up() }
       }
       super::irq::set_handler(80, Some(wrapper::<F>));
       super::irq::IrqGuard::new(80)
   }
}

impl IrqCan015<super::irq::Can0Ored015MbId> for Can0 {
   fn irq_can_0_15(&self) -> super::irq::IrqCan0Ored015Mb { super::irq::IRQ_CAN0_ORED_0_15_MB }
}

impl RegisterCan015Handler for Can0 {
   fn register_can_0_15_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCan015>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCan015>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_can_0_15() }
       }
       super::irq::set_handler(81, Some(wrapper::<F>));
       super::irq::IrqGuard::new(81)
   }
}

impl IrqCan1631<super::irq::Can0Ored1631MbId> for Can0 {
   fn irq_can_16_31(&self) -> super::irq::IrqCan0Ored1631Mb { super::irq::IRQ_CAN0_ORED_16_31_MB }
}

impl RegisterCan1631Handler for Can0 {
   fn register_can_16_31_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCan1631>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCan1631>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_can_16_31() }
       }
       super::irq::set_handler(82, Some(wrapper::<F>));
       super::irq::IrqGuard::new(82)
   }
}

impl IrqCan<super::irq::Can1OredId> for Can1 {
   fn irq_can(&self) -> super::irq::IrqCan1Ored { super::irq::IRQ_CAN1_ORED }
}

impl RegisterCanHandler for Can1 {
   fn register_can_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCan>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCan>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_can() }
       }
       super::irq::set_handler(85, Some(wrapper::<F>));
       super::irq::IrqGuard::new(85)
   }
}

impl IrqCanError<super::irq::Can1ErrorId> for Can1 {
   fn irq_can_error(&self) -> super::irq::IrqCan1Error { super::irq::IRQ_CAN1_ERROR }
}

impl RegisterCanErrorHandler for Can1 {
   fn register_can_error_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCanError>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCanError>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_can_error() }
       }
       super::irq::set_handler(86, Some(wrapper::<F>));
       super::irq::IrqGuard::new(86)
   }
}

impl IrqCan015<super::irq::Can1Ored015MbId> for Can1 {
   fn irq_can_0_15(&self) -> super::irq::IrqCan1Ored015Mb { super::irq::IRQ_CAN1_ORED_0_15_MB }
}

impl RegisterCan015Handler for Can1 {
   fn register_can_0_15_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCan015>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCan015>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_can_0_15() }
       }
       super::irq::set_handler(88, Some(wrapper::<F>));
       super::irq::IrqGuard::new(88)
   }
}

impl IrqCan<super::irq::Can2OredId> for Can2 {
   fn irq_can(&self) -> super::irq::IrqCan2Ored { super::irq::IRQ_CAN2_ORED }
}

impl RegisterCanHandler for Can2 {
   fn register_can_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCan>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCan>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_can() }
       }
       super::irq::set_handler(92, Some(wrapper::<F>));
       super::irq::IrqGuard::new(92)
   }
}

impl IrqCanError<super::irq::Can2ErrorId> for Can2 {
   fn irq_can_error(&self) -> super::irq::IrqCan2Error { super::irq::IRQ_CAN2_ERROR }
}

impl RegisterCanErrorHandler for Can2 {
   fn register_can_error_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCanError>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCanError>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_can_error() }
       }
       super::irq::set_handler(93, Some(wrapper::<F>));
       super::irq::IrqGuard::new(93)
   }
}

impl IrqCan015<super::irq::Can2Ored015MbId> for Can2 {
   fn irq_can_0_15(&self) -> super::irq::IrqCan2Ored015Mb { super::irq::IRQ_CAN2_ORED_0_15_MB }
}

impl RegisterCan015Handler for Can2 {
   fn register_can_0_15_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleCan015>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleCan015>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_can_0_15() }
       }
       super::irq::set_handler(95, Some(wrapper::<F>));
       super::irq::IrqGuard::new(95)
   }
}

