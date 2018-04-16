pub use ::stm32_common::lptim::*;

::bobbin_mcu::periph!( LPTIM1, Lptim1, LPTIM1_PERIPH, LptimPeriph, LPTIM1_OWNED, LPTIM1_REF_COUNT, 0x40007c00, 0x00, 0x0b);
::bobbin_mcu::periph!( LPTIM2, Lptim2, LPTIM2_PERIPH, LptimPeriph, LPTIM2_OWNED, LPTIM2_REF_COUNT, 0x40009400, 0x01, 0x0c);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR1"), field: Some("LPTIM1RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Lptim1 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr1().lptim1rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr1(|r| r.set_lptim1rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR1"), field: Some("LPTIM1EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Lptim1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr1().lptim1en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr1(|r| r.set_lptim1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR1"), field: Some("LPTIM1SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Lptim1 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1smenr1().lptim1smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr1(|r| r.set_lptim1smen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR2"), field: Some("LPTIM2RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Lptim2 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr2().lptim2rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr2(|r| r.set_lptim2rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR2"), field: Some("LPTIM2EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Lptim2 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr2().lptim2en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr2(|r| r.set_lptim2en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR2"), field: Some("LPTIM2SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Lptim2 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1smenr2().lptim2smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr2(|r| r.set_lptim2smen(value));
        self
    }
}

