#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "PIT", peripherals: [Peripheral { derived_from: None, group_name: None, name: "PIT", address: 1073967104, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Periodic Interrupt Timer"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "PIT0", types: [], value: 48, description: None }, Interrupt { name: "PIT1", types: [], value: 49, description: None }, Interrupt { name: "PIT2", types: [], value: 50, description: None }, Interrupt { name: "PIT3", types: [], value: 51, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::pit::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::pit::*;

periph!( PIT, Pit, _PIT, PitPeriph, 0x40037000);



