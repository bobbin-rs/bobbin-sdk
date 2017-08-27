#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "FTM", peripherals: [Peripheral { derived_from: None, group_name: None, name: "FTM0", address: 1073971200, size: None, access: None, reset_value: None, reset_mask: None, description: Some("FlexTimer Module"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "FTM0", types: ["FTM"], value: 42, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "FTM0_CH0", index: Some(0), description: None, signals: [Signal { name: "FTM0_CH0", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM0_CH1", index: Some(1), description: None, signals: [Signal { name: "FTM0_CH1", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM0_CH2", index: Some(2), description: None, signals: [Signal { name: "FTM0_CH2", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM0_CH3", index: Some(3), description: None, signals: [Signal { name: "FTM0_CH3", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM0_CH4", index: Some(4), description: None, signals: [Signal { name: "FTM0_CH4", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM0_CH5", index: Some(5), description: None, signals: [Signal { name: "FTM0_CH5", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM0_CH6", index: Some(6), description: None, signals: [Signal { name: "FTM0_CH6", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM0_CH7", index: Some(7), description: None, signals: [Signal { name: "FTM0_CH7", types: ["FTM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "FTM1", address: 1073975296, size: None, access: None, reset_value: None, reset_mask: None, description: Some("FlexTimer Module"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "FTM1", types: ["FTM"], value: 43, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "FTM1_CH0", index: Some(0), description: None, signals: [Signal { name: "FTM1_CH0", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM1_CH1", index: Some(1), description: None, signals: [Signal { name: "FTM1_CH1", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM1_CH2", index: Some(2), description: None, signals: [Signal { name: "FTM1_CH2", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM1_CH3", index: Some(3), description: None, signals: [Signal { name: "FTM1_CH3", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM1_CH4", index: Some(4), description: None, signals: [Signal { name: "FTM1_CH4", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM1_CH5", index: Some(5), description: None, signals: [Signal { name: "FTM1_CH5", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM1_CH6", index: Some(6), description: None, signals: [Signal { name: "FTM1_CH6", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM1_CH7", index: Some(7), description: None, signals: [Signal { name: "FTM1_CH7", types: ["FTM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "FTM2", address: 1073979392, size: None, access: None, reset_value: None, reset_mask: None, description: Some("FlexTimer Module"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "FTM2", types: ["FTM"], value: 44, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "FTM2_CH0", index: Some(0), description: None, signals: [Signal { name: "FTM2_CH0", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM2_CH1", index: Some(1), description: None, signals: [Signal { name: "FTM2_CH1", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM2_CH2", index: Some(2), description: None, signals: [Signal { name: "FTM2_CH2", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM2_CH3", index: Some(3), description: None, signals: [Signal { name: "FTM2_CH3", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM2_CH4", index: Some(4), description: None, signals: [Signal { name: "FTM2_CH4", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM2_CH5", index: Some(5), description: None, signals: [Signal { name: "FTM2_CH5", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM2_CH6", index: Some(6), description: None, signals: [Signal { name: "FTM2_CH6", types: ["FTM"], description: None }], interrupts: [] }, Channel { name: "FTM2_CH7", index: Some(7), description: None, signals: [Signal { name: "FTM2_CH7", types: ["FTM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::ftm::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::ftm::*;

periph!( FTM0, Ftm0, _FTM0, FtmPeriph, 0x40038000);
periph!( FTM1, Ftm1, _FTM1, FtmPeriph, 0x40039000);
periph!( FTM2, Ftm2, _FTM2, FtmPeriph, 0x4003a000);

impl super::sig::Signal<super::sig::Ftm0Ch0> for Ftm0Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch0> for Ftm0Ch0 {}
impl super::sig::Signal<super::sig::Ftm0Ch1> for Ftm0Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch1> for Ftm0Ch1 {}
impl super::sig::Signal<super::sig::Ftm0Ch2> for Ftm0Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch2> for Ftm0Ch2 {}
impl super::sig::Signal<super::sig::Ftm0Ch3> for Ftm0Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch3> for Ftm0Ch3 {}
impl super::sig::Signal<super::sig::Ftm0Ch4> for Ftm0Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch4> for Ftm0Ch4 {}
impl super::sig::Signal<super::sig::Ftm0Ch5> for Ftm0Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch5> for Ftm0Ch5 {}
impl super::sig::Signal<super::sig::Ftm0Ch6> for Ftm0Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch6> for Ftm0Ch6 {}
impl super::sig::Signal<super::sig::Ftm0Ch7> for Ftm0Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch7> for Ftm0Ch7 {}

impl super::sig::Signal<super::sig::Ftm1Ch0> for Ftm1Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch0> for Ftm1Ch0 {}
impl super::sig::Signal<super::sig::Ftm1Ch1> for Ftm1Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch1> for Ftm1Ch1 {}
impl super::sig::Signal<super::sig::Ftm1Ch2> for Ftm1Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch2> for Ftm1Ch2 {}
impl super::sig::Signal<super::sig::Ftm1Ch3> for Ftm1Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch3> for Ftm1Ch3 {}
impl super::sig::Signal<super::sig::Ftm1Ch4> for Ftm1Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch4> for Ftm1Ch4 {}
impl super::sig::Signal<super::sig::Ftm1Ch5> for Ftm1Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch5> for Ftm1Ch5 {}
impl super::sig::Signal<super::sig::Ftm1Ch6> for Ftm1Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch6> for Ftm1Ch6 {}
impl super::sig::Signal<super::sig::Ftm1Ch7> for Ftm1Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch7> for Ftm1Ch7 {}

