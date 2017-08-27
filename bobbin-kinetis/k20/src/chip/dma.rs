#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "DMA", peripherals: [Peripheral { derived_from: None, group_name: None, name: "DMA", address: 1073774592, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Enhanced direct memory access controller"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "DMA_Error", types: [], value: 16, description: None }], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "DMA0", index: Some(0), description: None, signals: [], interrupts: [Interrupt { name: "DMA0", types: [], value: 0, description: None }] }, Channel { name: "DMA1", index: Some(1), description: None, signals: [], interrupts: [Interrupt { name: "DMA1", types: [], value: 1, description: None }] }, Channel { name: "DMA2", index: Some(2), description: None, signals: [], interrupts: [Interrupt { name: "DMA2", types: [], value: 2, description: None }] }, Channel { name: "DMA3", index: Some(3), description: None, signals: [], interrupts: [Interrupt { name: "DMA3", types: [], value: 3, description: None }] }, Channel { name: "DMA4", index: Some(4), description: None, signals: [], interrupts: [Interrupt { name: "DMA4", types: [], value: 4, description: None }] }, Channel { name: "DMA5", index: Some(5), description: None, signals: [], interrupts: [Interrupt { name: "DMA5", types: [], value: 5, description: None }] }, Channel { name: "DMA6", index: Some(6), description: None, signals: [], interrupts: [Interrupt { name: "DMA6", types: [], value: 6, description: None }] }, Channel { name: "DMA7", index: Some(7), description: None, signals: [], interrupts: [Interrupt { name: "DMA7", types: [], value: 7, description: None }] }, Channel { name: "DMA8", index: Some(8), description: None, signals: [], interrupts: [Interrupt { name: "DMA8", types: [], value: 8, description: None }] }, Channel { name: "DMA9", index: Some(9), description: None, signals: [], interrupts: [Interrupt { name: "DMA9", types: [], value: 9, description: None }] }, Channel { name: "DMA10", index: Some(10), description: None, signals: [], interrupts: [Interrupt { name: "DMA10", types: [], value: 10, description: None }] }, Channel { name: "DMA11", index: Some(11), description: None, signals: [], interrupts: [Interrupt { name: "DMA11", types: [], value: 11, description: None }] }, Channel { name: "DMA12", index: Some(12), description: None, signals: [], interrupts: [Interrupt { name: "DMA12", types: [], value: 12, description: None }] }, Channel { name: "DMA13", index: Some(13), description: None, signals: [], interrupts: [Interrupt { name: "DMA13", types: [], value: 13, description: None }] }, Channel { name: "DMA14", index: Some(14), description: None, signals: [], interrupts: [Interrupt { name: "DMA14", types: [], value: 14, description: None }] }, Channel { name: "DMA15", index: Some(15), description: None, signals: [], interrupts: [Interrupt { name: "DMA15", types: [], value: 15, description: None }] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::edma::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::edma::*;

periph!( DMA, Dma, _DMA, DmaPeriph, 0x40008000);



channel!(DMA0, Dma0, DMA, Dma, _DMA0, DmaCh, _DMA, 0);
channel!(DMA1, Dma1, DMA, Dma, _DMA1, DmaCh, _DMA, 1);
channel!(DMA2, Dma2, DMA, Dma, _DMA2, DmaCh, _DMA, 2);
channel!(DMA3, Dma3, DMA, Dma, _DMA3, DmaCh, _DMA, 3);
channel!(DMA4, Dma4, DMA, Dma, _DMA4, DmaCh, _DMA, 4);
channel!(DMA5, Dma5, DMA, Dma, _DMA5, DmaCh, _DMA, 5);
channel!(DMA6, Dma6, DMA, Dma, _DMA6, DmaCh, _DMA, 6);
channel!(DMA7, Dma7, DMA, Dma, _DMA7, DmaCh, _DMA, 7);
channel!(DMA8, Dma8, DMA, Dma, _DMA8, DmaCh, _DMA, 8);
channel!(DMA9, Dma9, DMA, Dma, _DMA9, DmaCh, _DMA, 9);
channel!(DMA10, Dma10, DMA, Dma, _DMA10, DmaCh, _DMA, 10);
channel!(DMA11, Dma11, DMA, Dma, _DMA11, DmaCh, _DMA, 11);
channel!(DMA12, Dma12, DMA, Dma, _DMA12, DmaCh, _DMA, 12);
channel!(DMA13, Dma13, DMA, Dma, _DMA13, DmaCh, _DMA, 13);
channel!(DMA14, Dma14, DMA, Dma, _DMA14, DmaCh, _DMA, 14);
channel!(DMA15, Dma15, DMA, Dma, _DMA15, DmaCh, _DMA, 15);
