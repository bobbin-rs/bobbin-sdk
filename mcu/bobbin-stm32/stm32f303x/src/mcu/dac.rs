#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dac::*;

periph!( DAC, Dac, DAC_PERIPH, DacPeriph, DAC_OWNED, DAC_REF_COUNT, 0x40007400, 0x00, 0x22);

channel!(DAC_CH1, DacCh1, dac_ch1, DAC, Dac, DAC_CH1_CH, DacCh, DAC_PERIPH, DAC_CH1_OWNED, DAC_CH1_REF_COUNT, 0);
channel!(DAC_CH2, DacCh2, dac_ch2, DAC, Dac, DAC_CH2_CH, DacCh, DAC_PERIPH, DAC_CH2_OWNED, DAC_CH2_REF_COUNT, 1);
// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("DAC1RST"), description: None }
impl ::bobbin_common::gate::GateRst for Dac {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().dac1rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_dac1rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("DAC1EN"), description: None }
impl ::bobbin_common::gate::GateEn for Dac {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().dac1en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_dac1en(value));
        self
    }
}

