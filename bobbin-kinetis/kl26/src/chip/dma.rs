#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "DMA", peripherals: [Peripheral { derived_from: None, group_name: None, name: "DMA", address: 1073774592, size: None, access: None, reset_value: None, reset_mask: None, description: Some("DMA Controller"), modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "DMA0", index: Some(0), description: None, signals: [], interrupts: [Interrupt { name: "DMA0", types: [], value: 0, description: None }] }, Channel { name: "DMA1", index: Some(1), description: None, signals: [], interrupts: [Interrupt { name: "DMA1", types: [], value: 1, description: None }] }, Channel { name: "DMA2", index: Some(2), description: None, signals: [], interrupts: [Interrupt { name: "DMA2", types: [], value: 2, description: None }] }, Channel { name: "DMA3", index: Some(3), description: None, signals: [], interrupts: [Interrupt { name: "DMA3", types: [], value: 3, description: None }] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::dma::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::dma::*;

periph!( DMA, Dma, _DMA, DmaPeriph, 0x40008000);



channel!(DMA0, Dma0, DMA, Dma, _DMA0, DmaCh, _DMA, 0);
channel!(DMA1, Dma1, DMA, Dma, _DMA1, DmaCh, _DMA, 1);
channel!(DMA2, Dma2, DMA, Dma, _DMA2, DmaCh, _DMA, 2);
channel!(DMA3, Dma3, DMA, Dma, _DMA3, DmaCh, _DMA, 3);
