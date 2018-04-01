#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::rtc::*;

periph!( RTC, Rtc, RTC_PERIPH, RtcPeriph, 0x40002800, 0x00, 0x0a);

