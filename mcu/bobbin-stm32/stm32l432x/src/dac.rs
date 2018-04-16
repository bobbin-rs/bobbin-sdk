pub use ::stm32_common::dac::*;

::bobbin_mcu::periph!( DAC1, Dac1, DAC1_PERIPH, DacPeriph, DAC1_OWNED, DAC1_REF_COUNT, 0x40007400, 0x00, 0x05);

::bobbin_mcu::channel!(DAC1_CH1, Dac1Ch1, dac1_ch1, DAC1, Dac1, DAC1_CH1_CH, DacCh, DAC1_PERIPH, DAC1_CH1_OWNED, DAC1_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(DAC1_CH2, Dac1Ch2, dac1_ch2, DAC1, Dac1, DAC1_CH2_CH, DacCh, DAC1_PERIPH, DAC1_CH2_OWNED, DAC1_CH2_REF_COUNT, 1);
// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR1"), field: Some("DAC1RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Dac1 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr1().dac1rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr1(|r| r.set_dac1rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR1"), field: Some("DAC1EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Dac1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr1().dac1en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr1(|r| r.set_dac1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR1"), field: Some("DAC1SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Dac1 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1smenr1().dac1smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr1(|r| r.set_dac1smen(value));
        self
    }
}

