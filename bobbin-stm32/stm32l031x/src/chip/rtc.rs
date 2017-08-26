#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::rtc::*;

periph!(RtcPeriph, RTC, Rtc, 0x40002800);



