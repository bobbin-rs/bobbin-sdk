#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dac::*;

periph!( DAC, Dac, DAC_PERIPH, DacPeriph, DAC_OWNED, DAC_REF_COUNT, 0x42004800, 0x00, 0x08);

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("DAC"), description: None }
impl ::bobbin_common::gate::GateEn for Dac {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().dac() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_dac(value));
        self
    }
}

