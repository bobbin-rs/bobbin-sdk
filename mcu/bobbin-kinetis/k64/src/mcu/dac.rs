#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dac::*;

periph!( DAC0, Dac0, DAC0_PERIPH, DacPeriph, 0x400cc000, 0x00, 0x1c);
periph!( DAC1, Dac1, DAC1_PERIPH, DacPeriph, 0x400cd000, 0x01, 0x1d);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC2"), field: Some("DAC0"), description: None }
impl ::bobbin_common::gate::GateEn for Dac0 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc2().dac0() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc2(|r| r.set_dac0(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC2"), field: Some("DAC1"), description: None }
impl ::bobbin_common::gate::GateEn for Dac1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc2().dac1() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc2(|r| r.set_dac1(value));
        self
    }
}

