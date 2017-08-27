#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "RTC", peripherals: [Peripheral { derived_from: None, group_name: None, name: "RTC", address: 1073752064, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Real-time clock"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "RTC", types: ["RTC"], value: 2, description: Some("RTC global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::rtc::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::rtc::*;

periph!( RTC, Rtc, _RTC, RtcPeriph, 0x40002800);



