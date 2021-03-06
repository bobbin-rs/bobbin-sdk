pub use ::stm32_common::tim_bas::*;

::bobbin_mcu::periph!( TIM6, Tim6, TIM6_PERIPH, TimBasPeriph, TIM6_OWNED, TIM6_REF_COUNT, 0x40001000, 0x00, 0x15);
::bobbin_mcu::periph!( TIM7, Tim7, TIM7_PERIPH, TimBasPeriph, TIM7_OWNED, TIM7_REF_COUNT, 0x40001400, 0x01, 0x16);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR1"), field: Some("TIM6RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Tim6 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr1().tim6rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr1(|r| r.set_tim6rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR1"), field: Some("TIM6EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Tim6 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr1().tim6en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr1(|r| r.set_tim6en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR1"), field: Some("TIM6SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Tim6 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1smenr1().tim6smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr1(|r| r.set_tim6smen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR1"), field: Some("TIM7RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Tim7 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr1().tim7rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr1(|r| r.set_tim7rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR1"), field: Some("TIM7EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Tim7 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr1().tim7en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr1(|r| r.set_tim7en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR1"), field: Some("TIM7SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Tim7 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1smenr1().tim7smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr1(|r| r.set_tim7smen(value));
        self
    }
}

