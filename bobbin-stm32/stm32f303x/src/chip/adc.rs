#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::adc_f3::*;

periph!( ADC1, Adc1, _ADC1, AdcPeriph, 0x50000000);
periph!( ADC2, Adc2, _ADC2, AdcPeriph, 0x50000100);
periph!( ADC3, Adc3, _ADC3, AdcPeriph, 0x50000400);
periph!( ADC4, Adc4, _ADC4, AdcPeriph, 0x50000500);

impl super::sig::Signal<super::sig::Adc1In1> for Adc1Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc1In1> for Adc1Ch1 {}
impl super::sig::Signal<super::sig::Adc1In2> for Adc1Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc1In2> for Adc1Ch2 {}
impl super::sig::Signal<super::sig::Adc1In3> for Adc1Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc1In3> for Adc1Ch3 {}
impl super::sig::Signal<super::sig::Adc1In4> for Adc1Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc1In4> for Adc1Ch4 {}
impl super::sig::Signal<super::sig::Adc1In5> for Adc1Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc1In5> for Adc1Ch5 {}
impl super::sig::Signal<super::sig::Adc1In6> for Adc1Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc1In6> for Adc1Ch6 {}
impl super::sig::Signal<super::sig::Adc1In7> for Adc1Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc1In7> for Adc1Ch7 {}
impl super::sig::Signal<super::sig::Adc1In8> for Adc1Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc1In8> for Adc1Ch8 {}
impl super::sig::Signal<super::sig::Adc1In9> for Adc1Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc1In9> for Adc1Ch9 {}
impl super::sig::Signal<super::sig::Adc1In10> for Adc1Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc1In10> for Adc1Ch10 {}
impl super::sig::Signal<super::sig::Adc1In11> for Adc1Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc1In11> for Adc1Ch11 {}
impl super::sig::Signal<super::sig::Adc1In12> for Adc1Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc1In12> for Adc1Ch12 {}
impl super::sig::Signal<super::sig::Adc1In13> for Adc1Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc1In13> for Adc1Ch13 {}
impl super::sig::Signal<super::sig::Adc1In14> for Adc1Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc1In14> for Adc1Ch14 {}
impl super::sig::Signal<super::sig::Adc1In15> for Adc1Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc1In15> for Adc1Ch15 {}

impl super::sig::Signal<super::sig::Adc2In1> for Adc2Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc2In1> for Adc2Ch1 {}
impl super::sig::Signal<super::sig::Adc2In2> for Adc2Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc2In2> for Adc2Ch2 {}
impl super::sig::Signal<super::sig::Adc2In3> for Adc2Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc2In3> for Adc2Ch3 {}
impl super::sig::Signal<super::sig::Adc2In4> for Adc2Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc2In4> for Adc2Ch4 {}
impl super::sig::Signal<super::sig::Adc2In5> for Adc2Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc2In5> for Adc2Ch5 {}
impl super::sig::Signal<super::sig::Adc2In6> for Adc2Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc2In6> for Adc2Ch6 {}
impl super::sig::Signal<super::sig::Adc2In7> for Adc2Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc2In7> for Adc2Ch7 {}
impl super::sig::Signal<super::sig::Adc2In8> for Adc2Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc2In8> for Adc2Ch8 {}
impl super::sig::Signal<super::sig::Adc2In9> for Adc2Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc2In9> for Adc2Ch9 {}
impl super::sig::Signal<super::sig::Adc2In10> for Adc2Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc2In10> for Adc2Ch10 {}
impl super::sig::Signal<super::sig::Adc2In11> for Adc2Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc2In11> for Adc2Ch11 {}
impl super::sig::Signal<super::sig::Adc2In12> for Adc2Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc2In12> for Adc2Ch12 {}
impl super::sig::Signal<super::sig::Adc2In13> for Adc2Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc2In13> for Adc2Ch13 {}
impl super::sig::Signal<super::sig::Adc2In14> for Adc2Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc2In14> for Adc2Ch14 {}
impl super::sig::Signal<super::sig::Adc2In15> for Adc2Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc2In15> for Adc2Ch15 {}

