#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "LPTMR", peripherals: [Peripheral { derived_from: None, group_name: None, name: "LPTMR0", address: 1074003968, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Low Power Timer"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "LPTMR0", types: ["LPTMR"], value: 58, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::lptmr::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::lptmr::*;

periph!( LPTMR0, Lptmr0, _LPTMR0, LptmrPeriph, 0x40040000);



