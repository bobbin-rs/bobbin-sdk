#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "PIT", peripherals: [Peripheral { derived_from: None, group_name: None, name: "PIT", address: 1073967104, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Periodic Interrupt Timer"), modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "PIT_CH0", index: Some(0), description: None, signals: [], interrupts: [Interrupt { name: "PIT0", types: ["PIT"], value: 48, description: None }] }, Channel { name: "PIT_CH1", index: Some(1), description: None, signals: [], interrupts: [Interrupt { name: "PIT1", types: ["PIT"], value: 49, description: None }] }, Channel { name: "PIT_CH2", index: Some(2), description: None, signals: [], interrupts: [Interrupt { name: "PIT2", types: ["PIT"], value: 50, description: None }] }, Channel { name: "PIT_CH3", index: Some(3), description: None, signals: [], interrupts: [Interrupt { name: "PIT3", types: ["PIT"], value: 51, description: None }] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::pit::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::pit::*;

periph!( PIT, Pit, _PIT, PitPeriph, 0x40037000);



channel!(PIT_CH0, PitCh0, PIT, Pit, _PIT_CH0, PitCh, _PIT, 0);
channel!(PIT_CH1, PitCh1, PIT, Pit, _PIT_CH1, PitCh, _PIT, 1);
channel!(PIT_CH2, PitCh2, PIT, Pit, _PIT_CH2, PitCh, _PIT, 2);
channel!(PIT_CH3, PitCh3, PIT, Pit, _PIT_CH3, PitCh, _PIT, 3);
