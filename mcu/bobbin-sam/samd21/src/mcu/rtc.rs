#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::rtc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( RTC, Rtc, RTC_PERIPH, RtcPeriph, 0x40001400, 0x05);


// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBAMASK"), field: Some("RTC"), description: None }
impl ::bobbin_common::gate::GateEn for Rtc {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbamask().rtc() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbamask(|r| r.set_rtc(value));
        self
    }
}

