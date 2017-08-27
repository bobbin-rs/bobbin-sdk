#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "WDOG", peripherals: [Peripheral { derived_from: None, group_name: None, name: "WDOG", address: 1074077696, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Generation 2008 Watchdog Timer"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "WDOG", types: ["WDOG"], value: 22, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::wdog::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::wdog::*;

periph!( WDOG, Wdog, _WDOG, WdogPeriph, 0x40052000);



