#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "TIM_ADV", peripherals: [Peripheral { derived_from: None, group_name: None, name: "TIM1", address: 1073807360, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Advanced-timers"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM1_BRK", types: [], value: 24, description: Some("TIM1 Break interrupt") }, Interrupt { name: "TIM1_UP", types: [], value: 25, description: Some("TIM1 Update interrupt") }, Interrupt { name: "TIM1_TRG_COM", types: [], value: 26, description: Some("TIM1 Trigger and Commutation interrupt") }, Interrupt { name: "TIM1_CC", types: [], value: 27, description: Some("TIM1 Capture Compare interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM8", address: 1073808384, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM8_BRK", types: [], value: 43, description: Some("TIM8 Break interrupt and TIM12 global interrupt") }, Interrupt { name: "TIM8_UP", types: [], value: 44, description: Some("TIM8 Update interrupt and TIM13 global interrupt") }, Interrupt { name: "TIM8_TRG_COM", types: [], value: 45, description: Some("TIM8 Trigger and Commutation interrupts and TIM14 global interrupt") }, Interrupt { name: "TIM8_CC", types: [], value: 46, description: Some("TIM8 Capture Compare interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::tim_adv::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::tim_adv::*;

periph!( TIM1, Tim1, _TIM1, TimAdvPeriph, 0x40010000);
periph!( TIM8, Tim8, _TIM8, TimAdvPeriph, 0x40010400);




