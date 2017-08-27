#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "ADC", peripherals: [Peripheral { derived_from: None, group_name: None, name: "ADC0", address: 1073983488, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Analog-to-Digital Converter"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "ADC0", types: [], value: 39, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "ADC0_CH0", index: Some(0), description: None, signals: [Signal { name: "ADC0_DP0", types: ["ADC"], description: None }, Signal { name: "ADC0_DM0", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH1", index: Some(1), description: None, signals: [Signal { name: "ADC0_DP1", types: ["ADC"], description: None }, Signal { name: "ADC0_DM1", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH2", index: Some(2), description: None, signals: [Signal { name: "ADC0_DP2", types: ["ADC"], description: None }, Signal { name: "ADC0_DM2", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH3", index: Some(3), description: None, signals: [Signal { name: "ADC0_DP3", types: ["ADC"], description: None }, Signal { name: "ADC0_DM3", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH4", index: Some(4), description: None, signals: [Signal { name: "ADC0_SE4a", types: ["ADC"], description: None }, Signal { name: "ADC0_SE4b", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH5", index: Some(5), description: None, signals: [Signal { name: "ADC0_SE5a", types: ["ADC"], description: None }, Signal { name: "ADC0_SE5b", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH6", index: Some(6), description: None, signals: [Signal { name: "ADC0_SE6a", types: ["ADC"], description: None }, Signal { name: "ADC0_SE6b", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH7", index: Some(7), description: None, signals: [Signal { name: "ADC0_SE7a", types: ["ADC"], description: None }, Signal { name: "ADC0_SE7b", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH8", index: Some(8), description: None, signals: [Signal { name: "ADC0_SE8", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH9", index: Some(9), description: None, signals: [Signal { name: "ADC0_SE9", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH10", index: Some(10), description: None, signals: [Signal { name: "ADC0_SE10", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH11", index: Some(11), description: None, signals: [Signal { name: "ADC0_SE11", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH12", index: Some(12), description: None, signals: [Signal { name: "ADC0_SE12", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH13", index: Some(13), description: None, signals: [Signal { name: "ADC0_SE13", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH14", index: Some(14), description: None, signals: [Signal { name: "ADC0_SE14", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH15", index: Some(15), description: None, signals: [Signal { name: "ADC0_SE15", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH16", index: Some(16), description: None, signals: [Signal { name: "ADC0_SE16", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH17", index: Some(17), description: None, signals: [Signal { name: "ADC0_SE17", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH18", index: Some(18), description: None, signals: [Signal { name: "ADC0_SE18", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH19", index: Some(19), description: None, signals: [Signal { name: "ADC0_SE19", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH20", index: Some(20), description: None, signals: [Signal { name: "ADC0_SE20", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH21", index: Some(21), description: None, signals: [Signal { name: "ADC0_SE21", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH22", index: Some(22), description: None, signals: [Signal { name: "ADC0_SE22", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_CH23", index: Some(23), description: None, signals: [Signal { name: "ADC0_SE23", types: ["ADC"], description: None }], interrupts: [] }, Channel { name: "ADC0_TEMP", index: Some(26), description: None, signals: [], interrupts: [] }, Channel { name: "ADC0_BANDGAP", index: Some(27), description: None, signals: [], interrupts: [] }, Channel { name: "ADC0_REFSH", index: Some(29), description: None, signals: [], interrupts: [] }, Channel { name: "ADC0_REFSL", index: Some(30), description: None, signals: [], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::adc::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::adc::*;

periph!( ADC0, Adc0, _ADC0, AdcPeriph, 0x4003b000);

