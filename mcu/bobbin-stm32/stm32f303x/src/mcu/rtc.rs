#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::rtc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( RTC, Rtc, RTC_PERIPH, RtcPeriph, RTC_OWNED, RTC_REF_COUNT, 0x40002800, 0x00, 0x04);


