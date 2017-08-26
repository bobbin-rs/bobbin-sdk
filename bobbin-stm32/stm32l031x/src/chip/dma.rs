#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::dma_f3::*;

periph!(DmaPeriph, DMA1, Dma1, 0x40020000);



channel!(DMA1_CH1, Dma1Ch1, DMA1, Dma1, 0);
channel!(DMA1_CH2, Dma1Ch2, DMA1, Dma1, 1);
channel!(DMA1_CH3, Dma1Ch3, DMA1, Dma1, 2);
channel!(DMA1_CH4, Dma1Ch4, DMA1, Dma1, 3);
channel!(DMA1_CH5, Dma1Ch5, DMA1, Dma1, 4);
channel!(DMA1_CH6, Dma1Ch6, DMA1, Dma1, 5);
channel!(DMA1_CH7, Dma1Ch7, DMA1, Dma1, 6);
