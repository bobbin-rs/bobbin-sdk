#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::adc::*;

periph!( ADC, Adc, ADC_PERIPH, AdcPeriph, 0x50040000, 0x14);

channel!(ADC1_CH1, Adc1Ch1, ADC, Adc, ADC1_CH1_CH, AdcCh, ADC_PERIPH, 1);
channel!(ADC1_CH2, Adc1Ch2, ADC, Adc, ADC1_CH2_CH, AdcCh, ADC_PERIPH, 2);
channel!(ADC1_CH3, Adc1Ch3, ADC, Adc, ADC1_CH3_CH, AdcCh, ADC_PERIPH, 3);
channel!(ADC1_CH4, Adc1Ch4, ADC, Adc, ADC1_CH4_CH, AdcCh, ADC_PERIPH, 4);
channel!(ADC1_CH5, Adc1Ch5, ADC, Adc, ADC1_CH5_CH, AdcCh, ADC_PERIPH, 5);
channel!(ADC1_CH6, Adc1Ch6, ADC, Adc, ADC1_CH6_CH, AdcCh, ADC_PERIPH, 6);
channel!(ADC1_CH7, Adc1Ch7, ADC, Adc, ADC1_CH7_CH, AdcCh, ADC_PERIPH, 7);
channel!(ADC1_CH8, Adc1Ch8, ADC, Adc, ADC1_CH8_CH, AdcCh, ADC_PERIPH, 8);
channel!(ADC1_CH9, Adc1Ch9, ADC, Adc, ADC1_CH9_CH, AdcCh, ADC_PERIPH, 9);
channel!(ADC1_CH10, Adc1Ch10, ADC, Adc, ADC1_CH10_CH, AdcCh, ADC_PERIPH, 10);
channel!(ADC1_CH11, Adc1Ch11, ADC, Adc, ADC1_CH11_CH, AdcCh, ADC_PERIPH, 11);
channel!(ADC1_CH12, Adc1Ch12, ADC, Adc, ADC1_CH12_CH, AdcCh, ADC_PERIPH, 12);
channel!(ADC1_CH13, Adc1Ch13, ADC, Adc, ADC1_CH13_CH, AdcCh, ADC_PERIPH, 13);
channel!(ADC1_CH14, Adc1Ch14, ADC, Adc, ADC1_CH14_CH, AdcCh, ADC_PERIPH, 14);
channel!(ADC1_CH15, Adc1Ch15, ADC, Adc, ADC1_CH15_CH, AdcCh, ADC_PERIPH, 15);
channel!(ADC1_TEMP, Adc1Temp, ADC, Adc, ADC1_TEMP_CH, AdcCh, ADC_PERIPH, 17);
channel!(ADC1_REFINT, Adc1Refint, ADC, Adc, ADC1_REFINT_CH, AdcCh, ADC_PERIPH, 0);
channel!(ADC1_BAT, Adc1Bat, ADC, Adc, ADC1_BAT_CH, AdcCh, ADC_PERIPH, 18);
