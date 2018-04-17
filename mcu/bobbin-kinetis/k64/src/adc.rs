pub use kinetis_common::adc::*;

::bobbin_mcu::periph!( ADC0, Adc0, ADC0_PERIPH, AdcPeriph, ADC0_OWNED, ADC0_REF_COUNT, 0x4003b000, 0x00, 0x28);
::bobbin_mcu::periph!( ADC1, Adc1, ADC1_PERIPH, AdcPeriph, ADC1_OWNED, ADC1_REF_COUNT, 0x400bb000, 0x01, 0x29);

::bobbin_mcu::channel!(ADC0_CH0, Adc0Ch0, adc0_ch0, ADC0, Adc0, ADC0_CH0_CH, AdcCh, ADC0_PERIPH, ADC0_CH0_OWNED, ADC0_CH0_REF_COUNT, 0);
::bobbin_mcu::channel!(ADC0_CH1, Adc0Ch1, adc0_ch1, ADC0, Adc0, ADC0_CH1_CH, AdcCh, ADC0_PERIPH, ADC0_CH1_OWNED, ADC0_CH1_REF_COUNT, 1);
::bobbin_mcu::channel!(ADC0_CH2, Adc0Ch2, adc0_ch2, ADC0, Adc0, ADC0_CH2_CH, AdcCh, ADC0_PERIPH, ADC0_CH2_OWNED, ADC0_CH2_REF_COUNT, 2);
::bobbin_mcu::channel!(ADC0_CH3, Adc0Ch3, adc0_ch3, ADC0, Adc0, ADC0_CH3_CH, AdcCh, ADC0_PERIPH, ADC0_CH3_OWNED, ADC0_CH3_REF_COUNT, 3);
::bobbin_mcu::channel!(ADC0_CH4, Adc0Ch4, adc0_ch4, ADC0, Adc0, ADC0_CH4_CH, AdcCh, ADC0_PERIPH, ADC0_CH4_OWNED, ADC0_CH4_REF_COUNT, 4);
::bobbin_mcu::channel!(ADC0_CH5, Adc0Ch5, adc0_ch5, ADC0, Adc0, ADC0_CH5_CH, AdcCh, ADC0_PERIPH, ADC0_CH5_OWNED, ADC0_CH5_REF_COUNT, 5);
::bobbin_mcu::channel!(ADC0_CH6, Adc0Ch6, adc0_ch6, ADC0, Adc0, ADC0_CH6_CH, AdcCh, ADC0_PERIPH, ADC0_CH6_OWNED, ADC0_CH6_REF_COUNT, 6);
::bobbin_mcu::channel!(ADC0_CH7, Adc0Ch7, adc0_ch7, ADC0, Adc0, ADC0_CH7_CH, AdcCh, ADC0_PERIPH, ADC0_CH7_OWNED, ADC0_CH7_REF_COUNT, 7);
::bobbin_mcu::channel!(ADC0_CH8, Adc0Ch8, adc0_ch8, ADC0, Adc0, ADC0_CH8_CH, AdcCh, ADC0_PERIPH, ADC0_CH8_OWNED, ADC0_CH8_REF_COUNT, 8);
::bobbin_mcu::channel!(ADC0_CH9, Adc0Ch9, adc0_ch9, ADC0, Adc0, ADC0_CH9_CH, AdcCh, ADC0_PERIPH, ADC0_CH9_OWNED, ADC0_CH9_REF_COUNT, 9);
::bobbin_mcu::channel!(ADC0_CH10, Adc0Ch10, adc0_ch10, ADC0, Adc0, ADC0_CH10_CH, AdcCh, ADC0_PERIPH, ADC0_CH10_OWNED, ADC0_CH10_REF_COUNT, 10);
::bobbin_mcu::channel!(ADC0_CH11, Adc0Ch11, adc0_ch11, ADC0, Adc0, ADC0_CH11_CH, AdcCh, ADC0_PERIPH, ADC0_CH11_OWNED, ADC0_CH11_REF_COUNT, 11);
::bobbin_mcu::channel!(ADC0_CH12, Adc0Ch12, adc0_ch12, ADC0, Adc0, ADC0_CH12_CH, AdcCh, ADC0_PERIPH, ADC0_CH12_OWNED, ADC0_CH12_REF_COUNT, 12);
::bobbin_mcu::channel!(ADC0_CH13, Adc0Ch13, adc0_ch13, ADC0, Adc0, ADC0_CH13_CH, AdcCh, ADC0_PERIPH, ADC0_CH13_OWNED, ADC0_CH13_REF_COUNT, 13);
::bobbin_mcu::channel!(ADC0_CH14, Adc0Ch14, adc0_ch14, ADC0, Adc0, ADC0_CH14_CH, AdcCh, ADC0_PERIPH, ADC0_CH14_OWNED, ADC0_CH14_REF_COUNT, 14);
::bobbin_mcu::channel!(ADC0_CH15, Adc0Ch15, adc0_ch15, ADC0, Adc0, ADC0_CH15_CH, AdcCh, ADC0_PERIPH, ADC0_CH15_OWNED, ADC0_CH15_REF_COUNT, 15);
::bobbin_mcu::channel!(ADC0_CH16, Adc0Ch16, adc0_ch16, ADC0, Adc0, ADC0_CH16_CH, AdcCh, ADC0_PERIPH, ADC0_CH16_OWNED, ADC0_CH16_REF_COUNT, 16);
::bobbin_mcu::channel!(ADC0_CH17, Adc0Ch17, adc0_ch17, ADC0, Adc0, ADC0_CH17_CH, AdcCh, ADC0_PERIPH, ADC0_CH17_OWNED, ADC0_CH17_REF_COUNT, 17);
::bobbin_mcu::channel!(ADC0_CH18, Adc0Ch18, adc0_ch18, ADC0, Adc0, ADC0_CH18_CH, AdcCh, ADC0_PERIPH, ADC0_CH18_OWNED, ADC0_CH18_REF_COUNT, 18);
::bobbin_mcu::channel!(ADC0_CH19, Adc0Ch19, adc0_ch19, ADC0, Adc0, ADC0_CH19_CH, AdcCh, ADC0_PERIPH, ADC0_CH19_OWNED, ADC0_CH19_REF_COUNT, 19);
::bobbin_mcu::channel!(ADC0_CH20, Adc0Ch20, adc0_ch20, ADC0, Adc0, ADC0_CH20_CH, AdcCh, ADC0_PERIPH, ADC0_CH20_OWNED, ADC0_CH20_REF_COUNT, 20);
::bobbin_mcu::channel!(ADC0_CH21, Adc0Ch21, adc0_ch21, ADC0, Adc0, ADC0_CH21_CH, AdcCh, ADC0_PERIPH, ADC0_CH21_OWNED, ADC0_CH21_REF_COUNT, 21);
::bobbin_mcu::channel!(ADC0_CH22, Adc0Ch22, adc0_ch22, ADC0, Adc0, ADC0_CH22_CH, AdcCh, ADC0_PERIPH, ADC0_CH22_OWNED, ADC0_CH22_REF_COUNT, 22);
::bobbin_mcu::channel!(ADC0_CH23, Adc0Ch23, adc0_ch23, ADC0, Adc0, ADC0_CH23_CH, AdcCh, ADC0_PERIPH, ADC0_CH23_OWNED, ADC0_CH23_REF_COUNT, 23);
::bobbin_mcu::channel!(ADC0_TEMP, Adc0Temp, adc0_temp, ADC0, Adc0, ADC0_TEMP_CH, AdcCh, ADC0_PERIPH, ADC0_TEMP_OWNED, ADC0_TEMP_REF_COUNT, 26);
::bobbin_mcu::channel!(ADC0_BANDGAP, Adc0Bandgap, adc0_bandgap, ADC0, Adc0, ADC0_BANDGAP_CH, AdcCh, ADC0_PERIPH, ADC0_BANDGAP_OWNED, ADC0_BANDGAP_REF_COUNT, 27);
::bobbin_mcu::channel!(ADC0_REFSH, Adc0Refsh, adc0_refsh, ADC0, Adc0, ADC0_REFSH_CH, AdcCh, ADC0_PERIPH, ADC0_REFSH_OWNED, ADC0_REFSH_REF_COUNT, 29);
::bobbin_mcu::channel!(ADC0_REFSL, Adc0Refsl, adc0_refsl, ADC0, Adc0, ADC0_REFSL_CH, AdcCh, ADC0_PERIPH, ADC0_REFSL_OWNED, ADC0_REFSL_REF_COUNT, 30);
::bobbin_mcu::channel!(ADC1_CH0, Adc1Ch0, adc1_ch0, ADC1, Adc1, ADC1_CH0_CH, AdcCh, ADC1_PERIPH, ADC1_CH0_OWNED, ADC1_CH0_REF_COUNT, 0);
::bobbin_mcu::channel!(ADC1_CH1, Adc1Ch1, adc1_ch1, ADC1, Adc1, ADC1_CH1_CH, AdcCh, ADC1_PERIPH, ADC1_CH1_OWNED, ADC1_CH1_REF_COUNT, 1);
::bobbin_mcu::channel!(ADC1_CH2, Adc1Ch2, adc1_ch2, ADC1, Adc1, ADC1_CH2_CH, AdcCh, ADC1_PERIPH, ADC1_CH2_OWNED, ADC1_CH2_REF_COUNT, 2);
::bobbin_mcu::channel!(ADC1_CH3, Adc1Ch3, adc1_ch3, ADC1, Adc1, ADC1_CH3_CH, AdcCh, ADC1_PERIPH, ADC1_CH3_OWNED, ADC1_CH3_REF_COUNT, 3);
::bobbin_mcu::channel!(ADC1_CH4, Adc1Ch4, adc1_ch4, ADC1, Adc1, ADC1_CH4_CH, AdcCh, ADC1_PERIPH, ADC1_CH4_OWNED, ADC1_CH4_REF_COUNT, 4);
::bobbin_mcu::channel!(ADC1_CH5, Adc1Ch5, adc1_ch5, ADC1, Adc1, ADC1_CH5_CH, AdcCh, ADC1_PERIPH, ADC1_CH5_OWNED, ADC1_CH5_REF_COUNT, 5);
::bobbin_mcu::channel!(ADC1_CH6, Adc1Ch6, adc1_ch6, ADC1, Adc1, ADC1_CH6_CH, AdcCh, ADC1_PERIPH, ADC1_CH6_OWNED, ADC1_CH6_REF_COUNT, 6);
::bobbin_mcu::channel!(ADC1_CH7, Adc1Ch7, adc1_ch7, ADC1, Adc1, ADC1_CH7_CH, AdcCh, ADC1_PERIPH, ADC1_CH7_OWNED, ADC1_CH7_REF_COUNT, 7);
::bobbin_mcu::channel!(ADC1_CH8, Adc1Ch8, adc1_ch8, ADC1, Adc1, ADC1_CH8_CH, AdcCh, ADC1_PERIPH, ADC1_CH8_OWNED, ADC1_CH8_REF_COUNT, 8);
::bobbin_mcu::channel!(ADC1_CH9, Adc1Ch9, adc1_ch9, ADC1, Adc1, ADC1_CH9_CH, AdcCh, ADC1_PERIPH, ADC1_CH9_OWNED, ADC1_CH9_REF_COUNT, 9);
::bobbin_mcu::channel!(ADC1_CH10, Adc1Ch10, adc1_ch10, ADC1, Adc1, ADC1_CH10_CH, AdcCh, ADC1_PERIPH, ADC1_CH10_OWNED, ADC1_CH10_REF_COUNT, 10);
::bobbin_mcu::channel!(ADC1_CH11, Adc1Ch11, adc1_ch11, ADC1, Adc1, ADC1_CH11_CH, AdcCh, ADC1_PERIPH, ADC1_CH11_OWNED, ADC1_CH11_REF_COUNT, 11);
::bobbin_mcu::channel!(ADC1_CH12, Adc1Ch12, adc1_ch12, ADC1, Adc1, ADC1_CH12_CH, AdcCh, ADC1_PERIPH, ADC1_CH12_OWNED, ADC1_CH12_REF_COUNT, 12);
::bobbin_mcu::channel!(ADC1_CH13, Adc1Ch13, adc1_ch13, ADC1, Adc1, ADC1_CH13_CH, AdcCh, ADC1_PERIPH, ADC1_CH13_OWNED, ADC1_CH13_REF_COUNT, 13);
::bobbin_mcu::channel!(ADC1_CH14, Adc1Ch14, adc1_ch14, ADC1, Adc1, ADC1_CH14_CH, AdcCh, ADC1_PERIPH, ADC1_CH14_OWNED, ADC1_CH14_REF_COUNT, 14);
::bobbin_mcu::channel!(ADC1_CH15, Adc1Ch15, adc1_ch15, ADC1, Adc1, ADC1_CH15_CH, AdcCh, ADC1_PERIPH, ADC1_CH15_OWNED, ADC1_CH15_REF_COUNT, 15);
::bobbin_mcu::channel!(ADC1_CH16, Adc1Ch16, adc1_ch16, ADC1, Adc1, ADC1_CH16_CH, AdcCh, ADC1_PERIPH, ADC1_CH16_OWNED, ADC1_CH16_REF_COUNT, 16);
::bobbin_mcu::channel!(ADC1_CH17, Adc1Ch17, adc1_ch17, ADC1, Adc1, ADC1_CH17_CH, AdcCh, ADC1_PERIPH, ADC1_CH17_OWNED, ADC1_CH17_REF_COUNT, 17);
::bobbin_mcu::channel!(ADC1_CH18, Adc1Ch18, adc1_ch18, ADC1, Adc1, ADC1_CH18_CH, AdcCh, ADC1_PERIPH, ADC1_CH18_OWNED, ADC1_CH18_REF_COUNT, 18);
::bobbin_mcu::channel!(ADC1_CH19, Adc1Ch19, adc1_ch19, ADC1, Adc1, ADC1_CH19_CH, AdcCh, ADC1_PERIPH, ADC1_CH19_OWNED, ADC1_CH19_REF_COUNT, 19);
::bobbin_mcu::channel!(ADC1_CH20, Adc1Ch20, adc1_ch20, ADC1, Adc1, ADC1_CH20_CH, AdcCh, ADC1_PERIPH, ADC1_CH20_OWNED, ADC1_CH20_REF_COUNT, 20);
::bobbin_mcu::channel!(ADC1_CH21, Adc1Ch21, adc1_ch21, ADC1, Adc1, ADC1_CH21_CH, AdcCh, ADC1_PERIPH, ADC1_CH21_OWNED, ADC1_CH21_REF_COUNT, 21);
::bobbin_mcu::channel!(ADC1_CH22, Adc1Ch22, adc1_ch22, ADC1, Adc1, ADC1_CH22_CH, AdcCh, ADC1_PERIPH, ADC1_CH22_OWNED, ADC1_CH22_REF_COUNT, 22);
::bobbin_mcu::channel!(ADC1_CH23, Adc1Ch23, adc1_ch23, ADC1, Adc1, ADC1_CH23_CH, AdcCh, ADC1_PERIPH, ADC1_CH23_OWNED, ADC1_CH23_REF_COUNT, 23);
::bobbin_mcu::channel!(ADC1_TEMP, Adc1Temp, adc1_temp, ADC1, Adc1, ADC1_TEMP_CH, AdcCh, ADC1_PERIPH, ADC1_TEMP_OWNED, ADC1_TEMP_REF_COUNT, 26);
::bobbin_mcu::channel!(ADC1_BANDGAP, Adc1Bandgap, adc1_bandgap, ADC1, Adc1, ADC1_BANDGAP_CH, AdcCh, ADC1_PERIPH, ADC1_BANDGAP_OWNED, ADC1_BANDGAP_REF_COUNT, 27);
::bobbin_mcu::channel!(ADC1_REFSH, Adc1Refsh, adc1_refsh, ADC1, Adc1, ADC1_REFSH_CH, AdcCh, ADC1_PERIPH, ADC1_REFSH_OWNED, ADC1_REFSH_REF_COUNT, 29);
::bobbin_mcu::channel!(ADC1_REFSL, Adc1Refsl, adc1_refsl, ADC1, Adc1, ADC1_REFSL_CH, AdcCh, ADC1_PERIPH, ADC1_REFSL_OWNED, ADC1_REFSL_REF_COUNT, 30);
// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("ADC0"), description: None }
impl ::bobbin_mcu::gate::GateEn for Adc0 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc6().adc0() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_adc0(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC3"), field: Some("ADC1"), description: None }
impl ::bobbin_mcu::gate::GateEn for Adc1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc3().adc1() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc3(|r| r.set_adc1(value));
        self
    }
}

