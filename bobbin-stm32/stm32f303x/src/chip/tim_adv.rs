#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "TIM_ADV", peripherals: [Peripheral { derived_from: None, group_name: None, name: "TIM1", address: 1073818624, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Advanced timer"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM1_BRK", types: [], value: 24, description: Some("TIM1 break interrupt") }, Interrupt { name: "TIM1_UP", types: [], value: 25, description: Some("TIM1 update interrupt") }, Interrupt { name: "TIM1_TRG_COM", types: [], value: 26, description: Some("TIM1 trigger and commutation interrupt") }, Interrupt { name: "TIM1_CC", types: [], value: 27, description: Some("TIM1 capture compare interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TIM1_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM1_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM1_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM1_CH2", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM1_CH3", index: Some(2), description: None, signals: [Signal { name: "TIM1_CH3", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM1_CH4", index: Some(3), description: None, signals: [Signal { name: "TIM1_CH4", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM8", address: 1073820672, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Advanced-timers"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM8_BRK", types: [], value: 43, description: Some("TIM8 break interrupt") }, Interrupt { name: "TIM8_UP", types: [], value: 44, description: Some("TIM8 update interrupt") }, Interrupt { name: "TIM8_TRG_COM", types: [], value: 45, description: Some("TIM8 Trigger and commutation interrupts") }, Interrupt { name: "TIM8_CC", types: [], value: 46, description: Some("TIM8 capture compare interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TIM8_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM8_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM8_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM8_CH2", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM8_CH3", index: Some(2), description: None, signals: [Signal { name: "TIM8_CH3", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM8_CH4", index: Some(3), description: None, signals: [Signal { name: "TIM8_CH4", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM20", address: 1073827840, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM20_BRK", types: [], value: 77, description: Some("TIM20 break interrupt") }, Interrupt { name: "TIM20_UP", types: [], value: 78, description: Some("TIM20 update interrupt") }, Interrupt { name: "TIM20_TRG_COM", types: [], value: 79, description: Some("TIM20 Trigger and commutation interrupts") }, Interrupt { name: "TIM20_CC", types: [], value: 80, description: Some("TIM20 capture compare interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TIM20_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM20_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM20_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM20_CH2", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM20_CH3", index: Some(2), description: None, signals: [Signal { name: "TIM20_CH3", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM20_CH4", index: Some(3), description: None, signals: [Signal { name: "TIM20_CH4", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::tim_adv::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::tim_adv::*;

periph!( TIM1, Tim1, _TIM1, TimAdvPeriph, 0x40012c00);
periph!( TIM8, Tim8, _TIM8, TimAdvPeriph, 0x40013400);
periph!( TIM20, Tim20, _TIM20, TimAdvPeriph, 0x40015000);

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

impl super::sig::Signal<super::sig::Tim20Ch1> for Tim20Ch1 {}
impl super::sig::SignalTim<super::sig::Tim20Ch1> for Tim20Ch1 {}
impl super::sig::Signal<super::sig::Tim20Ch2> for Tim20Ch2 {}
impl super::sig::SignalTim<super::sig::Tim20Ch2> for Tim20Ch2 {}
impl super::sig::Signal<super::sig::Tim20Ch3> for Tim20Ch3 {}
impl super::sig::SignalTim<super::sig::Tim20Ch3> for Tim20Ch3 {}
impl super::sig::Signal<super::sig::Tim20Ch4> for Tim20Ch4 {}
impl super::sig::SignalTim<super::sig::Tim20Ch4> for Tim20Ch4 {}


channel!(TIM1_CH1, Tim1Ch1, TIM1, Tim1, _TIM1_CH1, TimAdvCh, _TIM1, 0);
channel!(TIM1_CH2, Tim1Ch2, TIM1, Tim1, _TIM1_CH2, TimAdvCh, _TIM1, 1);
channel!(TIM1_CH3, Tim1Ch3, TIM1, Tim1, _TIM1_CH3, TimAdvCh, _TIM1, 2);
channel!(TIM1_CH4, Tim1Ch4, TIM1, Tim1, _TIM1_CH4, TimAdvCh, _TIM1, 3);
channel!(TIM8_CH1, Tim8Ch1, TIM8, Tim8, _TIM8_CH1, TimAdvCh, _TIM8, 0);
channel!(TIM8_CH2, Tim8Ch2, TIM8, Tim8, _TIM8_CH2, TimAdvCh, _TIM8, 1);
channel!(TIM8_CH3, Tim8Ch3, TIM8, Tim8, _TIM8_CH3, TimAdvCh, _TIM8, 2);
channel!(TIM8_CH4, Tim8Ch4, TIM8, Tim8, _TIM8_CH4, TimAdvCh, _TIM8, 3);
channel!(TIM20_CH1, Tim20Ch1, TIM20, Tim20, _TIM20_CH1, TimAdvCh, _TIM20, 0);
channel!(TIM20_CH2, Tim20Ch2, TIM20, Tim20, _TIM20_CH2, TimAdvCh, _TIM20, 1);
channel!(TIM20_CH3, Tim20Ch3, TIM20, Tim20, _TIM20_CH3, TimAdvCh, _TIM20, 2);
channel!(TIM20_CH4, Tim20Ch4, TIM20, Tim20, _TIM20_CH4, TimAdvCh, _TIM20, 3);
