#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "DMA", peripherals: [Peripheral { derived_from: None, group_name: None, name: "DMA1", address: 1073872896, size: None, access: None, reset_value: None, reset_mask: None, description: Some("DMA controller 1"), modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "DMA1_CH1", index: Some(0), description: None, signals: [], interrupts: [Interrupt { name: "DMA1_CH1", types: ["DMA"], value: 11, description: Some("DMA1 channel 1 interrupt") }] }, Channel { name: "DMA1_CH2", index: Some(1), description: None, signals: [], interrupts: [Interrupt { name: "DMA1_CH2", types: ["DMA"], value: 12, description: Some("DMA1 channel 2 interrupt") }] }, Channel { name: "DMA1_CH3", index: Some(2), description: None, signals: [], interrupts: [Interrupt { name: "DMA1_CH3", types: ["DMA"], value: 13, description: Some("DMA1 channel 3 interrupt") }] }, Channel { name: "DMA1_CH4", index: Some(3), description: None, signals: [], interrupts: [Interrupt { name: "DMA1_CH4", types: ["DMA"], value: 14, description: Some("DMA1 channel 4 interrupt") }] }, Channel { name: "DMA1_CH5", index: Some(4), description: None, signals: [], interrupts: [Interrupt { name: "DMA1_CH5", types: ["DMA"], value: 15, description: Some("DMA1 channel 5 interrupt") }] }, Channel { name: "DMA1_CH6", index: Some(5), description: None, signals: [], interrupts: [Interrupt { name: "DMA1_CH6", types: ["DMA"], value: 16, description: Some("DMA1 channel 6 interrupt") }] }, Channel { name: "DMA1_CH7", index: Some(6), description: None, signals: [], interrupts: [Interrupt { name: "DMA1_CH7", types: ["DMA"], value: 17, description: Some("DMA1 channel 7 interrupt") }] }], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "DMA2", address: 1073873920, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [Channel { name: "DMA2_CH1", index: Some(0), description: None, signals: [], interrupts: [Interrupt { name: "DMA2_CH1", types: ["DMA"], value: 56, description: Some("DMA2 channel1 global interrupt") }] }, Channel { name: "DMA2_CH2", index: Some(1), description: None, signals: [], interrupts: [Interrupt { name: "DMA2_CH2", types: ["DMA"], value: 57, description: Some("DMA2 channel2 global interrupt") }] }, Channel { name: "DMA2_CH3", index: Some(2), description: None, signals: [], interrupts: [Interrupt { name: "DMA2_CH3", types: ["DMA"], value: 58, description: Some("DMA2 channel3 global interrupt") }] }, Channel { name: "DMA2_CH4", index: Some(3), description: None, signals: [], interrupts: [Interrupt { name: "DMA2_CH4", types: ["DMA"], value: 59, description: Some("DMA2 channel4 global interrupt") }] }, Channel { name: "DMA2_CH5", index: Some(4), description: None, signals: [], interrupts: [Interrupt { name: "DMA2_CH5", types: ["DMA"], value: 60, description: Some("DMA2 channel5 global interrupt") }] }], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::dma_f3::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::dma_f3::*;

periph!( DMA1, Dma1, _DMA1, DmaPeriph, 0x40020000);
periph!( DMA2, Dma2, _DMA2, DmaPeriph, 0x40020400);




channel!(DMA1_CH1, Dma1Ch1, DMA1, Dma1, _DMA1_CH1, DmaCh, _DMA1, 0);
channel!(DMA1_CH2, Dma1Ch2, DMA1, Dma1, _DMA1_CH2, DmaCh, _DMA1, 1);
channel!(DMA1_CH3, Dma1Ch3, DMA1, Dma1, _DMA1_CH3, DmaCh, _DMA1, 2);
channel!(DMA1_CH4, Dma1Ch4, DMA1, Dma1, _DMA1_CH4, DmaCh, _DMA1, 3);
channel!(DMA1_CH5, Dma1Ch5, DMA1, Dma1, _DMA1_CH5, DmaCh, _DMA1, 4);
channel!(DMA1_CH6, Dma1Ch6, DMA1, Dma1, _DMA1_CH6, DmaCh, _DMA1, 5);
channel!(DMA1_CH7, Dma1Ch7, DMA1, Dma1, _DMA1_CH7, DmaCh, _DMA1, 6);
channel!(DMA2_CH1, Dma2Ch1, DMA2, Dma2, _DMA2_CH1, DmaCh, _DMA2, 0);
channel!(DMA2_CH2, Dma2Ch2, DMA2, Dma2, _DMA2_CH2, DmaCh, _DMA2, 1);
channel!(DMA2_CH3, Dma2Ch3, DMA2, Dma2, _DMA2_CH3, DmaCh, _DMA2, 2);
channel!(DMA2_CH4, Dma2Ch4, DMA2, Dma2, _DMA2_CH4, DmaCh, _DMA2, 3);
channel!(DMA2_CH5, Dma2Ch5, DMA2, Dma2, _DMA2_CH5, DmaCh, _DMA2, 4);
