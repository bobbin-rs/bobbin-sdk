#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::tc::*;

periph!( TC3, Tc3, TC3_PERIPH, TcPeriph, 0x42002c00, 0x00, 0x0b);
periph!( TC4, Tc4, TC4_PERIPH, TcPeriph, 0x42003000, 0x01, 0x0c);
periph!( TC5, Tc5, TC5_PERIPH, TcPeriph, 0x42003400, 0x02, 0x0d);

channel!(TC3_CH0, Tc3Ch0, TC3, Tc3, TC3_CH0_CH, TcCh, TC3_PERIPH, 0);
channel!(TC3_CH1, Tc3Ch1, TC3, Tc3, TC3_CH1_CH, TcCh, TC3_PERIPH, 1);
channel!(TC4_CH0, Tc4Ch0, TC4, Tc4, TC4_CH0_CH, TcCh, TC4_PERIPH, 0);
channel!(TC4_CH1, Tc4Ch1, TC4, Tc4, TC4_CH1_CH, TcCh, TC4_PERIPH, 1);
channel!(TC5_CH0, Tc5Ch0, TC5, Tc5, TC5_CH0_CH, TcCh, TC5_PERIPH, 0);
channel!(TC5_CH1, Tc5Ch1, TC5, Tc5, TC5_CH1_CH, TcCh, TC5_PERIPH, 1);
// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("TC3"), description: None }
impl ::bobbin_common::gate::GateEn for Tc3 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().tc3() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_tc3(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("TC4"), description: None }
impl ::bobbin_common::gate::GateEn for Tc4 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().tc4() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_tc4(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("TC5"), description: None }
impl ::bobbin_common::gate::GateEn for Tc5 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().tc5() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_tc5(value));
        self
    }
}

