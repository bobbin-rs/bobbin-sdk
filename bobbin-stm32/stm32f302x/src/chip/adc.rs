#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "ADC", peripherals: [Peripheral { derived_from: None, group_name: None, name: "ADC1", address: 1342177280, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Analog-to-Digital Converter"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "ADC1", types: [], value: 18, description: Some("ADC1 and ADC2 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "ADC1_CH1", index: Some(1), description: None, signals: [Signal { name: "ADC1_IN1", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH2", index: Some(2), description: None, signals: [Signal { name: "ADC1_IN2", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH3", index: Some(3), description: None, signals: [Signal { name: "ADC1_IN3", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH4", index: Some(4), description: None, signals: [Signal { name: "ADC1_IN4", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH5", index: Some(5), description: None, signals: [Signal { name: "ADC1_IN5", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH6", index: Some(6), description: None, signals: [Signal { name: "ADC1_IN6", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH7", index: Some(7), description: None, signals: [Signal { name: "ADC1_IN7", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH8", index: Some(8), description: None, signals: [Signal { name: "ADC1_IN8", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH9", index: Some(9), description: None, signals: [Signal { name: "ADC1_IN9", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH10", index: Some(10), description: None, signals: [Signal { name: "ADC1_IN10", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH11", index: Some(11), description: None, signals: [Signal { name: "ADC1_IN11", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH12", index: Some(12), description: None, signals: [Signal { name: "ADC1_IN12", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH13", index: Some(13), description: None, signals: [Signal { name: "ADC1_IN13", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH14", index: Some(14), description: None, signals: [Signal { name: "ADC1_IN14", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_CH15", index: Some(15), description: None, signals: [Signal { name: "ADC1_IN15", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC1_TEMP", index: Some(16), description: None, signals: [], interrupts: [] }, Channel { name: "ADC1_REFINT", index: Some(18), description: None, signals: [], interrupts: [] }, Channel { name: "ADC1_BAT", index: Some(17), description: None, signals: [], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::adc_f3::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::adc_f3::*;

periph!( ADC1, Adc1, _ADC1, AdcPeriph, 0x50000000);

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
