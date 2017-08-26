#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::adc_l0::*;

periph!(AdcPeriph, ADC1, Adc1, 0x40012400);

impl super::sig::Signal<super::sig::Adc1In0> for Adc1Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc1In0> for Adc1Ch0 {}
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
channel!(ADC1_REFINT, Adc1Refint, ADC1, Adc1, 17);
channel!(ADC1_TEMP, Adc1Temp, ADC1, Adc1, 18);