impl super::sig::Signal<super::sig::Adc0Dp0> for Adc0Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc0Dp0> for Adc0Ch0 {}
impl super::sig::Signal<super::sig::Adc0Dm0> for Adc0Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc0Dm0> for Adc0Ch0 {}
impl super::sig::Signal<super::sig::Adc0Dp1> for Adc0Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc0Dp1> for Adc0Ch1 {}
impl super::sig::Signal<super::sig::Adc0Dm1> for Adc0Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc0Dm1> for Adc0Ch1 {}
impl super::sig::Signal<super::sig::Adc0Dp2> for Adc0Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc0Dp2> for Adc0Ch2 {}
impl super::sig::Signal<super::sig::Adc0Dm2> for Adc0Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc0Dm2> for Adc0Ch2 {}
impl super::sig::Signal<super::sig::Adc0Dp3> for Adc0Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc0Dp3> for Adc0Ch3 {}
impl super::sig::Signal<super::sig::Adc0Dm3> for Adc0Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc0Dm3> for Adc0Ch3 {}
impl super::sig::Signal<super::sig::Adc0Se4a> for Adc0Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc0Se4a> for Adc0Ch4 {}
impl super::sig::Signal<super::sig::Adc0Se4b> for Adc0Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc0Se4b> for Adc0Ch4 {}
impl super::sig::Signal<super::sig::Adc0Se5a> for Adc0Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc0Se5a> for Adc0Ch5 {}
impl super::sig::Signal<super::sig::Adc0Se5b> for Adc0Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc0Se5b> for Adc0Ch5 {}
impl super::sig::Signal<super::sig::Adc0Se6a> for Adc0Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc0Se6a> for Adc0Ch6 {}
impl super::sig::Signal<super::sig::Adc0Se6b> for Adc0Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc0Se6b> for Adc0Ch6 {}
impl super::sig::Signal<super::sig::Adc0Se7a> for Adc0Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc0Se7a> for Adc0Ch7 {}
impl super::sig::Signal<super::sig::Adc0Se7b> for Adc0Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc0Se7b> for Adc0Ch7 {}
impl super::sig::Signal<super::sig::Adc0Se8> for Adc0Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc0Se8> for Adc0Ch8 {}
impl super::sig::Signal<super::sig::Adc0Se9> for Adc0Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc0Se9> for Adc0Ch9 {}
impl super::sig::Signal<super::sig::Adc0Se10> for Adc0Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc0Se10> for Adc0Ch10 {}
impl super::sig::Signal<super::sig::Adc0Se11> for Adc0Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc0Se11> for Adc0Ch11 {}
impl super::sig::Signal<super::sig::Adc0Se12> for Adc0Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc0Se12> for Adc0Ch12 {}
impl super::sig::Signal<super::sig::Adc0Se13> for Adc0Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc0Se13> for Adc0Ch13 {}
impl super::sig::Signal<super::sig::Adc0Se14> for Adc0Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc0Se14> for Adc0Ch14 {}
impl super::sig::Signal<super::sig::Adc0Se15> for Adc0Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc0Se15> for Adc0Ch15 {}
impl super::sig::Signal<super::sig::Adc0Se16> for Adc0Ch16 {}
impl super::sig::SignalAdc<super::sig::Adc0Se16> for Adc0Ch16 {}
impl super::sig::Signal<super::sig::Adc0Se17> for Adc0Ch17 {}
impl super::sig::SignalAdc<super::sig::Adc0Se17> for Adc0Ch17 {}
impl super::sig::Signal<super::sig::Adc0Se18> for Adc0Ch18 {}
impl super::sig::SignalAdc<super::sig::Adc0Se18> for Adc0Ch18 {}
impl super::sig::Signal<super::sig::Adc0Se19> for Adc0Ch19 {}
impl super::sig::SignalAdc<super::sig::Adc0Se19> for Adc0Ch19 {}
impl super::sig::Signal<super::sig::Adc0Se20> for Adc0Ch20 {}
impl super::sig::SignalAdc<super::sig::Adc0Se20> for Adc0Ch20 {}
impl super::sig::Signal<super::sig::Adc0Se21> for Adc0Ch21 {}
impl super::sig::SignalAdc<super::sig::Adc0Se21> for Adc0Ch21 {}
impl super::sig::Signal<super::sig::Adc0Se22> for Adc0Ch22 {}
impl super::sig::SignalAdc<super::sig::Adc0Se22> for Adc0Ch22 {}
impl super::sig::Signal<super::sig::Adc0Se23> for Adc0Ch23 {}
impl super::sig::SignalAdc<super::sig::Adc0Se23> for Adc0Ch23 {}