impl super::sig::Signal<super::sig::Adc3In1> for Adc3Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc3In1> for Adc3Ch1 {}
impl super::sig::Signal<super::sig::Adc3In2> for Adc3Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc3In2> for Adc3Ch2 {}
impl super::sig::Signal<super::sig::Adc3In3> for Adc3Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc3In3> for Adc3Ch3 {}
impl super::sig::Signal<super::sig::Adc3In4> for Adc3Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc3In4> for Adc3Ch4 {}
impl super::sig::Signal<super::sig::Adc3In5> for Adc3Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc3In5> for Adc3Ch5 {}
impl super::sig::Signal<super::sig::Adc3In6> for Adc3Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc3In6> for Adc3Ch6 {}
impl super::sig::Signal<super::sig::Adc3In7> for Adc3Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc3In7> for Adc3Ch7 {}
impl super::sig::Signal<super::sig::Adc3In8> for Adc3Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc3In8> for Adc3Ch8 {}
impl super::sig::Signal<super::sig::Adc3In9> for Adc3Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc3In9> for Adc3Ch9 {}
impl super::sig::Signal<super::sig::Adc3In10> for Adc3Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc3In10> for Adc3Ch10 {}
impl super::sig::Signal<super::sig::Adc3In11> for Adc3Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc3In11> for Adc3Ch11 {}
impl super::sig::Signal<super::sig::Adc3In12> for Adc3Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc3In12> for Adc3Ch12 {}
impl super::sig::Signal<super::sig::Adc3In13> for Adc3Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc3In13> for Adc3Ch13 {}
impl super::sig::Signal<super::sig::Adc3In14> for Adc3Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc3In14> for Adc3Ch14 {}
impl super::sig::Signal<super::sig::Adc3In15> for Adc3Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc3In15> for Adc3Ch15 {}

