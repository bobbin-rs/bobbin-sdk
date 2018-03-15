#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::dma::*;

periph!( DMA1, Dma1, DMA1_PERIPH, DmaPeriph, 0x40020000, 0x0a);

channel!(DMA1_CH1, Dma1Ch1, DMA1, Dma1, DMA1_CH1_CH, DmaCh, DMA1_PERIPH, 0);
channel!(DMA1_CH2, Dma1Ch2, DMA1, Dma1, DMA1_CH2_CH, DmaCh, DMA1_PERIPH, 1);
channel!(DMA1_CH3, Dma1Ch3, DMA1, Dma1, DMA1_CH3_CH, DmaCh, DMA1_PERIPH, 2);
channel!(DMA1_CH4, Dma1Ch4, DMA1, Dma1, DMA1_CH4_CH, DmaCh, DMA1_PERIPH, 3);
channel!(DMA1_CH5, Dma1Ch5, DMA1, Dma1, DMA1_CH5_CH, DmaCh, DMA1_PERIPH, 4);
channel!(DMA1_CH6, Dma1Ch6, DMA1, Dma1, DMA1_CH6_CH, DmaCh, DMA1_PERIPH, 5);
channel!(DMA1_CH7, Dma1Ch7, DMA1, Dma1, DMA1_CH7_CH, DmaCh, DMA1_PERIPH, 6);
