#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "TIM_ADV", peripherals: [Peripheral { derived_from: None, group_name: None, name: "TIM1", address: 1073807360, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Advanced-timers"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM1_BRK", types: ["BRK"], value: 24, description: Some("TIM1 Break interrupt") }, Interrupt { name: "TIM1_UP", types: ["UP"], value: 25, description: Some("TIM1 Update interrupt") }, Interrupt { name: "TIM1_TRG_COM", types: ["TRG_COM"], value: 26, description: Some("TIM1 Trigger and Commutation interrupts") }, Interrupt { name: "TIM1_CC", types: ["CC"], value: 27, description: Some("TIM1 Capture Compare interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TIM1_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM1_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM1_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM1_CH2", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM1_CH3", index: Some(2), description: None, signals: [Signal { name: "TIM1_CH3", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM1_CH4", index: Some(3), description: None, signals: [Signal { name: "TIM1_CH4", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM8", address: 1073808384, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM8_BRK", types: ["BRK"], value: 43, description: Some("TIM8 Break interrupt and TIM12 global interrupt") }, Interrupt { name: "TIM8_UP", types: ["UP"], value: 44, description: Some("TIM8 Update interrupt and TIM13 global interrupt") }, Interrupt { name: "TIM8_TRG_COM", types: ["TRG_COM"], value: 45, description: Some("TIM8 Trigger and Commutation interrupts and TIM14 global interrupt") }, Interrupt { name: "TIM8_CC", types: ["CC"], value: 46, description: Some("TIM8 Capture Compare interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TIM8_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM8_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM8_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM8_CH2", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM8_CH3", index: Some(2), description: None, signals: [Signal { name: "TIM8_CH3", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM8_CH4", index: Some(3), description: None, signals: [Signal { name: "TIM8_CH4", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::tim_adv::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::tim_adv::*;

periph!( TIM1, Tim1, _TIM1, TimAdvPeriph, 0x40010000);
periph!( TIM8, Tim8, _TIM8, TimAdvPeriph, 0x40010400);

impl super::sig::Signal<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::SignalTim<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::Signal<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::SignalTim<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::Signal<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::SignalTim<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::Signal<super::sig::Tim1Ch4> for Tim1Ch4 {}
impl super::sig::SignalTim<super::sig::Tim1Ch4> for Tim1Ch4 {}

impl super::sig::Signal<super::sig::Tim8Ch1> for Tim8Ch1 {}
impl super::sig::SignalTim<super::sig::Tim8Ch1> for Tim8Ch1 {}
impl super::sig::Signal<super::sig::Tim8Ch2> for Tim8Ch2 {}
impl super::sig::SignalTim<super::sig::Tim8Ch2> for Tim8Ch2 {}
impl super::sig::Signal<super::sig::Tim8Ch3> for Tim8Ch3 {}
impl super::sig::SignalTim<super::sig::Tim8Ch3> for Tim8Ch3 {}
impl super::sig::Signal<super::sig::Tim8Ch4> for Tim8Ch4 {}
impl super::sig::SignalTim<super::sig::Tim8Ch4> for Tim8Ch4 {}


channel!(TIM1_CH1, Tim1Ch1, TIM1, Tim1, _TIM1_CH1, TimAdvCh, _TIM1, 0);
channel!(TIM1_CH2, Tim1Ch2, TIM1, Tim1, _TIM1_CH2, TimAdvCh, _TIM1, 1);
channel!(TIM1_CH3, Tim1Ch3, TIM1, Tim1, _TIM1_CH3, TimAdvCh, _TIM1, 2);
channel!(TIM1_CH4, Tim1Ch4, TIM1, Tim1, _TIM1_CH4, TimAdvCh, _TIM1, 3);
channel!(TIM8_CH1, Tim8Ch1, TIM8, Tim8, _TIM8_CH1, TimAdvCh, _TIM8, 0);
channel!(TIM8_CH2, Tim8Ch2, TIM8, Tim8, _TIM8_CH2, TimAdvCh, _TIM8, 1);
channel!(TIM8_CH3, Tim8Ch3, TIM8, Tim8, _TIM8_CH3, TimAdvCh, _TIM8, 2);
channel!(TIM8_CH4, Tim8Ch4, TIM8, Tim8, _TIM8_CH4, TimAdvCh, _TIM8, 3);
