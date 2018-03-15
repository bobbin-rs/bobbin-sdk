#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::rtc::*;

periph!( RTC, Rtc, RTC_PERIPH, RtcPeriph, 0x40002800, 0x09);

