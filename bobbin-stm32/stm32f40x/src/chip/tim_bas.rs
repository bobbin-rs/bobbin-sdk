#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "TIM_BAS", peripherals: [Peripheral { derived_from: None, group_name: None, name: "TIM6", address: 1073745920, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Basic timers"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM6", types: ["TIM"], value: 54, description: Some("TIM6 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "TIM7", address: 1073746944, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "TIM7", types: ["TIM"], value: 55, description: Some("TIM7 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::tim_bas::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::tim_bas::*;

periph!( TIM6, Tim6, _TIM6, TimBasPeriph, 0x40001000);
periph!( TIM7, Tim7, _TIM7, TimBasPeriph, 0x40001400);




