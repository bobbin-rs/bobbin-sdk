#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "SPI", peripherals: [Peripheral { derived_from: None, group_name: None, name: "SPI0", address: 1073922048, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Serial Peripheral Interface"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "SPI0", types: ["SPI"], value: 26, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "SPI1", address: 1073926144, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Serial Peripheral Interface"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "SPI1", types: ["SPI"], value: 27, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "SPI2", address: 1074446336, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Serial Peripheral Interface"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "SPI2", types: ["SPI"], value: 65, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::spi::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::spi::*;

periph!( SPI0, Spi0, _SPI0, SpiPeriph, 0x4002c000);
periph!( SPI1, Spi1, _SPI1, SpiPeriph, 0x4002d000);
periph!( SPI2, Spi2, _SPI2, SpiPeriph, 0x400ac000);





