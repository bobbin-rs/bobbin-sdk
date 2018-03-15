#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::dma::*;

periph!( DMA, Dma, DMA_PERIPH, DmaPeriph, 0x40008000, 0x06);

channel!(DMA0, Dma0, DMA, Dma, DMA0_CH, DmaCh, DMA_PERIPH, 0);
channel!(DMA1, Dma1, DMA, Dma, DMA1_CH, DmaCh, DMA_PERIPH, 1);
channel!(DMA2, Dma2, DMA, Dma, DMA2_CH, DmaCh, DMA_PERIPH, 2);
channel!(DMA3, Dma3, DMA, Dma, DMA3_CH, DmaCh, DMA_PERIPH, 3);