channel!(ADC0_CH0, Adc0Ch0, ADC0, Adc0, _ADC0_CH0, AdcCh, _ADC0, 0);
channel!(ADC0_CH1, Adc0Ch1, ADC0, Adc0, _ADC0_CH1, AdcCh, _ADC0, 1);
channel!(ADC0_CH2, Adc0Ch2, ADC0, Adc0, _ADC0_CH2, AdcCh, _ADC0, 2);
channel!(ADC0_CH3, Adc0Ch3, ADC0, Adc0, _ADC0_CH3, AdcCh, _ADC0, 3);
channel!(ADC0_CH4, Adc0Ch4, ADC0, Adc0, _ADC0_CH4, AdcCh, _ADC0, 4);
channel!(ADC0_CH5, Adc0Ch5, ADC0, Adc0, _ADC0_CH5, AdcCh, _ADC0, 5);
channel!(ADC0_CH6, Adc0Ch6, ADC0, Adc0, _ADC0_CH6, AdcCh, _ADC0, 6);
channel!(ADC0_CH7, Adc0Ch7, ADC0, Adc0, _ADC0_CH7, AdcCh, _ADC0, 7);
channel!(ADC0_CH8, Adc0Ch8, ADC0, Adc0, _ADC0_CH8, AdcCh, _ADC0, 8);
channel!(ADC0_CH9, Adc0Ch9, ADC0, Adc0, _ADC0_CH9, AdcCh, _ADC0, 9);
channel!(ADC0_CH10, Adc0Ch10, ADC0, Adc0, _ADC0_CH10, AdcCh, _ADC0, 10);
channel!(ADC0_CH11, Adc0Ch11, ADC0, Adc0, _ADC0_CH11, AdcCh, _ADC0, 11);
channel!(ADC0_CH12, Adc0Ch12, ADC0, Adc0, _ADC0_CH12, AdcCh, _ADC0, 12);
channel!(ADC0_CH13, Adc0Ch13, ADC0, Adc0, _ADC0_CH13, AdcCh, _ADC0, 13);
channel!(ADC0_CH14, Adc0Ch14, ADC0, Adc0, _ADC0_CH14, AdcCh, _ADC0, 14);
channel!(ADC0_CH15, Adc0Ch15, ADC0, Adc0, _ADC0_CH15, AdcCh, _ADC0, 15);
channel!(ADC0_CH16, Adc0Ch16, ADC0, Adc0, _ADC0_CH16, AdcCh, _ADC0, 16);
channel!(ADC0_CH17, Adc0Ch17, ADC0, Adc0, _ADC0_CH17, AdcCh, _ADC0, 17);
channel!(ADC0_CH18, Adc0Ch18, ADC0, Adc0, _ADC0_CH18, AdcCh, _ADC0, 18);
channel!(ADC0_CH19, Adc0Ch19, ADC0, Adc0, _ADC0_CH19, AdcCh, _ADC0, 19);
channel!(ADC0_CH20, Adc0Ch20, ADC0, Adc0, _ADC0_CH20, AdcCh, _ADC0, 20);
channel!(ADC0_CH21, Adc0Ch21, ADC0, Adc0, _ADC0_CH21, AdcCh, _ADC0, 21);
channel!(ADC0_CH22, Adc0Ch22, ADC0, Adc0, _ADC0_CH22, AdcCh, _ADC0, 22);
channel!(ADC0_CH23, Adc0Ch23, ADC0, Adc0, _ADC0_CH23, AdcCh, _ADC0, 23);
channel!(ADC0_TEMP, Adc0Temp, ADC0, Adc0, _ADC0_TEMP, AdcCh, _ADC0, 26);
channel!(ADC0_BANDGAP, Adc0Bandgap, ADC0, Adc0, _ADC0_BANDGAP, AdcCh, _ADC0, 27);
channel!(ADC0_REFSH, Adc0Refsh, ADC0, Adc0, _ADC0_REFSH, AdcCh, _ADC0, 29);
channel!(ADC0_REFSL, Adc0Refsl, ADC0, Adc0, _ADC0_REFSL, AdcCh, _ADC0, 30);
