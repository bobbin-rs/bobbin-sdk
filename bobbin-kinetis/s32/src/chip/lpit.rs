#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "LPIT", peripherals: [Peripheral { derived_from: None, group_name: None, name: "LPIT0", address: 1073967104, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Low Power Periodic Interrupt Timer (LPIT)"), modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "LPIT0_CH0", index: Some(0), description: None, signals: [], interrupts: [Interrupt { name: "LPIT0_CH0", types: ["LPIT"], value: 48, description: None }] }, Channel { name: "LPIT0_CH1", index: Some(1), description: None, signals: [], interrupts: [Interrupt { name: "LPIT0_CH1", types: ["LPIT"], value: 49, description: None }] }, Channel { name: "LPIT0_CH2", index: Some(2), description: None, signals: [], interrupts: [Interrupt { name: "LPIT0_CH2", types: ["LPIT"], value: 50, description: None }] }, Channel { name: "LPIT0_CH3", index: Some(3), description: None, signals: [], interrupts: [Interrupt { name: "LPIT0_CH3", types: ["LPIT"], value: 51, description: None }] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::lpit::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::lpit::*;

periph!( LPIT0, Lpit0, _LPIT0, LpitPeriph, 0x40037000);



channel!(LPIT0_CH0, Lpit0Ch0, LPIT0, Lpit0, _LPIT0_CH0, LpitCh, _LPIT0, 0);
channel!(LPIT0_CH1, Lpit0Ch1, LPIT0, Lpit0, _LPIT0_CH1, LpitCh, _LPIT0, 1);
channel!(LPIT0_CH2, Lpit0Ch2, LPIT0, Lpit0, _LPIT0_CH2, LpitCh, _LPIT0, 2);
channel!(LPIT0_CH3, Lpit0Ch3, LPIT0, Lpit0, _LPIT0_CH3, LpitCh, _LPIT0, 3);
