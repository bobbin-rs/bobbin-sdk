#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "DMAMUX", peripherals: [Peripheral { derived_from: None, group_name: None, name: "DMAMUX", address: 1073876992, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::dmamux::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::dmamux::*;

periph!( DMAMUX, Dmamux, _DMAMUX, DmamuxPeriph, 0x40021000);



