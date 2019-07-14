pub use ::stm32_common::adc_f3::*;

::bobbin_mcu::periph!( ADC1, Adc1, ADC1_PERIPH, AdcPeriph, ADC1_OWNED, ADC1_REF_COUNT, 0x50040000, 0x00, 0x0c);

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
::bobbin_mcu::channel!(ADC1_TEMP, Adc1Temp, adc1_temp, ADC1, Adc1, ADC1_TEMP_CH, AdcCh, ADC1_PERIPH, ADC1_TEMP_OWNED, ADC1_TEMP_REF_COUNT, 17);
::bobbin_mcu::channel!(ADC1_REFINT, Adc1Refint, adc1_refint, ADC1, Adc1, ADC1_REFINT_CH, AdcCh, ADC1_PERIPH, ADC1_REFINT_OWNED, ADC1_REFINT_REF_COUNT, 0);
::bobbin_mcu::channel!(ADC1_BAT, Adc1Bat, adc1_bat, ADC1, Adc1, ADC1_BAT_CH, AdcCh, ADC1_PERIPH, ADC1_BAT_OWNED, ADC1_BAT_REF_COUNT, 18);
// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("ADCRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Adc1 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2rstr().adcrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_adcrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("ADCEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Adc1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2enr().adcen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_adcen(value));
        self
    }
}

