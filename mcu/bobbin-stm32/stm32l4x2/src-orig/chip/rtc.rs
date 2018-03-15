#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::rtc::*;

periph!( RTC, Rtc, _RTC, RtcPeriph, 0x40002800);




pub trait IrqRtcTamp<T> {
    fn irq_rtc_tamp(&self) -> T;
}

pub trait IrqRtcWkup<T> {
    fn irq_rtc_wkup(&self) -> T;
}

pub trait IrqRtcAlarm<T> {
    fn irq_rtc_alarm(&self) -> T;
}

impl IrqRtcTamp<super::irq::IrqRtcTampStamp> for Rtc {
    fn irq_rtc_tamp(&self) -> super::irq::IrqRtcTampStamp { super::irq::IRQ_RTC_TAMP_STAMP }
}

impl IrqRtcWkup<super::irq::IrqRtcWkup> for Rtc {
    fn irq_rtc_wkup(&self) -> super::irq::IrqRtcWkup { super::irq::IRQ_RTC_WKUP }
}

impl IrqRtcAlarm<super::irq::IrqRtcAlarm> for Rtc {
    fn irq_rtc_alarm(&self) -> super::irq::IrqRtcAlarm { super::irq::IRQ_RTC_ALARM }
}