impl super::sig::Signal<super::sig::Adc4In1> for Adc4Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc4In1> for Adc4Ch1 {}
impl super::sig::Signal<super::sig::Adc4In2> for Adc4Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc4In2> for Adc4Ch2 {}
impl super::sig::Signal<super::sig::Adc4In3> for Adc4Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc4In3> for Adc4Ch3 {}
impl super::sig::Signal<super::sig::Adc4In4> for Adc4Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc4In4> for Adc4Ch4 {}
impl super::sig::Signal<super::sig::Adc4In5> for Adc4Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc4In5> for Adc4Ch5 {}
impl super::sig::Signal<super::sig::Adc4In6> for Adc4Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc4In6> for Adc4Ch6 {}
impl super::sig::Signal<super::sig::Adc4In7> for Adc4Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc4In7> for Adc4Ch7 {}
impl super::sig::Signal<super::sig::Adc4In8> for Adc4Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc4In8> for Adc4Ch8 {}
impl super::sig::Signal<super::sig::Adc4In9> for Adc4Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc4In9> for Adc4Ch9 {}
impl super::sig::Signal<super::sig::Adc4In10> for Adc4Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc4In10> for Adc4Ch10 {}
impl super::sig::Signal<super::sig::Adc4In11> for Adc4Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc4In11> for Adc4Ch11 {}
impl super::sig::Signal<super::sig::Adc4In12> for Adc4Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc4In12> for Adc4Ch12 {}
impl super::sig::Signal<super::sig::Adc4In13> for Adc4Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc4In13> for Adc4Ch13 {}
impl super::sig::Signal<super::sig::Adc4In14> for Adc4Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc4In14> for Adc4Ch14 {}
impl super::sig::Signal<super::sig::Adc4In15> for Adc4Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc4In15> for Adc4Ch15 {}


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
channel!(ADC1_TEMP, Adc1Temp, ADC1, Adc1, _ADC1_TEMP, AdcCh, _ADC1, 16);
channel!(ADC1_REFINT, Adc1Refint, ADC1, Adc1, _ADC1_REFINT, AdcCh, _ADC1, 18);
channel!(ADC1_BAT, Adc1Bat, ADC1, Adc1, _ADC1_BAT, AdcCh, _ADC1, 17);
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
channel!(ADC2_REFINT, Adc2Refint, ADC2, Adc2, _ADC2_REFINT, AdcCh, _ADC2, 18);
channel!(ADC3_CH1, Adc3Ch1, ADC3, Adc3, _ADC3_CH1, AdcCh, _ADC3, 1);
channel!(ADC3_CH2, Adc3Ch2, ADC3, Adc3, _ADC3_CH2, AdcCh, _ADC3, 2);
channel!(ADC3_CH3, Adc3Ch3, ADC3, Adc3, _ADC3_CH3, AdcCh, _ADC3, 3);
channel!(ADC3_CH4, Adc3Ch4, ADC3, Adc3, _ADC3_CH4, AdcCh, _ADC3, 4);
channel!(ADC3_CH5, Adc3Ch5, ADC3, Adc3, _ADC3_CH5, AdcCh, _ADC3, 5);
channel!(ADC3_CH6, Adc3Ch6, ADC3, Adc3, _ADC3_CH6, AdcCh, _ADC3, 6);
channel!(ADC3_CH7, Adc3Ch7, ADC3, Adc3, _ADC3_CH7, AdcCh, _ADC3, 7);
channel!(ADC3_CH8, Adc3Ch8, ADC3, Adc3, _ADC3_CH8, AdcCh, _ADC3, 8);
channel!(ADC3_CH9, Adc3Ch9, ADC3, Adc3, _ADC3_CH9, AdcCh, _ADC3, 9);
channel!(ADC3_CH10, Adc3Ch10, ADC3, Adc3, _ADC3_CH10, AdcCh, _ADC3, 10);
channel!(ADC3_CH11, Adc3Ch11, ADC3, Adc3, _ADC3_CH11, AdcCh, _ADC3, 11);
channel!(ADC3_CH12, Adc3Ch12, ADC3, Adc3, _ADC3_CH12, AdcCh, _ADC3, 12);
channel!(ADC3_CH13, Adc3Ch13, ADC3, Adc3, _ADC3_CH13, AdcCh, _ADC3, 13);
channel!(ADC3_CH14, Adc3Ch14, ADC3, Adc3, _ADC3_CH14, AdcCh, _ADC3, 14);
channel!(ADC3_CH15, Adc3Ch15, ADC3, Adc3, _ADC3_CH15, AdcCh, _ADC3, 15);
channel!(ADC3_REFINT, Adc3Refint, ADC3, Adc3, _ADC3_REFINT, AdcCh, _ADC3, 18);
channel!(ADC4_CH1, Adc4Ch1, ADC4, Adc4, _ADC4_CH1, AdcCh, _ADC4, 1);
channel!(ADC4_CH2, Adc4Ch2, ADC4, Adc4, _ADC4_CH2, AdcCh, _ADC4, 2);
channel!(ADC4_CH3, Adc4Ch3, ADC4, Adc4, _ADC4_CH3, AdcCh, _ADC4, 3);
channel!(ADC4_CH4, Adc4Ch4, ADC4, Adc4, _ADC4_CH4, AdcCh, _ADC4, 4);
channel!(ADC4_CH5, Adc4Ch5, ADC4, Adc4, _ADC4_CH5, AdcCh, _ADC4, 5);
channel!(ADC4_CH6, Adc4Ch6, ADC4, Adc4, _ADC4_CH6, AdcCh, _ADC4, 6);
channel!(ADC4_CH7, Adc4Ch7, ADC4, Adc4, _ADC4_CH7, AdcCh, _ADC4, 7);
channel!(ADC4_CH8, Adc4Ch8, ADC4, Adc4, _ADC4_CH8, AdcCh, _ADC4, 8);
channel!(ADC4_CH9, Adc4Ch9, ADC4, Adc4, _ADC4_CH9, AdcCh, _ADC4, 9);
channel!(ADC4_CH10, Adc4Ch10, ADC4, Adc4, _ADC4_CH10, AdcCh, _ADC4, 10);
channel!(ADC4_CH11, Adc4Ch11, ADC4, Adc4, _ADC4_CH11, AdcCh, _ADC4, 11);
channel!(ADC4_CH12, Adc4Ch12, ADC4, Adc4, _ADC4_CH12, AdcCh, _ADC4, 12);
channel!(ADC4_CH13, Adc4Ch13, ADC4, Adc4, _ADC4_CH13, AdcCh, _ADC4, 13);
channel!(ADC4_CH14, Adc4Ch14, ADC4, Adc4, _ADC4_CH14, AdcCh, _ADC4, 14);
channel!(ADC4_CH15, Adc4Ch15, ADC4, Adc4, _ADC4_CH15, AdcCh, _ADC4, 15);
channel!(ADC4_REFINT, Adc4Refint, ADC4, Adc4, _ADC4_REFINT, AdcCh, _ADC4, 18);

