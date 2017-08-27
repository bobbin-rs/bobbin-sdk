#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::adc_f1::*;

periph!(_ADC1, AdcPeriph, ADC1, Adc1, 0x40012400);
periph!(_ADC2, AdcPeriph, ADC2, Adc2, 0x40012800);




channel!(ADC1_CH0, Adc1Ch0, ADC1, Adc1, 0);
channel!(ADC1_CH1, Adc1Ch1, ADC1, Adc1, 1);
channel!(ADC1_CH2, Adc1Ch2, ADC1, Adc1, 2);
channel!(ADC1_CH3, Adc1Ch3, ADC1, Adc1, 3);
channel!(ADC1_CH4, Adc1Ch4, ADC1, Adc1, 4);
channel!(ADC1_CH5, Adc1Ch5, ADC1, Adc1, 5);
channel!(ADC1_CH6, Adc1Ch6, ADC1, Adc1, 6);
channel!(ADC1_CH7, Adc1Ch7, ADC1, Adc1, 7);
channel!(ADC1_CH8, Adc1Ch8, ADC1, Adc1, 8);
channel!(ADC1_CH9, Adc1Ch9, ADC1, Adc1, 9);
channel!(ADC1_CH10, Adc1Ch10, ADC1, Adc1, 10);
channel!(ADC1_CH11, Adc1Ch11, ADC1, Adc1, 11);
channel!(ADC1_CH12, Adc1Ch12, ADC1, Adc1, 12);
channel!(ADC1_CH13, Adc1Ch13, ADC1, Adc1, 13);
channel!(ADC1_CH14, Adc1Ch14, ADC1, Adc1, 14);
channel!(ADC1_CH15, Adc1Ch15, ADC1, Adc1, 15);
channel!(ADC2_CH0, Adc2Ch0, ADC2, Adc2, 0);
channel!(ADC2_CH1, Adc2Ch1, ADC2, Adc2, 1);
channel!(ADC2_CH2, Adc2Ch2, ADC2, Adc2, 2);
channel!(ADC2_CH3, Adc2Ch3, ADC2, Adc2, 3);
channel!(ADC2_CH4, Adc2Ch4, ADC2, Adc2, 4);
channel!(ADC2_CH5, Adc2Ch5, ADC2, Adc2, 5);
channel!(ADC2_CH6, Adc2Ch6, ADC2, Adc2, 6);
channel!(ADC2_CH7, Adc2Ch7, ADC2, Adc2, 7);
channel!(ADC2_CH8, Adc2Ch8, ADC2, Adc2, 8);
channel!(ADC2_CH9, Adc2Ch9, ADC2, Adc2, 9);
channel!(ADC2_CH10, Adc2Ch10, ADC2, Adc2, 10);
channel!(ADC2_CH11, Adc2Ch11, ADC2, Adc2, 11);
channel!(ADC2_CH12, Adc2Ch12, ADC2, Adc2, 12);
channel!(ADC2_CH13, Adc2Ch13, ADC2, Adc2, 13);
channel!(ADC2_CH14, Adc2Ch14, ADC2, Adc2, 14);
channel!(ADC2_CH15, Adc2Ch15, ADC2, Adc2, 15);
