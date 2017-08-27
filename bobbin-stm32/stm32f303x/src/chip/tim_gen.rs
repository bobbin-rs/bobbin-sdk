#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "TIM_GEN", peripherals: [Peripheral { derived_from: None, group_name: None, name: "TIM2", address: 1073741824, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General purpose timer"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM2", types: [], value: 28, description: Some("TIM2 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TIM2_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM2_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM2_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM2_CH2", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM2_CH3", index: Some(2), description: None, signals: [Signal { name: "TIM2_CH3", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM2_CH4", index: Some(3), description: None, signals: [Signal { name: "TIM2_CH4", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM3", address: 1073742848, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM3", types: [], value: 29, description: Some("TIM3 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TIM3_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM3_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM3_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM3_CH2", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM3_CH3", index: Some(2), description: None, signals: [Signal { name: "TIM3_CH3", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM3_CH4", index: Some(3), description: None, signals: [Signal { name: "TIM3_CH4", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM4", address: 1073743872, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM4", types: [], value: 30, description: Some("TIM4 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TIM4_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM4_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM4_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM4_CH2", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM4_CH3", index: Some(2), description: None, signals: [Signal { name: "TIM4_CH3", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM4_CH4", index: Some(3), description: None, signals: [Signal { name: "TIM4_CH4", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM15", address: 1073823744, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General purpose timers"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM15", types: [], value: 24, description: Some("TIM1 Break/TIM15 global interruts") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TIM15_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM15_CH1", types: ["TIM"], description: None }], interrupts: [] }, Channel { name: "TIM15_CH2", index: Some(1), description: None, signals: [Signal { name: "TIM15_CH2", types: ["TIM"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM16", address: 1073824768, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General-purpose-timers"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM16", types: [], value: 25, description: Some("TIM16 global interrupts") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TIM16_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM16_CH1", types: ["TIM"], description: None }, Signal { name: "TIM16_CH1N", types: ["TIMN"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM17", address: 1073825792, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General purpose timer"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM17", types: [], value: 26, description: Some("TIM17 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "TIM17_CH1", index: Some(0), description: None, signals: [Signal { name: "TIM17_CH1", types: ["TIM"], description: None }, Signal { name: "TIM17_CH1N", types: ["TIMN"], description: None }], interrupts: [] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::tim_gen::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::tim_gen::*;

periph!( TIM2, Tim2, _TIM2, TimGenPeriph, 0x40000000);
periph!( TIM3, Tim3, _TIM3, TimGenPeriph, 0x40000400);
periph!( TIM4, Tim4, _TIM4, TimGenPeriph, 0x40000800);
periph!( TIM15, Tim15, _TIM15, TimGenPeriph, 0x40014000);
periph!( TIM16, Tim16, _TIM16, TimGenPeriph, 0x40014400);
periph!( TIM17, Tim17, _TIM17, TimGenPeriph, 0x40014800);

impl super::sig::Signal<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::SignalTim<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::Signal<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::SignalTim<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::Signal<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::SignalTim<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::Signal<super::sig::Tim2Ch4> for Tim2Ch4 {}
impl super::sig::SignalTim<super::sig::Tim2Ch4> for Tim2Ch4 {}

impl super::sig::Signal<super::sig::Tim3Ch1> for Tim3Ch1 {}
impl super::sig::SignalTim<super::sig::Tim3Ch1> for Tim3Ch1 {}
impl super::sig::Signal<super::sig::Tim3Ch2> for Tim3Ch2 {}
impl super::sig::SignalTim<super::sig::Tim3Ch2> for Tim3Ch2 {}
impl super::sig::Signal<super::sig::Tim3Ch3> for Tim3Ch3 {}
impl super::sig::SignalTim<super::sig::Tim3Ch3> for Tim3Ch3 {}
impl super::sig::Signal<super::sig::Tim3Ch4> for Tim3Ch4 {}
impl super::sig::SignalTim<super::sig::Tim3Ch4> for Tim3Ch4 {}

