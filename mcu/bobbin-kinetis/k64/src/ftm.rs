pub use kinetis_common::ftm::*;

::bobbin_mcu::periph!( FTM0, Ftm0, FTM0_PERIPH, FtmPeriph, FTM0_OWNED, FTM0_REF_COUNT, 0x40038000, 0x00, 0x0a);
::bobbin_mcu::periph!( FTM1, Ftm1, FTM1_PERIPH, FtmPeriph, FTM1_OWNED, FTM1_REF_COUNT, 0x40039000, 0x01, 0x0b);
::bobbin_mcu::periph!( FTM2, Ftm2, FTM2_PERIPH, FtmPeriph, FTM2_OWNED, FTM2_REF_COUNT, 0x4003a000, 0x02, 0x0c);

::bobbin_mcu::channel!(FTM0_CH0, Ftm0Ch0, ftm0_ch0, FTM0, Ftm0, FTM0_CH0_CH, FtmCh, FTM0_PERIPH, FTM0_CH0_OWNED, FTM0_CH0_REF_COUNT, 0);
::bobbin_mcu::channel!(FTM0_CH1, Ftm0Ch1, ftm0_ch1, FTM0, Ftm0, FTM0_CH1_CH, FtmCh, FTM0_PERIPH, FTM0_CH1_OWNED, FTM0_CH1_REF_COUNT, 1);
::bobbin_mcu::channel!(FTM0_CH2, Ftm0Ch2, ftm0_ch2, FTM0, Ftm0, FTM0_CH2_CH, FtmCh, FTM0_PERIPH, FTM0_CH2_OWNED, FTM0_CH2_REF_COUNT, 2);
::bobbin_mcu::channel!(FTM0_CH3, Ftm0Ch3, ftm0_ch3, FTM0, Ftm0, FTM0_CH3_CH, FtmCh, FTM0_PERIPH, FTM0_CH3_OWNED, FTM0_CH3_REF_COUNT, 3);
::bobbin_mcu::channel!(FTM0_CH4, Ftm0Ch4, ftm0_ch4, FTM0, Ftm0, FTM0_CH4_CH, FtmCh, FTM0_PERIPH, FTM0_CH4_OWNED, FTM0_CH4_REF_COUNT, 4);
::bobbin_mcu::channel!(FTM0_CH5, Ftm0Ch5, ftm0_ch5, FTM0, Ftm0, FTM0_CH5_CH, FtmCh, FTM0_PERIPH, FTM0_CH5_OWNED, FTM0_CH5_REF_COUNT, 5);
::bobbin_mcu::channel!(FTM0_CH6, Ftm0Ch6, ftm0_ch6, FTM0, Ftm0, FTM0_CH6_CH, FtmCh, FTM0_PERIPH, FTM0_CH6_OWNED, FTM0_CH6_REF_COUNT, 6);
::bobbin_mcu::channel!(FTM0_CH7, Ftm0Ch7, ftm0_ch7, FTM0, Ftm0, FTM0_CH7_CH, FtmCh, FTM0_PERIPH, FTM0_CH7_OWNED, FTM0_CH7_REF_COUNT, 7);
::bobbin_mcu::channel!(FTM1_CH0, Ftm1Ch0, ftm1_ch0, FTM1, Ftm1, FTM1_CH0_CH, FtmCh, FTM1_PERIPH, FTM1_CH0_OWNED, FTM1_CH0_REF_COUNT, 0);
::bobbin_mcu::channel!(FTM1_CH1, Ftm1Ch1, ftm1_ch1, FTM1, Ftm1, FTM1_CH1_CH, FtmCh, FTM1_PERIPH, FTM1_CH1_OWNED, FTM1_CH1_REF_COUNT, 1);
::bobbin_mcu::channel!(FTM1_CH2, Ftm1Ch2, ftm1_ch2, FTM1, Ftm1, FTM1_CH2_CH, FtmCh, FTM1_PERIPH, FTM1_CH2_OWNED, FTM1_CH2_REF_COUNT, 2);
::bobbin_mcu::channel!(FTM1_CH3, Ftm1Ch3, ftm1_ch3, FTM1, Ftm1, FTM1_CH3_CH, FtmCh, FTM1_PERIPH, FTM1_CH3_OWNED, FTM1_CH3_REF_COUNT, 3);
::bobbin_mcu::channel!(FTM1_CH4, Ftm1Ch4, ftm1_ch4, FTM1, Ftm1, FTM1_CH4_CH, FtmCh, FTM1_PERIPH, FTM1_CH4_OWNED, FTM1_CH4_REF_COUNT, 4);
::bobbin_mcu::channel!(FTM1_CH5, Ftm1Ch5, ftm1_ch5, FTM1, Ftm1, FTM1_CH5_CH, FtmCh, FTM1_PERIPH, FTM1_CH5_OWNED, FTM1_CH5_REF_COUNT, 5);
::bobbin_mcu::channel!(FTM1_CH6, Ftm1Ch6, ftm1_ch6, FTM1, Ftm1, FTM1_CH6_CH, FtmCh, FTM1_PERIPH, FTM1_CH6_OWNED, FTM1_CH6_REF_COUNT, 6);
::bobbin_mcu::channel!(FTM1_CH7, Ftm1Ch7, ftm1_ch7, FTM1, Ftm1, FTM1_CH7_CH, FtmCh, FTM1_PERIPH, FTM1_CH7_OWNED, FTM1_CH7_REF_COUNT, 7);
::bobbin_mcu::channel!(FTM2_CH0, Ftm2Ch0, ftm2_ch0, FTM2, Ftm2, FTM2_CH0_CH, FtmCh, FTM2_PERIPH, FTM2_CH0_OWNED, FTM2_CH0_REF_COUNT, 0);
::bobbin_mcu::channel!(FTM2_CH1, Ftm2Ch1, ftm2_ch1, FTM2, Ftm2, FTM2_CH1_CH, FtmCh, FTM2_PERIPH, FTM2_CH1_OWNED, FTM2_CH1_REF_COUNT, 1);
::bobbin_mcu::channel!(FTM2_CH2, Ftm2Ch2, ftm2_ch2, FTM2, Ftm2, FTM2_CH2_CH, FtmCh, FTM2_PERIPH, FTM2_CH2_OWNED, FTM2_CH2_REF_COUNT, 2);
::bobbin_mcu::channel!(FTM2_CH3, Ftm2Ch3, ftm2_ch3, FTM2, Ftm2, FTM2_CH3_CH, FtmCh, FTM2_PERIPH, FTM2_CH3_OWNED, FTM2_CH3_REF_COUNT, 3);
::bobbin_mcu::channel!(FTM2_CH4, Ftm2Ch4, ftm2_ch4, FTM2, Ftm2, FTM2_CH4_CH, FtmCh, FTM2_PERIPH, FTM2_CH4_OWNED, FTM2_CH4_REF_COUNT, 4);
::bobbin_mcu::channel!(FTM2_CH5, Ftm2Ch5, ftm2_ch5, FTM2, Ftm2, FTM2_CH5_CH, FtmCh, FTM2_PERIPH, FTM2_CH5_OWNED, FTM2_CH5_REF_COUNT, 5);
::bobbin_mcu::channel!(FTM2_CH6, Ftm2Ch6, ftm2_ch6, FTM2, Ftm2, FTM2_CH6_CH, FtmCh, FTM2_PERIPH, FTM2_CH6_OWNED, FTM2_CH6_REF_COUNT, 6);
::bobbin_mcu::channel!(FTM2_CH7, Ftm2Ch7, ftm2_ch7, FTM2, Ftm2, FTM2_CH7_CH, FtmCh, FTM2_PERIPH, FTM2_CH7_OWNED, FTM2_CH7_REF_COUNT, 7);
// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("FTM0"), description: None }
impl ::bobbin_mcu::gate::GateEn for Ftm0 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc6().ftm0() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_ftm0(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("FTM1"), description: None }
impl ::bobbin_mcu::gate::GateEn for Ftm1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc6().ftm1() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_ftm1(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC3"), field: Some("FTM2"), description: None }
impl ::bobbin_mcu::gate::GateEn for Ftm2 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc3().ftm2() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc3(|r| r.set_ftm2(value));
        self
    }
}

