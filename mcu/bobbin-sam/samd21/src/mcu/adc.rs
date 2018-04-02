#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::adc::*;

periph!( ADC, Adc, ADC_PERIPH, AdcPeriph, ADC_OWNED, ADC_REF_COUNT, 0x42004000, 0x00, 0x07);

channel!(ADC_CH0, AdcCh0, adc_ch0, ADC, Adc, ADC_CH0_CH, AdcCh, ADC_PERIPH, ADC_CH0_OWNED, ADC_CH0_REF_COUNT, 0);
channel!(ADC_CH1, AdcCh1, adc_ch1, ADC, Adc, ADC_CH1_CH, AdcCh, ADC_PERIPH, ADC_CH1_OWNED, ADC_CH1_REF_COUNT, 1);
channel!(ADC_CH2, AdcCh2, adc_ch2, ADC, Adc, ADC_CH2_CH, AdcCh, ADC_PERIPH, ADC_CH2_OWNED, ADC_CH2_REF_COUNT, 2);
channel!(ADC_CH3, AdcCh3, adc_ch3, ADC, Adc, ADC_CH3_CH, AdcCh, ADC_PERIPH, ADC_CH3_OWNED, ADC_CH3_REF_COUNT, 3);
channel!(ADC_CH4, AdcCh4, adc_ch4, ADC, Adc, ADC_CH4_CH, AdcCh, ADC_PERIPH, ADC_CH4_OWNED, ADC_CH4_REF_COUNT, 4);
channel!(ADC_CH5, AdcCh5, adc_ch5, ADC, Adc, ADC_CH5_CH, AdcCh, ADC_PERIPH, ADC_CH5_OWNED, ADC_CH5_REF_COUNT, 5);
channel!(ADC_CH6, AdcCh6, adc_ch6, ADC, Adc, ADC_CH6_CH, AdcCh, ADC_PERIPH, ADC_CH6_OWNED, ADC_CH6_REF_COUNT, 6);
channel!(ADC_CH7, AdcCh7, adc_ch7, ADC, Adc, ADC_CH7_CH, AdcCh, ADC_PERIPH, ADC_CH7_OWNED, ADC_CH7_REF_COUNT, 7);
channel!(ADC_CH8, AdcCh8, adc_ch8, ADC, Adc, ADC_CH8_CH, AdcCh, ADC_PERIPH, ADC_CH8_OWNED, ADC_CH8_REF_COUNT, 8);
channel!(ADC_CH9, AdcCh9, adc_ch9, ADC, Adc, ADC_CH9_CH, AdcCh, ADC_PERIPH, ADC_CH9_OWNED, ADC_CH9_REF_COUNT, 9);
channel!(ADC_CH10, AdcCh10, adc_ch10, ADC, Adc, ADC_CH10_CH, AdcCh, ADC_PERIPH, ADC_CH10_OWNED, ADC_CH10_REF_COUNT, 10);
channel!(ADC_CH11, AdcCh11, adc_ch11, ADC, Adc, ADC_CH11_CH, AdcCh, ADC_PERIPH, ADC_CH11_OWNED, ADC_CH11_REF_COUNT, 11);
channel!(ADC_CH12, AdcCh12, adc_ch12, ADC, Adc, ADC_CH12_CH, AdcCh, ADC_PERIPH, ADC_CH12_OWNED, ADC_CH12_REF_COUNT, 12);
channel!(ADC_CH13, AdcCh13, adc_ch13, ADC, Adc, ADC_CH13_CH, AdcCh, ADC_PERIPH, ADC_CH13_OWNED, ADC_CH13_REF_COUNT, 13);
channel!(ADC_CH14, AdcCh14, adc_ch14, ADC, Adc, ADC_CH14_CH, AdcCh, ADC_PERIPH, ADC_CH14_OWNED, ADC_CH14_REF_COUNT, 14);
channel!(ADC_CH15, AdcCh15, adc_ch15, ADC, Adc, ADC_CH15_CH, AdcCh, ADC_PERIPH, ADC_CH15_OWNED, ADC_CH15_REF_COUNT, 15);
channel!(ADC_CH16, AdcCh16, adc_ch16, ADC, Adc, ADC_CH16_CH, AdcCh, ADC_PERIPH, ADC_CH16_OWNED, ADC_CH16_REF_COUNT, 16);
channel!(ADC_CH17, AdcCh17, adc_ch17, ADC, Adc, ADC_CH17_CH, AdcCh, ADC_PERIPH, ADC_CH17_OWNED, ADC_CH17_REF_COUNT, 17);
channel!(ADC_CH18, AdcCh18, adc_ch18, ADC, Adc, ADC_CH18_CH, AdcCh, ADC_PERIPH, ADC_CH18_OWNED, ADC_CH18_REF_COUNT, 18);
channel!(ADC_CH19, AdcCh19, adc_ch19, ADC, Adc, ADC_CH19_CH, AdcCh, ADC_PERIPH, ADC_CH19_OWNED, ADC_CH19_REF_COUNT, 19);
channel!(ADC_TEMP, AdcTemp, adc_temp, ADC, Adc, ADC_TEMP_CH, AdcCh, ADC_PERIPH, ADC_TEMP_OWNED, ADC_TEMP_REF_COUNT, 24);
channel!(ADC_BANDGAP, AdcBandgap, adc_bandgap, ADC, Adc, ADC_BANDGAP_CH, AdcCh, ADC_PERIPH, ADC_BANDGAP_OWNED, ADC_BANDGAP_REF_COUNT, 25);
channel!(ADC_SCALED_CORE, AdcScaledCore, adc_scaled_core, ADC, Adc, ADC_SCALED_CORE_CH, AdcCh, ADC_PERIPH, ADC_SCALED_CORE_OWNED, ADC_SCALED_CORE_REF_COUNT, 26);
channel!(ADC_SCALED_IO, AdcScaledIo, adc_scaled_io, ADC, Adc, ADC_SCALED_IO_CH, AdcCh, ADC_PERIPH, ADC_SCALED_IO_OWNED, ADC_SCALED_IO_REF_COUNT, 27);
channel!(ADC_DAC, AdcDac, adc_dac, ADC, Adc, ADC_DAC_CH, AdcCh, ADC_PERIPH, ADC_DAC_OWNED, ADC_DAC_REF_COUNT, 28);
// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("ADC"), description: None }
impl ::bobbin_common::gate::GateEn for Adc {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().adc() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_adc(value));
        self
    }
}