impl super::sig::Signal<super::sig::Tim4Ch1> for Tim4Ch1 {}
impl super::sig::SignalTim<super::sig::Tim4Ch1> for Tim4Ch1 {}
impl super::sig::Signal<super::sig::Tim4Ch2> for Tim4Ch2 {}
impl super::sig::SignalTim<super::sig::Tim4Ch2> for Tim4Ch2 {}
impl super::sig::Signal<super::sig::Tim4Ch3> for Tim4Ch3 {}
impl super::sig::SignalTim<super::sig::Tim4Ch3> for Tim4Ch3 {}
impl super::sig::Signal<super::sig::Tim4Ch4> for Tim4Ch4 {}
impl super::sig::SignalTim<super::sig::Tim4Ch4> for Tim4Ch4 {}

impl super::sig::Signal<super::sig::Tim15Ch1> for Tim15Ch1 {}
impl super::sig::SignalTim<super::sig::Tim15Ch1> for Tim15Ch1 {}
impl super::sig::Signal<super::sig::Tim15Ch2> for Tim15Ch2 {}
impl super::sig::SignalTim<super::sig::Tim15Ch2> for Tim15Ch2 {}

impl super::sig::Signal<super::sig::Tim16Ch1> for Tim16Ch1 {}
impl super::sig::SignalTim<super::sig::Tim16Ch1> for Tim16Ch1 {}
impl super::sig::Signal<super::sig::Tim16Ch1n> for Tim16Ch1 {}
impl super::sig::SignalTimn<super::sig::Tim16Ch1n> for Tim16Ch1 {}

impl super::sig::Signal<super::sig::Tim17Ch1> for Tim17Ch1 {}
impl super::sig::SignalTim<super::sig::Tim17Ch1> for Tim17Ch1 {}
impl super::sig::Signal<super::sig::Tim17Ch1n> for Tim17Ch1 {}
impl super::sig::SignalTimn<super::sig::Tim17Ch1n> for Tim17Ch1 {}


channel!(TIM2_CH1, Tim2Ch1, TIM2, Tim2, _TIM2_CH1, TimGenCh, _TIM2, 0);
channel!(TIM2_CH2, Tim2Ch2, TIM2, Tim2, _TIM2_CH2, TimGenCh, _TIM2, 1);
channel!(TIM2_CH3, Tim2Ch3, TIM2, Tim2, _TIM2_CH3, TimGenCh, _TIM2, 2);
channel!(TIM2_CH4, Tim2Ch4, TIM2, Tim2, _TIM2_CH4, TimGenCh, _TIM2, 3);
channel!(TIM3_CH1, Tim3Ch1, TIM3, Tim3, _TIM3_CH1, TimGenCh, _TIM3, 0);
channel!(TIM3_CH2, Tim3Ch2, TIM3, Tim3, _TIM3_CH2, TimGenCh, _TIM3, 1);
channel!(TIM3_CH3, Tim3Ch3, TIM3, Tim3, _TIM3_CH3, TimGenCh, _TIM3, 2);
channel!(TIM3_CH4, Tim3Ch4, TIM3, Tim3, _TIM3_CH4, TimGenCh, _TIM3, 3);
channel!(TIM4_CH1, Tim4Ch1, TIM4, Tim4, _TIM4_CH1, TimGenCh, _TIM4, 0);
channel!(TIM4_CH2, Tim4Ch2, TIM4, Tim4, _TIM4_CH2, TimGenCh, _TIM4, 1);
channel!(TIM4_CH3, Tim4Ch3, TIM4, Tim4, _TIM4_CH3, TimGenCh, _TIM4, 2);
channel!(TIM4_CH4, Tim4Ch4, TIM4, Tim4, _TIM4_CH4, TimGenCh, _TIM4, 3);
channel!(TIM15_CH1, Tim15Ch1, TIM15, Tim15, _TIM15_CH1, TimGenCh, _TIM15, 0);
channel!(TIM15_CH2, Tim15Ch2, TIM15, Tim15, _TIM15_CH2, TimGenCh, _TIM15, 1);
channel!(TIM16_CH1, Tim16Ch1, TIM16, Tim16, _TIM16_CH1, TimGenCh, _TIM16, 0);
channel!(TIM17_CH1, Tim17Ch1, TIM17, Tim17, _TIM17_CH1, TimGenCh, _TIM17, 0);
