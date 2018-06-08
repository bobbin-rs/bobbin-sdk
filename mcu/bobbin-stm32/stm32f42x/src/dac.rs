pub use ::stm32_common::dac::*;

::bobbin_mcu::periph!( DAC, Dac, DAC_PERIPH, DacPeriph, DAC_OWNED, DAC_REF_COUNT, 0x40007400, 0x00, 0x0e);

::bobbin_mcu::channel!(DAC_CH1, DacCh1, dac_ch1, DAC, Dac, DAC_CH1_CH, DacCh, DAC_PERIPH, DAC_CH1_OWNED, DAC_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(DAC_CH2, DacCh2, dac_ch2, DAC, Dac, DAC_CH2_CH, DacCh, DAC_PERIPH, DAC_CH2_OWNED, DAC_CH2_REF_COUNT, 1);
// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("DACRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Dac {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr().dacrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_dacrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("DACEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Dac {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr().dacen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_dacen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("DACLPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Dac {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1lpenr().daclpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_daclpen(value));
        self
    }
}

