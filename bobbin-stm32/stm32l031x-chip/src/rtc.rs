pub use stm32_common::chip::rtc::*;

pub const RTC: Rtc = Periph(0x40002800, RtcId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RtcId {}
pub type Rtc = Periph<RtcId>;



pub trait IrqRtc<T> {
   fn irq_rtc(&self) -> super::irq::Irq<T>;
}

pub trait RegisterRtcHandler {
   fn register_rtc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleRtc>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleRtc {
   fn handle_rtc(&self);
}

impl IrqRtc<super::irq::RtcId> for Rtc {
   fn irq_rtc(&self) -> super::irq::IrqRtc { super::irq::IRQ_RTC }
}

impl RegisterRtcHandler for Rtc {
   fn register_rtc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleRtc>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleRtc>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_rtc() }
       }
       super::irq::set_handler(2, Some(wrapper::<F>));
       super::irq::IrqGuard::new(2)
   }
}

