pub use ::stm32_common::adc_f3::*;

::bobbin_mcu::periph!( ADC1, Adc1, ADC1_PERIPH, AdcPeriph, ADC1_OWNED, ADC1_REF_COUNT, 0x50000000, 0x00, 0x1c);
::bobbin_mcu::periph!( ADC2, Adc2, ADC2_PERIPH, AdcPeriph, ADC2_OWNED, ADC2_REF_COUNT, 0x50000100, 0x01, 0x1d);
::bobbin_mcu::periph!( ADC3, Adc3, ADC3_PERIPH, AdcPeriph, ADC3_OWNED, ADC3_REF_COUNT, 0x50000400, 0x02, 0x1e);
::bobbin_mcu::periph!( ADC4, Adc4, ADC4_PERIPH, AdcPeriph, ADC4_OWNED, ADC4_REF_COUNT, 0x50000500, 0x03, 0x1f);

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
::bobbin_mcu::channel!(ADC1_TEMP, Adc1Temp, adc1_temp, ADC1, Adc1, ADC1_TEMP_CH, AdcCh, ADC1_PERIPH, ADC1_TEMP_OWNED, ADC1_TEMP_REF_COUNT, 16);
::bobbin_mcu::channel!(ADC1_REFINT, Adc1Refint, adc1_refint, ADC1, Adc1, ADC1_REFINT_CH, AdcCh, ADC1_PERIPH, ADC1_REFINT_OWNED, ADC1_REFINT_REF_COUNT, 18);
::bobbin_mcu::channel!(ADC1_BAT, Adc1Bat, adc1_bat, ADC1, Adc1, ADC1_BAT_CH, AdcCh, ADC1_PERIPH, ADC1_BAT_OWNED, ADC1_BAT_REF_COUNT, 17);
::bobbin_mcu::channel!(ADC2_CH1, Adc2Ch1, adc2_ch1, ADC2, Adc2, ADC2_CH1_CH, AdcCh, ADC2_PERIPH, ADC2_CH1_OWNED, ADC2_CH1_REF_COUNT, 1);
::bobbin_mcu::channel!(ADC2_CH2, Adc2Ch2, adc2_ch2, ADC2, Adc2, ADC2_CH2_CH, AdcCh, ADC2_PERIPH, ADC2_CH2_OWNED, ADC2_CH2_REF_COUNT, 2);
::bobbin_mcu::channel!(ADC2_CH3, Adc2Ch3, adc2_ch3, ADC2, Adc2, ADC2_CH3_CH, AdcCh, ADC2_PERIPH, ADC2_CH3_OWNED, ADC2_CH3_REF_COUNT, 3);
::bobbin_mcu::channel!(ADC2_CH4, Adc2Ch4, adc2_ch4, ADC2, Adc2, ADC2_CH4_CH, AdcCh, ADC2_PERIPH, ADC2_CH4_OWNED, ADC2_CH4_REF_COUNT, 4);
::bobbin_mcu::channel!(ADC2_CH5, Adc2Ch5, adc2_ch5, ADC2, Adc2, ADC2_CH5_CH, AdcCh, ADC2_PERIPH, ADC2_CH5_OWNED, ADC2_CH5_REF_COUNT, 5);
::bobbin_mcu::channel!(ADC2_CH6, Adc2Ch6, adc2_ch6, ADC2, Adc2, ADC2_CH6_CH, AdcCh, ADC2_PERIPH, ADC2_CH6_OWNED, ADC2_CH6_REF_COUNT, 6);
::bobbin_mcu::channel!(ADC2_CH7, Adc2Ch7, adc2_ch7, ADC2, Adc2, ADC2_CH7_CH, AdcCh, ADC2_PERIPH, ADC2_CH7_OWNED, ADC2_CH7_REF_COUNT, 7);
::bobbin_mcu::channel!(ADC2_CH8, Adc2Ch8, adc2_ch8, ADC2, Adc2, ADC2_CH8_CH, AdcCh, ADC2_PERIPH, ADC2_CH8_OWNED, ADC2_CH8_REF_COUNT, 8);
::bobbin_mcu::channel!(ADC2_CH9, Adc2Ch9, adc2_ch9, ADC2, Adc2, ADC2_CH9_CH, AdcCh, ADC2_PERIPH, ADC2_CH9_OWNED, ADC2_CH9_REF_COUNT, 9);
::bobbin_mcu::channel!(ADC2_CH10, Adc2Ch10, adc2_ch10, ADC2, Adc2, ADC2_CH10_CH, AdcCh, ADC2_PERIPH, ADC2_CH10_OWNED, ADC2_CH10_REF_COUNT, 10);
::bobbin_mcu::channel!(ADC2_CH11, Adc2Ch11, adc2_ch11, ADC2, Adc2, ADC2_CH11_CH, AdcCh, ADC2_PERIPH, ADC2_CH11_OWNED, ADC2_CH11_REF_COUNT, 11);
::bobbin_mcu::channel!(ADC2_CH12, Adc2Ch12, adc2_ch12, ADC2, Adc2, ADC2_CH12_CH, AdcCh, ADC2_PERIPH, ADC2_CH12_OWNED, ADC2_CH12_REF_COUNT, 12);
::bobbin_mcu::channel!(ADC2_CH13, Adc2Ch13, adc2_ch13, ADC2, Adc2, ADC2_CH13_CH, AdcCh, ADC2_PERIPH, ADC2_CH13_OWNED, ADC2_CH13_REF_COUNT, 13);
::bobbin_mcu::channel!(ADC2_CH14, Adc2Ch14, adc2_ch14, ADC2, Adc2, ADC2_CH14_CH, AdcCh, ADC2_PERIPH, ADC2_CH14_OWNED, ADC2_CH14_REF_COUNT, 14);
::bobbin_mcu::channel!(ADC2_CH15, Adc2Ch15, adc2_ch15, ADC2, Adc2, ADC2_CH15_CH, AdcCh, ADC2_PERIPH, ADC2_CH15_OWNED, ADC2_CH15_REF_COUNT, 15);
::bobbin_mcu::channel!(ADC2_REFINT, Adc2Refint, adc2_refint, ADC2, Adc2, ADC2_REFINT_CH, AdcCh, ADC2_PERIPH, ADC2_REFINT_OWNED, ADC2_REFINT_REF_COUNT, 18);
::bobbin_mcu::channel!(ADC3_CH1, Adc3Ch1, adc3_ch1, ADC3, Adc3, ADC3_CH1_CH, AdcCh, ADC3_PERIPH, ADC3_CH1_OWNED, ADC3_CH1_REF_COUNT, 1);
::bobbin_mcu::channel!(ADC3_CH2, Adc3Ch2, adc3_ch2, ADC3, Adc3, ADC3_CH2_CH, AdcCh, ADC3_PERIPH, ADC3_CH2_OWNED, ADC3_CH2_REF_COUNT, 2);
::bobbin_mcu::channel!(ADC3_CH3, Adc3Ch3, adc3_ch3, ADC3, Adc3, ADC3_CH3_CH, AdcCh, ADC3_PERIPH, ADC3_CH3_OWNED, ADC3_CH3_REF_COUNT, 3);
::bobbin_mcu::channel!(ADC3_CH4, Adc3Ch4, adc3_ch4, ADC3, Adc3, ADC3_CH4_CH, AdcCh, ADC3_PERIPH, ADC3_CH4_OWNED, ADC3_CH4_REF_COUNT, 4);
::bobbin_mcu::channel!(ADC3_CH5, Adc3Ch5, adc3_ch5, ADC3, Adc3, ADC3_CH5_CH, AdcCh, ADC3_PERIPH, ADC3_CH5_OWNED, ADC3_CH5_REF_COUNT, 5);
::bobbin_mcu::channel!(ADC3_CH6, Adc3Ch6, adc3_ch6, ADC3, Adc3, ADC3_CH6_CH, AdcCh, ADC3_PERIPH, ADC3_CH6_OWNED, ADC3_CH6_REF_COUNT, 6);
::bobbin_mcu::channel!(ADC3_CH7, Adc3Ch7, adc3_ch7, ADC3, Adc3, ADC3_CH7_CH, AdcCh, ADC3_PERIPH, ADC3_CH7_OWNED, ADC3_CH7_REF_COUNT, 7);
::bobbin_mcu::channel!(ADC3_CH8, Adc3Ch8, adc3_ch8, ADC3, Adc3, ADC3_CH8_CH, AdcCh, ADC3_PERIPH, ADC3_CH8_OWNED, ADC3_CH8_REF_COUNT, 8);
::bobbin_mcu::channel!(ADC3_CH9, Adc3Ch9, adc3_ch9, ADC3, Adc3, ADC3_CH9_CH, AdcCh, ADC3_PERIPH, ADC3_CH9_OWNED, ADC3_CH9_REF_COUNT, 9);
::bobbin_mcu::channel!(ADC3_CH10, Adc3Ch10, adc3_ch10, ADC3, Adc3, ADC3_CH10_CH, AdcCh, ADC3_PERIPH, ADC3_CH10_OWNED, ADC3_CH10_REF_COUNT, 10);
::bobbin_mcu::channel!(ADC3_CH11, Adc3Ch11, adc3_ch11, ADC3, Adc3, ADC3_CH11_CH, AdcCh, ADC3_PERIPH, ADC3_CH11_OWNED, ADC3_CH11_REF_COUNT, 11);
::bobbin_mcu::channel!(ADC3_CH12, Adc3Ch12, adc3_ch12, ADC3, Adc3, ADC3_CH12_CH, AdcCh, ADC3_PERIPH, ADC3_CH12_OWNED, ADC3_CH12_REF_COUNT, 12);
::bobbin_mcu::channel!(ADC3_CH13, Adc3Ch13, adc3_ch13, ADC3, Adc3, ADC3_CH13_CH, AdcCh, ADC3_PERIPH, ADC3_CH13_OWNED, ADC3_CH13_REF_COUNT, 13);
::bobbin_mcu::channel!(ADC3_CH14, Adc3Ch14, adc3_ch14, ADC3, Adc3, ADC3_CH14_CH, AdcCh, ADC3_PERIPH, ADC3_CH14_OWNED, ADC3_CH14_REF_COUNT, 14);
::bobbin_mcu::channel!(ADC3_CH15, Adc3Ch15, adc3_ch15, ADC3, Adc3, ADC3_CH15_CH, AdcCh, ADC3_PERIPH, ADC3_CH15_OWNED, ADC3_CH15_REF_COUNT, 15);
::bobbin_mcu::channel!(ADC3_REFINT, Adc3Refint, adc3_refint, ADC3, Adc3, ADC3_REFINT_CH, AdcCh, ADC3_PERIPH, ADC3_REFINT_OWNED, ADC3_REFINT_REF_COUNT, 18);
::bobbin_mcu::channel!(ADC4_CH1, Adc4Ch1, adc4_ch1, ADC4, Adc4, ADC4_CH1_CH, AdcCh, ADC4_PERIPH, ADC4_CH1_OWNED, ADC4_CH1_REF_COUNT, 1);
::bobbin_mcu::channel!(ADC4_CH2, Adc4Ch2, adc4_ch2, ADC4, Adc4, ADC4_CH2_CH, AdcCh, ADC4_PERIPH, ADC4_CH2_OWNED, ADC4_CH2_REF_COUNT, 2);
::bobbin_mcu::channel!(ADC4_CH3, Adc4Ch3, adc4_ch3, ADC4, Adc4, ADC4_CH3_CH, AdcCh, ADC4_PERIPH, ADC4_CH3_OWNED, ADC4_CH3_REF_COUNT, 3);
::bobbin_mcu::channel!(ADC4_CH4, Adc4Ch4, adc4_ch4, ADC4, Adc4, ADC4_CH4_CH, AdcCh, ADC4_PERIPH, ADC4_CH4_OWNED, ADC4_CH4_REF_COUNT, 4);
::bobbin_mcu::channel!(ADC4_CH5, Adc4Ch5, adc4_ch5, ADC4, Adc4, ADC4_CH5_CH, AdcCh, ADC4_PERIPH, ADC4_CH5_OWNED, ADC4_CH5_REF_COUNT, 5);
::bobbin_mcu::channel!(ADC4_CH6, Adc4Ch6, adc4_ch6, ADC4, Adc4, ADC4_CH6_CH, AdcCh, ADC4_PERIPH, ADC4_CH6_OWNED, ADC4_CH6_REF_COUNT, 6);
::bobbin_mcu::channel!(ADC4_CH7, Adc4Ch7, adc4_ch7, ADC4, Adc4, ADC4_CH7_CH, AdcCh, ADC4_PERIPH, ADC4_CH7_OWNED, ADC4_CH7_REF_COUNT, 7);
::bobbin_mcu::channel!(ADC4_CH8, Adc4Ch8, adc4_ch8, ADC4, Adc4, ADC4_CH8_CH, AdcCh, ADC4_PERIPH, ADC4_CH8_OWNED, ADC4_CH8_REF_COUNT, 8);
::bobbin_mcu::channel!(ADC4_CH9, Adc4Ch9, adc4_ch9, ADC4, Adc4, ADC4_CH9_CH, AdcCh, ADC4_PERIPH, ADC4_CH9_OWNED, ADC4_CH9_REF_COUNT, 9);
::bobbin_mcu::channel!(ADC4_CH10, Adc4Ch10, adc4_ch10, ADC4, Adc4, ADC4_CH10_CH, AdcCh, ADC4_PERIPH, ADC4_CH10_OWNED, ADC4_CH10_REF_COUNT, 10);
::bobbin_mcu::channel!(ADC4_CH11, Adc4Ch11, adc4_ch11, ADC4, Adc4, ADC4_CH11_CH, AdcCh, ADC4_PERIPH, ADC4_CH11_OWNED, ADC4_CH11_REF_COUNT, 11);
::bobbin_mcu::channel!(ADC4_CH12, Adc4Ch12, adc4_ch12, ADC4, Adc4, ADC4_CH12_CH, AdcCh, ADC4_PERIPH, ADC4_CH12_OWNED, ADC4_CH12_REF_COUNT, 12);
::bobbin_mcu::channel!(ADC4_CH13, Adc4Ch13, adc4_ch13, ADC4, Adc4, ADC4_CH13_CH, AdcCh, ADC4_PERIPH, ADC4_CH13_OWNED, ADC4_CH13_REF_COUNT, 13);
::bobbin_mcu::channel!(ADC4_CH14, Adc4Ch14, adc4_ch14, ADC4, Adc4, ADC4_CH14_CH, AdcCh, ADC4_PERIPH, ADC4_CH14_OWNED, ADC4_CH14_REF_COUNT, 14);
::bobbin_mcu::channel!(ADC4_CH15, Adc4Ch15, adc4_ch15, ADC4, Adc4, ADC4_CH15_CH, AdcCh, ADC4_PERIPH, ADC4_CH15_OWNED, ADC4_CH15_REF_COUNT, 15);
::bobbin_mcu::channel!(ADC4_REFINT, Adc4Refint, adc4_refint, ADC4, Adc4, ADC4_REFINT_CH, AdcCh, ADC4_PERIPH, ADC4_REFINT_OWNED, ADC4_REFINT_REF_COUNT, 18);
// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("ADC12EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Adc1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().adc12en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_adc12en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("ADC12EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Adc2 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().adc12en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_adc12en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("ADC34EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Adc3 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().adc34en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_adc34en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("ADC34EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Adc4 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().adc34en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_adc34en(value));
        self
    }
}

