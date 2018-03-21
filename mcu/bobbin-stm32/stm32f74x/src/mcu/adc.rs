#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::adc::*;

periph!( ADC1, Adc1, ADC1_PERIPH, AdcPeriph, 0x40012000, 0x2b);
periph!( ADC2, Adc2, ADC2_PERIPH, AdcPeriph, 0x40012100, 0x2c);
periph!( ADC3, Adc3, ADC3_PERIPH, AdcPeriph, 0x40012200, 0x2d);

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
channel!(ADC1_TEMP, Adc1Temp, ADC1, Adc1, ADC1_TEMP_CH, AdcCh, ADC1_PERIPH, 16);
channel!(ADC1_REF, Adc1Ref, ADC1, Adc1, ADC1_REF_CH, AdcCh, ADC1_PERIPH, 17);
channel!(ADC1_BAT, Adc1Bat, ADC1, Adc1, ADC1_BAT_CH, AdcCh, ADC1_PERIPH, 18);
channel!(ADC2_CH0, Adc2Ch0, ADC2, Adc2, ADC2_CH0_CH, AdcCh, ADC2_PERIPH, 0);
channel!(ADC2_CH1, Adc2Ch1, ADC2, Adc2, ADC2_CH1_CH, AdcCh, ADC2_PERIPH, 1);
channel!(ADC2_CH2, Adc2Ch2, ADC2, Adc2, ADC2_CH2_CH, AdcCh, ADC2_PERIPH, 2);
channel!(ADC2_CH3, Adc2Ch3, ADC2, Adc2, ADC2_CH3_CH, AdcCh, ADC2_PERIPH, 3);
channel!(ADC2_CH4, Adc2Ch4, ADC2, Adc2, ADC2_CH4_CH, AdcCh, ADC2_PERIPH, 4);
channel!(ADC2_CH5, Adc2Ch5, ADC2, Adc2, ADC2_CH5_CH, AdcCh, ADC2_PERIPH, 5);
channel!(ADC2_CH6, Adc2Ch6, ADC2, Adc2, ADC2_CH6_CH, AdcCh, ADC2_PERIPH, 6);
channel!(ADC2_CH7, Adc2Ch7, ADC2, Adc2, ADC2_CH7_CH, AdcCh, ADC2_PERIPH, 7);
channel!(ADC2_CH8, Adc2Ch8, ADC2, Adc2, ADC2_CH8_CH, AdcCh, ADC2_PERIPH, 8);
channel!(ADC2_CH9, Adc2Ch9, ADC2, Adc2, ADC2_CH9_CH, AdcCh, ADC2_PERIPH, 9);
channel!(ADC2_CH10, Adc2Ch10, ADC2, Adc2, ADC2_CH10_CH, AdcCh, ADC2_PERIPH, 10);
channel!(ADC2_CH11, Adc2Ch11, ADC2, Adc2, ADC2_CH11_CH, AdcCh, ADC2_PERIPH, 11);
channel!(ADC2_CH12, Adc2Ch12, ADC2, Adc2, ADC2_CH12_CH, AdcCh, ADC2_PERIPH, 12);
channel!(ADC2_CH13, Adc2Ch13, ADC2, Adc2, ADC2_CH13_CH, AdcCh, ADC2_PERIPH, 13);
channel!(ADC2_CH14, Adc2Ch14, ADC2, Adc2, ADC2_CH14_CH, AdcCh, ADC2_PERIPH, 14);
channel!(ADC2_CH15, Adc2Ch15, ADC2, Adc2, ADC2_CH15_CH, AdcCh, ADC2_PERIPH, 15);
channel!(ADC3_CH0, Adc3Ch0, ADC3, Adc3, ADC3_CH0_CH, AdcCh, ADC3_PERIPH, 0);
channel!(ADC3_CH1, Adc3Ch1, ADC3, Adc3, ADC3_CH1_CH, AdcCh, ADC3_PERIPH, 1);
channel!(ADC3_CH2, Adc3Ch2, ADC3, Adc3, ADC3_CH2_CH, AdcCh, ADC3_PERIPH, 2);
channel!(ADC3_CH3, Adc3Ch3, ADC3, Adc3, ADC3_CH3_CH, AdcCh, ADC3_PERIPH, 3);
channel!(ADC3_CH4, Adc3Ch4, ADC3, Adc3, ADC3_CH4_CH, AdcCh, ADC3_PERIPH, 4);
channel!(ADC3_CH5, Adc3Ch5, ADC3, Adc3, ADC3_CH5_CH, AdcCh, ADC3_PERIPH, 5);
channel!(ADC3_CH6, Adc3Ch6, ADC3, Adc3, ADC3_CH6_CH, AdcCh, ADC3_PERIPH, 6);
channel!(ADC3_CH7, Adc3Ch7, ADC3, Adc3, ADC3_CH7_CH, AdcCh, ADC3_PERIPH, 7);
channel!(ADC3_CH8, Adc3Ch8, ADC3, Adc3, ADC3_CH8_CH, AdcCh, ADC3_PERIPH, 8);
channel!(ADC3_CH9, Adc3Ch9, ADC3, Adc3, ADC3_CH9_CH, AdcCh, ADC3_PERIPH, 9);
channel!(ADC3_CH10, Adc3Ch10, ADC3, Adc3, ADC3_CH10_CH, AdcCh, ADC3_PERIPH, 10);
channel!(ADC3_CH11, Adc3Ch11, ADC3, Adc3, ADC3_CH11_CH, AdcCh, ADC3_PERIPH, 11);
channel!(ADC3_CH12, Adc3Ch12, ADC3, Adc3, ADC3_CH12_CH, AdcCh, ADC3_PERIPH, 12);
channel!(ADC3_CH13, Adc3Ch13, ADC3, Adc3, ADC3_CH13_CH, AdcCh, ADC3_PERIPH, 13);
channel!(ADC3_CH14, Adc3Ch14, ADC3, Adc3, ADC3_CH14_CH, AdcCh, ADC3_PERIPH, 14);
channel!(ADC3_CH15, Adc3Ch15, ADC3, Adc3, ADC3_CH15_CH, AdcCh, ADC3_PERIPH, 15);
// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("ADCRST"), description: None }
impl ::bobbin_common::gate::GateRst for Adc1 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb2rstr().adcrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_adcrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("ADC1EN"), description: None }
impl ::bobbin_common::gate::GateEn for Adc1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb2enr().adc1en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_adc1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2LPENR"), field: Some("ADC1LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Adc1 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb2lpenr().adc1lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2lpenr(|r| r.set_adc1lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("ADCRST"), description: None }
impl ::bobbin_common::gate::GateRst for Adc2 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb2rstr().adcrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_adcrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("ADC2EN"), description: None }
impl ::bobbin_common::gate::GateEn for Adc2 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb2enr().adc2en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_adc2en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2LPENR"), field: Some("ADC2LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Adc2 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb2lpenr().adc2lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2lpenr(|r| r.set_adc2lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("ADCRST"), description: None }
impl ::bobbin_common::gate::GateRst for Adc3 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb2rstr().adcrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_adcrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("ADC3EN"), description: None }
impl ::bobbin_common::gate::GateEn for Adc3 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb2enr().adc3en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_adc3en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2LPENR"), field: Some("ADC3LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Adc3 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb2lpenr().adc3lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2lpenr(|r| r.set_adc3lpen(value));
        self
    }
}

