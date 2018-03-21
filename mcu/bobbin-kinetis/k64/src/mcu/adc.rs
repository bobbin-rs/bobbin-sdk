#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::adc::*;

periph!( ADC0, Adc0, ADC0_PERIPH, AdcPeriph, 0x4003b000, 0x00, 0x26);
periph!( ADC1, Adc1, ADC1_PERIPH, AdcPeriph, 0x400bb000, 0x01, 0x27);

channel!(ADC0_CH0, Adc0Ch0, ADC0, Adc0, ADC0_CH0_CH, AdcCh, ADC0_PERIPH, 0);
channel!(ADC0_CH1, Adc0Ch1, ADC0, Adc0, ADC0_CH1_CH, AdcCh, ADC0_PERIPH, 1);
channel!(ADC0_CH2, Adc0Ch2, ADC0, Adc0, ADC0_CH2_CH, AdcCh, ADC0_PERIPH, 2);
channel!(ADC0_CH3, Adc0Ch3, ADC0, Adc0, ADC0_CH3_CH, AdcCh, ADC0_PERIPH, 3);
channel!(ADC0_CH4, Adc0Ch4, ADC0, Adc0, ADC0_CH4_CH, AdcCh, ADC0_PERIPH, 4);
channel!(ADC0_CH5, Adc0Ch5, ADC0, Adc0, ADC0_CH5_CH, AdcCh, ADC0_PERIPH, 5);
channel!(ADC0_CH6, Adc0Ch6, ADC0, Adc0, ADC0_CH6_CH, AdcCh, ADC0_PERIPH, 6);
channel!(ADC0_CH7, Adc0Ch7, ADC0, Adc0, ADC0_CH7_CH, AdcCh, ADC0_PERIPH, 7);
channel!(ADC0_CH8, Adc0Ch8, ADC0, Adc0, ADC0_CH8_CH, AdcCh, ADC0_PERIPH, 8);
channel!(ADC0_CH9, Adc0Ch9, ADC0, Adc0, ADC0_CH9_CH, AdcCh, ADC0_PERIPH, 9);
channel!(ADC0_CH10, Adc0Ch10, ADC0, Adc0, ADC0_CH10_CH, AdcCh, ADC0_PERIPH, 10);
channel!(ADC0_CH11, Adc0Ch11, ADC0, Adc0, ADC0_CH11_CH, AdcCh, ADC0_PERIPH, 11);
channel!(ADC0_CH12, Adc0Ch12, ADC0, Adc0, ADC0_CH12_CH, AdcCh, ADC0_PERIPH, 12);
channel!(ADC0_CH13, Adc0Ch13, ADC0, Adc0, ADC0_CH13_CH, AdcCh, ADC0_PERIPH, 13);
channel!(ADC0_CH14, Adc0Ch14, ADC0, Adc0, ADC0_CH14_CH, AdcCh, ADC0_PERIPH, 14);
channel!(ADC0_CH15, Adc0Ch15, ADC0, Adc0, ADC0_CH15_CH, AdcCh, ADC0_PERIPH, 15);
channel!(ADC0_CH16, Adc0Ch16, ADC0, Adc0, ADC0_CH16_CH, AdcCh, ADC0_PERIPH, 16);
channel!(ADC0_CH17, Adc0Ch17, ADC0, Adc0, ADC0_CH17_CH, AdcCh, ADC0_PERIPH, 17);
channel!(ADC0_CH18, Adc0Ch18, ADC0, Adc0, ADC0_CH18_CH, AdcCh, ADC0_PERIPH, 18);
channel!(ADC0_CH19, Adc0Ch19, ADC0, Adc0, ADC0_CH19_CH, AdcCh, ADC0_PERIPH, 19);
channel!(ADC0_CH20, Adc0Ch20, ADC0, Adc0, ADC0_CH20_CH, AdcCh, ADC0_PERIPH, 20);
channel!(ADC0_CH21, Adc0Ch21, ADC0, Adc0, ADC0_CH21_CH, AdcCh, ADC0_PERIPH, 21);
channel!(ADC0_CH22, Adc0Ch22, ADC0, Adc0, ADC0_CH22_CH, AdcCh, ADC0_PERIPH, 22);
channel!(ADC0_CH23, Adc0Ch23, ADC0, Adc0, ADC0_CH23_CH, AdcCh, ADC0_PERIPH, 23);
channel!(ADC0_TEMP, Adc0Temp, ADC0, Adc0, ADC0_TEMP_CH, AdcCh, ADC0_PERIPH, 26);
channel!(ADC0_BANDGAP, Adc0Bandgap, ADC0, Adc0, ADC0_BANDGAP_CH, AdcCh, ADC0_PERIPH, 27);
channel!(ADC0_REFSH, Adc0Refsh, ADC0, Adc0, ADC0_REFSH_CH, AdcCh, ADC0_PERIPH, 29);
channel!(ADC0_REFSL, Adc0Refsl, ADC0, Adc0, ADC0_REFSL_CH, AdcCh, ADC0_PERIPH, 30);
channel!(ADC1_CH0, Adc1Ch0, ADC1, Adc1, ADC1_CH0_CH, AdcCh, ADC1_PERIPH, 0);
channel!(ADC1_CH1, Adc1Ch1, ADC1, Adc1, ADC1_CH1_CH, AdcCh, ADC1_PERIPH, 1);
channel!(ADC1_CH2, Adc1Ch2, ADC1, Adc1, ADC1_CH2_CH, AdcCh, ADC1_PERIPH, 2);
channel!(ADC1_CH3, Adc1Ch3, ADC1, Adc1, ADC1_CH3_CH, AdcCh, ADC1_PERIPH, 3);
channel!(ADC1_CH4, Adc1Ch4, ADC1, Adc1, ADC1_CH4_CH, AdcCh, ADC1_PERIPH, 4);
channel!(ADC1_CH5, Adc1Ch5, ADC1, Adc1, ADC1_CH5_CH, AdcCh, ADC1_PERIPH, 5);
channel!(ADC1_CH6, Adc1Ch6, ADC1, Adc1, ADC1_CH6_CH, AdcCh, ADC1_PERIPH, 6);
channel!(ADC1_CH7, Adc1Ch7, ADC1, Adc1, ADC1_CH7_CH, AdcCh, ADC1_PERIPH, 7);
channel!(ADC1_CH8, Adc1Ch8, ADC1, Adc1, ADC1_CH8_CH, AdcCh, ADC1_PERIPH, 8);
channel!(ADC1_CH9, Adc1Ch9, ADC1, Adc1, ADC1_CH9_CH, AdcCh, ADC1_PERIPH, 9);
channel!(ADC1_CH10, Adc1Ch10, ADC1, Adc1, ADC1_CH10_CH, AdcCh, ADC1_PERIPH, 10);
channel!(ADC1_CH11, Adc1Ch11, ADC1, Adc1, ADC1_CH11_CH, AdcCh, ADC1_PERIPH, 11);
channel!(ADC1_CH12, Adc1Ch12, ADC1, Adc1, ADC1_CH12_CH, AdcCh, ADC1_PERIPH, 12);
channel!(ADC1_CH13, Adc1Ch13, ADC1, Adc1, ADC1_CH13_CH, AdcCh, ADC1_PERIPH, 13);
channel!(ADC1_CH14, Adc1Ch14, ADC1, Adc1, ADC1_CH14_CH, AdcCh, ADC1_PERIPH, 14);
channel!(ADC1_CH15, Adc1Ch15, ADC1, Adc1, ADC1_CH15_CH, AdcCh, ADC1_PERIPH, 15);
channel!(ADC1_CH16, Adc1Ch16, ADC1, Adc1, ADC1_CH16_CH, AdcCh, ADC1_PERIPH, 16);
channel!(ADC1_CH17, Adc1Ch17, ADC1, Adc1, ADC1_CH17_CH, AdcCh, ADC1_PERIPH, 17);
channel!(ADC1_CH18, Adc1Ch18, ADC1, Adc1, ADC1_CH18_CH, AdcCh, ADC1_PERIPH, 18);
channel!(ADC1_CH19, Adc1Ch19, ADC1, Adc1, ADC1_CH19_CH, AdcCh, ADC1_PERIPH, 19);
channel!(ADC1_CH20, Adc1Ch20, ADC1, Adc1, ADC1_CH20_CH, AdcCh, ADC1_PERIPH, 20);
channel!(ADC1_CH21, Adc1Ch21, ADC1, Adc1, ADC1_CH21_CH, AdcCh, ADC1_PERIPH, 21);
channel!(ADC1_CH22, Adc1Ch22, ADC1, Adc1, ADC1_CH22_CH, AdcCh, ADC1_PERIPH, 22);
channel!(ADC1_CH23, Adc1Ch23, ADC1, Adc1, ADC1_CH23_CH, AdcCh, ADC1_PERIPH, 23);
channel!(ADC1_TEMP, Adc1Temp, ADC1, Adc1, ADC1_TEMP_CH, AdcCh, ADC1_PERIPH, 26);
channel!(ADC1_BANDGAP, Adc1Bandgap, ADC1, Adc1, ADC1_BANDGAP_CH, AdcCh, ADC1_PERIPH, 27);
channel!(ADC1_REFSH, Adc1Refsh, ADC1, Adc1, ADC1_REFSH_CH, AdcCh, ADC1_PERIPH, 29);
channel!(ADC1_REFSL, Adc1Refsl, ADC1, Adc1, ADC1_REFSL_CH, AdcCh, ADC1_PERIPH, 30);
// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("ADC0"), description: None }
impl ::bobbin_common::gate::GateEn for Adc0 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc6().adc0() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_adc0(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC3"), field: Some("ADC1"), description: None }
impl ::bobbin_common::gate::GateEn for Adc1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc3().adc1() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc3(|r| r.set_adc1(value));
        self
    }
}

