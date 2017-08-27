#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "TIM_GEN", peripherals: [Peripheral { derived_from: None, group_name: None, name: "TIM2", address: 1073741824, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General purpose timer"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM2", types: ["TIM"], value: 15, description: Some("TIM2 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "TIM2_ETR", types: ["ETR"], description: None }], pins: [], channels: [Channel { name: "TIM2_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM2_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM2_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM2_CH2", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM2_CH3", index: Some(2), description: None, signals: [Signal { name: "TIM2_CH3", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM2_CH4", index: Some(3), description: None, signals: [Signal { name: "TIM2_CH4", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM21", address: 1073809408, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General-purpose-timers"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM21", types: ["TIM"], value: 20, description: Some("TIMER21 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "TIM21_ETR", types: ["ETR"], description: None }], pins: [], channels: [Channel { name: "TIM21_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM21_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM21_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM21_CH2", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM22", address: 1073812480, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General-purpose-timers"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM22", types: ["TIM"], value: 22, description: Some("TIMER22 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "TIM22_ETR", types: ["ETR"], description: None }], pins: [], channels: [Channel { name: "TIM22_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM22_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM22_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM22_CH2", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::tim_gen::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::tim_gen::*;

periph!( TIM2, Tim2, _TIM2, TimGenPeriph, 0x40000000);
periph!( TIM21, Tim21, _TIM21, TimGenPeriph, 0x40010800);
periph!( TIM22, Tim22, _TIM22, TimGenPeriph, 0x40011400);

impl super::sig::Signal<super::sig::Tim2Etr> for Tim2 {}
impl super::sig::SignalEtr<super::sig::Tim2Etr> for Tim2 {}
impl super::sig::Signal<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::SignalTim<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::Signal<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::SignalTim<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::Signal<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::SignalTim<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::Signal<super::sig::Tim2Ch4> for Tim2Ch4 {}
impl super::sig::SignalTim<super::sig::Tim2Ch4> for Tim2Ch4 {}

impl super::sig::Signal<super::sig::Tim21Etr> for Tim21 {}
impl super::sig::SignalEtr<super::sig::Tim21Etr> for Tim21 {}
impl super::sig::Signal<super::sig::Tim21Ch1> for Tim21Ch1 {}
impl super::sig::SignalTim<super::sig::Tim21Ch1> for Tim21Ch1 {}
impl super::sig::Signal<super::sig::Tim21Ch2> for Tim21Ch2 {}
impl super::sig::SignalTim<super::sig::Tim21Ch2> for Tim21Ch2 {}

impl super::sig::Signal<super::sig::Tim22Etr> for Tim22 {}
impl super::sig::SignalEtr<super::sig::Tim22Etr> for Tim22 {}
impl super::sig::Signal<super::sig::Tim22Ch1> for Tim22Ch1 {}
impl super::sig::SignalTim<super::sig::Tim22Ch1> for Tim22Ch1 {}
impl super::sig::Signal<super::sig::Tim22Ch2> for Tim22Ch2 {}
impl super::sig::SignalTim<super::sig::Tim22Ch2> for Tim22Ch2 {}


channel!(TIM2_CH1, Tim2Ch1, TIM2, Tim2, _TIM2_CH1, TimGenCh, _TIM2, 0);
channel!(TIM2_CH2, Tim2Ch2, TIM2, Tim2, _TIM2_CH2, TimGenCh, _TIM2, 1);
channel!(TIM2_CH3, Tim2Ch3, TIM2, Tim2, _TIM2_CH3, TimGenCh, _TIM2, 2);
channel!(TIM2_CH4, Tim2Ch4, TIM2, Tim2, _TIM2_CH4, TimGenCh, _TIM2, 3);
channel!(TIM21_CH1, Tim21Ch1, TIM21, Tim21, _TIM21_CH1, TimGenCh, _TIM21, 0);
channel!(TIM21_CH2, Tim21Ch2, TIM21, Tim21, _TIM21_CH2, TimGenCh, _TIM21, 1);
channel!(TIM22_CH1, Tim22Ch1, TIM22, Tim22, _TIM22_CH1, TimGenCh, _TIM22, 0);
channel!(TIM22_CH2, Tim22Ch2, TIM22, Tim22, _TIM22_CH2, TimGenCh, _TIM22, 1);
