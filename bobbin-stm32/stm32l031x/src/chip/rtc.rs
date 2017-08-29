#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::rtc::*;

periph!( RTC, Rtc, _RTC, RtcPeriph, 0x40002800);




pub trait IrqRtc<T> {
   fn irq_rtc(&self) -> T;
}

impl IrqRtc<super::irq::IrqRtc> for Rtc {
   fn irq_rtc(&self) -> super::irq::IrqRtc { super::irq::IRQ_RTC }
}

