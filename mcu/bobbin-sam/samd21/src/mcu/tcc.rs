#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::tcc::*;

periph!( TCC0, Tcc0, TCC0_PERIPH, TccPeriph, TCC0_OWNED, TCC0_REF_COUNT, 0x42002000, 0x00, 0x09);
periph!( TCC1, Tcc1, TCC1_PERIPH, TccPeriph, TCC1_OWNED, TCC1_REF_COUNT, 0x42002400, 0x01, 0x0a);
periph!( TCC2, Tcc2, TCC2_PERIPH, TccPeriph, TCC2_OWNED, TCC2_REF_COUNT, 0x42002800, 0x02, 0x0b);

channel!(TCC0_CH0, Tcc0Ch0, tcc0_ch0, TCC0, Tcc0, TCC0_CH0_CH, TccCh, TCC0_PERIPH, TCC0_CH0_OWNED, TCC0_CH0_REF_COUNT, 0);
channel!(TCC0_CH1, Tcc0Ch1, tcc0_ch1, TCC0, Tcc0, TCC0_CH1_CH, TccCh, TCC0_PERIPH, TCC0_CH1_OWNED, TCC0_CH1_REF_COUNT, 1);
channel!(TCC0_CH2, Tcc0Ch2, tcc0_ch2, TCC0, Tcc0, TCC0_CH2_CH, TccCh, TCC0_PERIPH, TCC0_CH2_OWNED, TCC0_CH2_REF_COUNT, 2);
channel!(TCC0_CH3, Tcc0Ch3, tcc0_ch3, TCC0, Tcc0, TCC0_CH3_CH, TccCh, TCC0_PERIPH, TCC0_CH3_OWNED, TCC0_CH3_REF_COUNT, 3);
channel!(TCC1_CH0, Tcc1Ch0, tcc1_ch0, TCC1, Tcc1, TCC1_CH0_CH, TccCh, TCC1_PERIPH, TCC1_CH0_OWNED, TCC1_CH0_REF_COUNT, 0);
channel!(TCC1_CH1, Tcc1Ch1, tcc1_ch1, TCC1, Tcc1, TCC1_CH1_CH, TccCh, TCC1_PERIPH, TCC1_CH1_OWNED, TCC1_CH1_REF_COUNT, 1);
channel!(TCC2_CH0, Tcc2Ch0, tcc2_ch0, TCC2, Tcc2, TCC2_CH0_CH, TccCh, TCC2_PERIPH, TCC2_CH0_OWNED, TCC2_CH0_REF_COUNT, 0);
channel!(TCC2_CH1, Tcc2Ch1, tcc2_ch1, TCC2, Tcc2, TCC2_CH1_CH, TccCh, TCC2_PERIPH, TCC2_CH1_OWNED, TCC2_CH1_REF_COUNT, 1);
// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("TCC0"), description: None }
impl ::bobbin_common::gate::GateEn for Tcc0 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().tcc0() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_tcc0(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("TCC1"), description: None }
impl ::bobbin_common::gate::GateEn for Tcc1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().tcc1() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_tcc1(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("TCC2"), description: None }
impl ::bobbin_common::gate::GateEn for Tcc2 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().tcc2() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_tcc2(value));
        self
    }
}

