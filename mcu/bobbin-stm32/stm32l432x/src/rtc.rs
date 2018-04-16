pub use ::stm32_common::rtc::*;

::bobbin_mcu::periph!( RTC, Rtc, RTC_PERIPH, RtcPeriph, RTC_OWNED, RTC_REF_COUNT, 0x40002800, 0x00, 0x0a);

