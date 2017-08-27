#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "LPTIM", peripherals: [Peripheral { derived_from: None, group_name: None, name: "LPTIM", address: 1073773568, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Low power timer"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "LPTIM1", types: ["LPTIM"], value: 13, description: Some("LPTIMER1 interrupt through EXTI29") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::lptim::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::lptim::*;

periph!( LPTIM, Lptim, _LPTIM, LptimPeriph, 0x40007c00);



