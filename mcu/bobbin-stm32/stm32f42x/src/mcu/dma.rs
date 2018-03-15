#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::dma::*;

periph!( DMA1, Dma1, DMA1_PERIPH, DmaPeriph, 0x40026000, 0x3d);
periph!( DMA2, Dma2, DMA2_PERIPH, DmaPeriph, 0x40026400, 0x3e);

channel!(DMA1_STREAM0, Dma1Stream0, DMA1, Dma1, DMA1_STREAM0_CH, DmaCh, DMA1_PERIPH, 0);
channel!(DMA1_STREAM1, Dma1Stream1, DMA1, Dma1, DMA1_STREAM1_CH, DmaCh, DMA1_PERIPH, 1);
channel!(DMA1_STREAM2, Dma1Stream2, DMA1, Dma1, DMA1_STREAM2_CH, DmaCh, DMA1_PERIPH, 2);
channel!(DMA1_STREAM3, Dma1Stream3, DMA1, Dma1, DMA1_STREAM3_CH, DmaCh, DMA1_PERIPH, 3);
channel!(DMA1_STREAM4, Dma1Stream4, DMA1, Dma1, DMA1_STREAM4_CH, DmaCh, DMA1_PERIPH, 4);
channel!(DMA1_STREAM5, Dma1Stream5, DMA1, Dma1, DMA1_STREAM5_CH, DmaCh, DMA1_PERIPH, 5);
channel!(DMA1_STREAM6, Dma1Stream6, DMA1, Dma1, DMA1_STREAM6_CH, DmaCh, DMA1_PERIPH, 6);
channel!(DMA1_STREAM7, Dma1Stream7, DMA1, Dma1, DMA1_STREAM7_CH, DmaCh, DMA1_PERIPH, 7);
channel!(DMA2_STREAM0, Dma2Stream0, DMA2, Dma2, DMA2_STREAM0_CH, DmaCh, DMA2_PERIPH, 0);
channel!(DMA2_STREAM1, Dma2Stream1, DMA2, Dma2, DMA2_STREAM1_CH, DmaCh, DMA2_PERIPH, 1);
channel!(DMA2_STREAM2, Dma2Stream2, DMA2, Dma2, DMA2_STREAM2_CH, DmaCh, DMA2_PERIPH, 2);
channel!(DMA2_STREAM3, Dma2Stream3, DMA2, Dma2, DMA2_STREAM3_CH, DmaCh, DMA2_PERIPH, 3);
channel!(DMA2_STREAM4, Dma2Stream4, DMA2, Dma2, DMA2_STREAM4_CH, DmaCh, DMA2_PERIPH, 4);
channel!(DMA2_STREAM5, Dma2Stream5, DMA2, Dma2, DMA2_STREAM5_CH, DmaCh, DMA2_PERIPH, 5);
channel!(DMA2_STREAM6, Dma2Stream6, DMA2, Dma2, DMA2_STREAM6_CH, DmaCh, DMA2_PERIPH, 6);
channel!(DMA2_STREAM7, Dma2Stream7, DMA2, Dma2, DMA2_STREAM7_CH, DmaCh, DMA2_PERIPH, 7);