impl super::sig::Signal<super::sig::Ftm2Ch0> for Ftm2Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch0> for Ftm2Ch0 {}
impl super::sig::Signal<super::sig::Ftm2Ch1> for Ftm2Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch1> for Ftm2Ch1 {}
impl super::sig::Signal<super::sig::Ftm2Ch2> for Ftm2Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch2> for Ftm2Ch2 {}
impl super::sig::Signal<super::sig::Ftm2Ch3> for Ftm2Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch3> for Ftm2Ch3 {}
impl super::sig::Signal<super::sig::Ftm2Ch4> for Ftm2Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch4> for Ftm2Ch4 {}
impl super::sig::Signal<super::sig::Ftm2Ch5> for Ftm2Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch5> for Ftm2Ch5 {}
impl super::sig::Signal<super::sig::Ftm2Ch6> for Ftm2Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch6> for Ftm2Ch6 {}
impl super::sig::Signal<super::sig::Ftm2Ch7> for Ftm2Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch7> for Ftm2Ch7 {}


channel!(FTM0_CH0, Ftm0Ch0, FTM0, Ftm0, _FTM0_CH0, FtmCh, _FTM0, 0);
channel!(FTM0_CH1, Ftm0Ch1, FTM0, Ftm0, _FTM0_CH1, FtmCh, _FTM0, 1);
channel!(FTM0_CH2, Ftm0Ch2, FTM0, Ftm0, _FTM0_CH2, FtmCh, _FTM0, 2);
channel!(FTM0_CH3, Ftm0Ch3, FTM0, Ftm0, _FTM0_CH3, FtmCh, _FTM0, 3);
channel!(FTM0_CH4, Ftm0Ch4, FTM0, Ftm0, _FTM0_CH4, FtmCh, _FTM0, 4);
channel!(FTM0_CH5, Ftm0Ch5, FTM0, Ftm0, _FTM0_CH5, FtmCh, _FTM0, 5);
channel!(FTM0_CH6, Ftm0Ch6, FTM0, Ftm0, _FTM0_CH6, FtmCh, _FTM0, 6);
channel!(FTM0_CH7, Ftm0Ch7, FTM0, Ftm0, _FTM0_CH7, FtmCh, _FTM0, 7);
channel!(FTM1_CH0, Ftm1Ch0, FTM1, Ftm1, _FTM1_CH0, FtmCh, _FTM1, 0);
channel!(FTM1_CH1, Ftm1Ch1, FTM1, Ftm1, _FTM1_CH1, FtmCh, _FTM1, 1);
channel!(FTM1_CH2, Ftm1Ch2, FTM1, Ftm1, _FTM1_CH2, FtmCh, _FTM1, 2);
channel!(FTM1_CH3, Ftm1Ch3, FTM1, Ftm1, _FTM1_CH3, FtmCh, _FTM1, 3);
channel!(FTM1_CH4, Ftm1Ch4, FTM1, Ftm1, _FTM1_CH4, FtmCh, _FTM1, 4);
channel!(FTM1_CH5, Ftm1Ch5, FTM1, Ftm1, _FTM1_CH5, FtmCh, _FTM1, 5);
channel!(FTM1_CH6, Ftm1Ch6, FTM1, Ftm1, _FTM1_CH6, FtmCh, _FTM1, 6);
channel!(FTM1_CH7, Ftm1Ch7, FTM1, Ftm1, _FTM1_CH7, FtmCh, _FTM1, 7);
channel!(FTM2_CH0, Ftm2Ch0, FTM2, Ftm2, _FTM2_CH0, FtmCh, _FTM2, 0);
channel!(FTM2_CH1, Ftm2Ch1, FTM2, Ftm2, _FTM2_CH1, FtmCh, _FTM2, 1);
channel!(FTM2_CH2, Ftm2Ch2, FTM2, Ftm2, _FTM2_CH2, FtmCh, _FTM2, 2);
channel!(FTM2_CH3, Ftm2Ch3, FTM2, Ftm2, _FTM2_CH3, FtmCh, _FTM2, 3);
channel!(FTM2_CH4, Ftm2Ch4, FTM2, Ftm2, _FTM2_CH4, FtmCh, _FTM2, 4);
channel!(FTM2_CH5, Ftm2Ch5, FTM2, Ftm2, _FTM2_CH5, FtmCh, _FTM2, 5);
channel!(FTM2_CH6, Ftm2Ch6, FTM2, Ftm2, _FTM2_CH6, FtmCh, _FTM2, 6);
channel!(FTM2_CH7, Ftm2Ch7, FTM2, Ftm2, _FTM2_CH7, FtmCh, _FTM2, 7);
