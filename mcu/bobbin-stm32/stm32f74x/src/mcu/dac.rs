#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dac::*;

periph!( DAC, Dac, DAC_PERIPH, DacPeriph, 0x40007400, 0x00, 0x12);

channel!(DAC_CH1, DacCh1, DAC, Dac, DAC_CH1_CH, DacCh, DAC_PERIPH, 0);
channel!(DAC_CH2, DacCh2, DAC, Dac, DAC_CH2_CH, DacCh, DAC_PERIPH, 1);
// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("DACRST"), description: None }
impl ::bobbin_common::gate::GateRst for Dac {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().dacrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_dacrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("DACEN"), description: None }
impl ::bobbin_common::gate::GateEn for Dac {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().dacen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_dacen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("DACLPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Dac {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1lpenr().daclpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_daclpen(value));
        self
    }
}

