#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "CRC", peripherals: [Peripheral { derived_from: None, group_name: None, name: "CRC", address: 1073885184, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Cyclic redundancy check calculation unit"), modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::crc::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::crc::*;

periph!( CRC, Crc, _CRC, CrcPeriph, 0x40023000);



