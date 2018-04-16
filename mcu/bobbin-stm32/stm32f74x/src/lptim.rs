pub use ::stm32_common::lptim::*;

::bobbin_mcu::periph!( LPTIM1, Lptim1, LPTIM1_PERIPH, LptimPeriph, LPTIM1_OWNED, LPTIM1_REF_COUNT, 0x40002400, 0x00, 0x2a);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("LPTIM1RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Lptim1 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr().lptim1rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_lptim1rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("LPTIM1EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Lptim1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr().lptim1en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_lptim1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("LPTIM1LPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Lptim1 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1lpenr().lptim1lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_lptim1lpen(value));
        self
    }
}

