#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "I2C", peripherals: [Peripheral { derived_from: None, group_name: None, name: "I2C0", address: 1074159616, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Inter-Integrated Circuit"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "I2C0", types: [], value: 24, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "I2C1", address: 1074163712, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Inter-Integrated Circuit"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "I2C1", types: [], value: 25, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::i2c::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::i2c::*;

periph!( I2C0, I2c0, _I2C0, I2cPeriph, 0x40066000);
periph!( I2C1, I2c1, _I2C1, I2cPeriph, 0x40067000);




