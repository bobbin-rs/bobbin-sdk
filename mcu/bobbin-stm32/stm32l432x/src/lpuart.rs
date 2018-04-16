pub use ::stm32_common::lpuart::*;

::bobbin_mcu::periph!( LPUART1, Lpuart1, LPUART1_PERIPH, LpuartPeriph, LPUART1_OWNED, LPUART1_REF_COUNT, 0x40008000, 0x00, 0x2a);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR2"), field: Some("LPUART1RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Lpuart1 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr2().lpuart1rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr2(|r| r.set_lpuart1rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR2"), field: Some("LPUART1EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Lpuart1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr2().lpuart1en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr2(|r| r.set_lpuart1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR2"), field: Some("LPUART1SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Lpuart1 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1smenr2().lpuart1smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr2(|r| r.set_lpuart1smen(value));
        self
    }
}

