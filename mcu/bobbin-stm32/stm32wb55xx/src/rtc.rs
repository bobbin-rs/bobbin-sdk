pub use ::stm32_common::rtc::*;

::bobbin_mcu::periph!( RTC, Rtc, RTC_PERIPH, RtcPeriph, RTC_OWNED, RTC_REF_COUNT, 0x40002800, 0x00, 0x19);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("BDCR"), field: Some("BDRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Rtc {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.bdcr().bdrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_bdcr(|r| r.set_bdrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("BDCR"), field: Some("RTCEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Rtc {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.bdcr().rtcen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_bdcr(|r| r.set_rtcen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR1"), field: Some("RTCAPBSMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Rtc {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1smenr1().rtcapbsmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr1(|r| r.set_rtcapbsmen(value));
        self
    }
}

