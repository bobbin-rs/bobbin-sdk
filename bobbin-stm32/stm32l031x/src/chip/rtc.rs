#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::rtc::*;

periph!( RTC, Rtc, _RTC, RtcPeriph, 0x40002800);



