#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::adc::*;

// //! Analog-to-Digital Converter

periph!( ADC, Adc, ADC_PERIPH, AdcPeriph, 0x42004000, 0x00, 0x07);

channel!(ADC_CH0, AdcCh0, ADC, Adc, ADC_CH0_CH, AdcCh, ADC_PERIPH, 0);
channel!(ADC_CH1, AdcCh1, ADC, Adc, ADC_CH1_CH, AdcCh, ADC_PERIPH, 1);
channel!(ADC_CH2, AdcCh2, ADC, Adc, ADC_CH2_CH, AdcCh, ADC_PERIPH, 2);
channel!(ADC_CH3, AdcCh3, ADC, Adc, ADC_CH3_CH, AdcCh, ADC_PERIPH, 3);
channel!(ADC_CH4, AdcCh4, ADC, Adc, ADC_CH4_CH, AdcCh, ADC_PERIPH, 4);
channel!(ADC_CH5, AdcCh5, ADC, Adc, ADC_CH5_CH, AdcCh, ADC_PERIPH, 5);
channel!(ADC_CH6, AdcCh6, ADC, Adc, ADC_CH6_CH, AdcCh, ADC_PERIPH, 6);
channel!(ADC_CH7, AdcCh7, ADC, Adc, ADC_CH7_CH, AdcCh, ADC_PERIPH, 7);
channel!(ADC_CH8, AdcCh8, ADC, Adc, ADC_CH8_CH, AdcCh, ADC_PERIPH, 8);
channel!(ADC_CH9, AdcCh9, ADC, Adc, ADC_CH9_CH, AdcCh, ADC_PERIPH, 9);
channel!(ADC_CH10, AdcCh10, ADC, Adc, ADC_CH10_CH, AdcCh, ADC_PERIPH, 10);
channel!(ADC_CH11, AdcCh11, ADC, Adc, ADC_CH11_CH, AdcCh, ADC_PERIPH, 11);
channel!(ADC_CH12, AdcCh12, ADC, Adc, ADC_CH12_CH, AdcCh, ADC_PERIPH, 12);
channel!(ADC_CH13, AdcCh13, ADC, Adc, ADC_CH13_CH, AdcCh, ADC_PERIPH, 13);
channel!(ADC_CH14, AdcCh14, ADC, Adc, ADC_CH14_CH, AdcCh, ADC_PERIPH, 14);
channel!(ADC_CH15, AdcCh15, ADC, Adc, ADC_CH15_CH, AdcCh, ADC_PERIPH, 15);
channel!(ADC_CH16, AdcCh16, ADC, Adc, ADC_CH16_CH, AdcCh, ADC_PERIPH, 16);
channel!(ADC_CH17, AdcCh17, ADC, Adc, ADC_CH17_CH, AdcCh, ADC_PERIPH, 17);
channel!(ADC_CH18, AdcCh18, ADC, Adc, ADC_CH18_CH, AdcCh, ADC_PERIPH, 18);
channel!(ADC_CH19, AdcCh19, ADC, Adc, ADC_CH19_CH, AdcCh, ADC_PERIPH, 19);
channel!(ADC_TEMP, AdcTemp, ADC, Adc, ADC_TEMP_CH, AdcCh, ADC_PERIPH, 24);
channel!(ADC_BANDGAP, AdcBandgap, ADC, Adc, ADC_BANDGAP_CH, AdcCh, ADC_PERIPH, 25);
channel!(ADC_SCALED_CORE, AdcScaledCore, ADC, Adc, ADC_SCALED_CORE_CH, AdcCh, ADC_PERIPH, 26);
channel!(ADC_SCALED_IO, AdcScaledIo, ADC, Adc, ADC_SCALED_IO_CH, AdcCh, ADC_PERIPH, 27);
channel!(ADC_DAC, AdcDac, ADC, Adc, ADC_DAC_CH, AdcCh, ADC_PERIPH, 28);
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

