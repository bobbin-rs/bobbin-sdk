#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::adc::*;

periph!( ADC1, Adc1, ADC1_PERIPH, AdcPeriph, 0x50000000, 0x19);

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
channel!(ADC1_REFINT, Adc1Refint, ADC1, Adc1, ADC1_REFINT_CH, AdcCh, ADC1_PERIPH, 18);
channel!(ADC1_BAT, Adc1Bat, ADC1, Adc1, ADC1_BAT_CH, AdcCh, ADC1_PERIPH, 17);
