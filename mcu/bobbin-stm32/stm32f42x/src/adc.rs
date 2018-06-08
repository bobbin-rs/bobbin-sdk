pub use ::stm32_common::adc_f24::*;

::bobbin_mcu::periph!( ADC1, Adc1, ADC1_PERIPH, AdcPeriph, ADC1_OWNED, ADC1_REF_COUNT, 0x40012000, 0x00, 0x21);
::bobbin_mcu::periph!( ADC2, Adc2, ADC2_PERIPH, AdcPeriph, ADC2_OWNED, ADC2_REF_COUNT, 0x40012100, 0x01, 0x22);
::bobbin_mcu::periph!( ADC3, Adc3, ADC3_PERIPH, AdcPeriph, ADC3_OWNED, ADC3_REF_COUNT, 0x40012200, 0x02, 0x23);

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
::bobbin_mcu::channel!(ADC1_TEMP, Adc1Temp, adc1_temp, ADC1, Adc1, ADC1_TEMP_CH, AdcCh, ADC1_PERIPH, ADC1_TEMP_OWNED, ADC1_TEMP_REF_COUNT, 16);
::bobbin_mcu::channel!(ADC1_REF, Adc1Ref, adc1_ref, ADC1, Adc1, ADC1_REF_CH, AdcCh, ADC1_PERIPH, ADC1_REF_OWNED, ADC1_REF_REF_COUNT, 17);
::bobbin_mcu::channel!(ADC1_BAT, Adc1Bat, adc1_bat, ADC1, Adc1, ADC1_BAT_CH, AdcCh, ADC1_PERIPH, ADC1_BAT_OWNED, ADC1_BAT_REF_COUNT, 18);
::bobbin_mcu::channel!(ADC2_CH0, Adc2Ch0, adc2_ch0, ADC2, Adc2, ADC2_CH0_CH, AdcCh, ADC2_PERIPH, ADC2_CH0_OWNED, ADC2_CH0_REF_COUNT, 0);
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
::bobbin_mcu::channel!(ADC3_CH0, Adc3Ch0, adc3_ch0, ADC3, Adc3, ADC3_CH0_CH, AdcCh, ADC3_PERIPH, ADC3_CH0_OWNED, ADC3_CH0_REF_COUNT, 0);
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
// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("ADCRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Adc1 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2rstr().adcrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_adcrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("ADC1EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Adc1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2enr().adc1en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_adc1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2LPENR"), field: Some("ADC1LPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Adc1 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2lpenr().adc1lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2lpenr(|r| r.set_adc1lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("ADCRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Adc2 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2rstr().adcrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_adcrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("ADC2EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Adc2 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2enr().adc2en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_adc2en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2LPENR"), field: Some("ADC2LPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Adc2 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2lpenr().adc2lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2lpenr(|r| r.set_adc2lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("ADCRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Adc3 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2rstr().adcrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_adcrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("ADC3EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Adc3 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2enr().adc3en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_adc3en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2LPENR"), field: Some("ADC3LPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Adc3 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2lpenr().adc3lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2lpenr(|r| r.set_adc3lpen(value));
        self
    }
}

