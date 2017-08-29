#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::adc_f1::*;

periph!( ADC1, Adc1, _ADC1, AdcPeriph, 0x40012400);
periph!( ADC2, Adc2, _ADC2, AdcPeriph, 0x40012800);




channel!(ADC1_CH0, Adc1Ch0, ADC1, Adc1, _ADC1_CH0, AdcCh, _ADC1, 0);
channel!(ADC1_CH1, Adc1Ch1, ADC1, Adc1, _ADC1_CH1, AdcCh, _ADC1, 1);
channel!(ADC1_CH2, Adc1Ch2, ADC1, Adc1, _ADC1_CH2, AdcCh, _ADC1, 2);
channel!(ADC1_CH3, Adc1Ch3, ADC1, Adc1, _ADC1_CH3, AdcCh, _ADC1, 3);
channel!(ADC1_CH4, Adc1Ch4, ADC1, Adc1, _ADC1_CH4, AdcCh, _ADC1, 4);
channel!(ADC1_CH5, Adc1Ch5, ADC1, Adc1, _ADC1_CH5, AdcCh, _ADC1, 5);
channel!(ADC1_CH6, Adc1Ch6, ADC1, Adc1, _ADC1_CH6, AdcCh, _ADC1, 6);
channel!(ADC1_CH7, Adc1Ch7, ADC1, Adc1, _ADC1_CH7, AdcCh, _ADC1, 7);
channel!(ADC1_CH8, Adc1Ch8, ADC1, Adc1, _ADC1_CH8, AdcCh, _ADC1, 8);
channel!(ADC1_CH9, Adc1Ch9, ADC1, Adc1, _ADC1_CH9, AdcCh, _ADC1, 9);
channel!(ADC1_CH10, Adc1Ch10, ADC1, Adc1, _ADC1_CH10, AdcCh, _ADC1, 10);
channel!(ADC1_CH11, Adc1Ch11, ADC1, Adc1, _ADC1_CH11, AdcCh, _ADC1, 11);
channel!(ADC1_CH12, Adc1Ch12, ADC1, Adc1, _ADC1_CH12, AdcCh, _ADC1, 12);
channel!(ADC1_CH13, Adc1Ch13, ADC1, Adc1, _ADC1_CH13, AdcCh, _ADC1, 13);
channel!(ADC1_CH14, Adc1Ch14, ADC1, Adc1, _ADC1_CH14, AdcCh, _ADC1, 14);
channel!(ADC1_CH15, Adc1Ch15, ADC1, Adc1, _ADC1_CH15, AdcCh, _ADC1, 15);
channel!(ADC2_CH0, Adc2Ch0, ADC2, Adc2, _ADC2_CH0, AdcCh, _ADC2, 0);
channel!(ADC2_CH1, Adc2Ch1, ADC2, Adc2, _ADC2_CH1, AdcCh, _ADC2, 1);
channel!(ADC2_CH2, Adc2Ch2, ADC2, Adc2, _ADC2_CH2, AdcCh, _ADC2, 2);
channel!(ADC2_CH3, Adc2Ch3, ADC2, Adc2, _ADC2_CH3, AdcCh, _ADC2, 3);
channel!(ADC2_CH4, Adc2Ch4, ADC2, Adc2, _ADC2_CH4, AdcCh, _ADC2, 4);
channel!(ADC2_CH5, Adc2Ch5, ADC2, Adc2, _ADC2_CH5, AdcCh, _ADC2, 5);
channel!(ADC2_CH6, Adc2Ch6, ADC2, Adc2, _ADC2_CH6, AdcCh, _ADC2, 6);
channel!(ADC2_CH7, Adc2Ch7, ADC2, Adc2, _ADC2_CH7, AdcCh, _ADC2, 7);
channel!(ADC2_CH8, Adc2Ch8, ADC2, Adc2, _ADC2_CH8, AdcCh, _ADC2, 8);
channel!(ADC2_CH9, Adc2Ch9, ADC2, Adc2, _ADC2_CH9, AdcCh, _ADC2, 9);
channel!(ADC2_CH10, Adc2Ch10, ADC2, Adc2, _ADC2_CH10, AdcCh, _ADC2, 10);
channel!(ADC2_CH11, Adc2Ch11, ADC2, Adc2, _ADC2_CH11, AdcCh, _ADC2, 11);
channel!(ADC2_CH12, Adc2Ch12, ADC2, Adc2, _ADC2_CH12, AdcCh, _ADC2, 12);
channel!(ADC2_CH13, Adc2Ch13, ADC2, Adc2, _ADC2_CH13, AdcCh, _ADC2, 13);
channel!(ADC2_CH14, Adc2Ch14, ADC2, Adc2, _ADC2_CH14, AdcCh, _ADC2, 14);
channel!(ADC2_CH15, Adc2Ch15, ADC2, Adc2, _ADC2_CH15, AdcCh, _ADC2, 15);

