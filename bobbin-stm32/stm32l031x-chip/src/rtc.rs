pub use stm32_common::chip::rtc::*;

pub const RTC: Rtc = Periph(0x40002800, RtcId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RtcId {}
pub type Rtc = Periph<RtcId>;



