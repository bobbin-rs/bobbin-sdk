#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::ftm::*;

periph!( FTM0, Ftm0, FTM0_PERIPH, FtmPeriph, 0x40038000, 0x0a);
periph!( FTM1, Ftm1, FTM1_PERIPH, FtmPeriph, 0x40039000, 0x0b);
periph!( FTM2, Ftm2, FTM2_PERIPH, FtmPeriph, 0x4003a000, 0x0c);

channel!(FTM0_CH0, Ftm0Ch0, FTM0, Ftm0, FTM0_CH0_CH, FtmCh, FTM0_PERIPH, 0);
channel!(FTM0_CH1, Ftm0Ch1, FTM0, Ftm0, FTM0_CH1_CH, FtmCh, FTM0_PERIPH, 1);
channel!(FTM0_CH2, Ftm0Ch2, FTM0, Ftm0, FTM0_CH2_CH, FtmCh, FTM0_PERIPH, 2);
channel!(FTM0_CH3, Ftm0Ch3, FTM0, Ftm0, FTM0_CH3_CH, FtmCh, FTM0_PERIPH, 3);
channel!(FTM0_CH4, Ftm0Ch4, FTM0, Ftm0, FTM0_CH4_CH, FtmCh, FTM0_PERIPH, 4);
channel!(FTM0_CH5, Ftm0Ch5, FTM0, Ftm0, FTM0_CH5_CH, FtmCh, FTM0_PERIPH, 5);
channel!(FTM0_CH6, Ftm0Ch6, FTM0, Ftm0, FTM0_CH6_CH, FtmCh, FTM0_PERIPH, 6);
channel!(FTM0_CH7, Ftm0Ch7, FTM0, Ftm0, FTM0_CH7_CH, FtmCh, FTM0_PERIPH, 7);
channel!(FTM1_CH0, Ftm1Ch0, FTM1, Ftm1, FTM1_CH0_CH, FtmCh, FTM1_PERIPH, 0);
channel!(FTM1_CH1, Ftm1Ch1, FTM1, Ftm1, FTM1_CH1_CH, FtmCh, FTM1_PERIPH, 1);
channel!(FTM1_CH2, Ftm1Ch2, FTM1, Ftm1, FTM1_CH2_CH, FtmCh, FTM1_PERIPH, 2);
channel!(FTM1_CH3, Ftm1Ch3, FTM1, Ftm1, FTM1_CH3_CH, FtmCh, FTM1_PERIPH, 3);
channel!(FTM1_CH4, Ftm1Ch4, FTM1, Ftm1, FTM1_CH4_CH, FtmCh, FTM1_PERIPH, 4);
channel!(FTM1_CH5, Ftm1Ch5, FTM1, Ftm1, FTM1_CH5_CH, FtmCh, FTM1_PERIPH, 5);
channel!(FTM1_CH6, Ftm1Ch6, FTM1, Ftm1, FTM1_CH6_CH, FtmCh, FTM1_PERIPH, 6);
channel!(FTM1_CH7, Ftm1Ch7, FTM1, Ftm1, FTM1_CH7_CH, FtmCh, FTM1_PERIPH, 7);
channel!(FTM2_CH0, Ftm2Ch0, FTM2, Ftm2, FTM2_CH0_CH, FtmCh, FTM2_PERIPH, 0);
channel!(FTM2_CH1, Ftm2Ch1, FTM2, Ftm2, FTM2_CH1_CH, FtmCh, FTM2_PERIPH, 1);
channel!(FTM2_CH2, Ftm2Ch2, FTM2, Ftm2, FTM2_CH2_CH, FtmCh, FTM2_PERIPH, 2);
channel!(FTM2_CH3, Ftm2Ch3, FTM2, Ftm2, FTM2_CH3_CH, FtmCh, FTM2_PERIPH, 3);
channel!(FTM2_CH4, Ftm2Ch4, FTM2, Ftm2, FTM2_CH4_CH, FtmCh, FTM2_PERIPH, 4);
channel!(FTM2_CH5, Ftm2Ch5, FTM2, Ftm2, FTM2_CH5_CH, FtmCh, FTM2_PERIPH, 5);
channel!(FTM2_CH6, Ftm2Ch6, FTM2, Ftm2, FTM2_CH6_CH, FtmCh, FTM2_PERIPH, 6);
channel!(FTM2_CH7, Ftm2Ch7, FTM2, Ftm2, FTM2_CH7_CH, FtmCh, FTM2_PERIPH, 7);
// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("FTM0"), description: None }
impl ::bobbin_common::gate::GateEn for Ftm0 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc6().ftm0() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_ftm0(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("FTM1"), description: None }
impl ::bobbin_common::gate::GateEn for Ftm1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc6().ftm1() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_ftm1(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC3"), field: Some("FTM2"), description: None }
impl ::bobbin_common::gate::GateEn for Ftm2 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc3().ftm2() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc3(|r| r.set_ftm2(value));
        self
    }
}

