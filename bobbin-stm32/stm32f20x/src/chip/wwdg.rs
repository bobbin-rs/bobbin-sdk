#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "WWDG", peripherals: [Peripheral { derived_from: None, group_name: None, name: "WWDG", address: 1073753088, size: None, access: None, reset_value: None, reset_mask: None, description: Some("System window watchdog"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "WWDG", types: ["WWDG"], value: 0, description: Some("Window Watchdog interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::wwdg::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::wwdg::*;

periph!( WWDG, Wwdg, _WWDG, WwdgPeriph, 0x40002c00);



