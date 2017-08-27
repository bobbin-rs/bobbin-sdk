#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "IWDG", peripherals: [Peripheral { derived_from: None, group_name: None, name: "IWDG", address: 1073754112, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Independent watchdog"), modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::iwdg::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::iwdg::*;

periph!( IWDG, Iwdg, _IWDG, IwdgPeriph, 0x40003000);



