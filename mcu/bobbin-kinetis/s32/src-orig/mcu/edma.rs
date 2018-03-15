#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::edma::*;

periph!( DMA, Dma, DMA_PERIPH, EdmaPeriph, 0x40008000, 0x09);

channel!(DMA0, Dma0, DMA, Dma, DMA0_CH, EdmaCh, DMA_PERIPH, 0);
channel!(DMA1, Dma1, DMA, Dma, DMA1_CH, EdmaCh, DMA_PERIPH, 1);
channel!(DMA2, Dma2, DMA, Dma, DMA2_CH, EdmaCh, DMA_PERIPH, 2);
channel!(DMA3, Dma3, DMA, Dma, DMA3_CH, EdmaCh, DMA_PERIPH, 3);
channel!(DMA4, Dma4, DMA, Dma, DMA4_CH, EdmaCh, DMA_PERIPH, 4);
channel!(DMA5, Dma5, DMA, Dma, DMA5_CH, EdmaCh, DMA_PERIPH, 5);
channel!(DMA6, Dma6, DMA, Dma, DMA6_CH, EdmaCh, DMA_PERIPH, 6);
channel!(DMA7, Dma7, DMA, Dma, DMA7_CH, EdmaCh, DMA_PERIPH, 7);
channel!(DMA8, Dma8, DMA, Dma, DMA8_CH, EdmaCh, DMA_PERIPH, 8);
channel!(DMA9, Dma9, DMA, Dma, DMA9_CH, EdmaCh, DMA_PERIPH, 9);
channel!(DMA10, Dma10, DMA, Dma, DMA10_CH, EdmaCh, DMA_PERIPH, 10);
channel!(DMA11, Dma11, DMA, Dma, DMA11_CH, EdmaCh, DMA_PERIPH, 11);
channel!(DMA12, Dma12, DMA, Dma, DMA12_CH, EdmaCh, DMA_PERIPH, 12);
channel!(DMA13, Dma13, DMA, Dma, DMA13_CH, EdmaCh, DMA_PERIPH, 13);
channel!(DMA14, Dma14, DMA, Dma, DMA14_CH, EdmaCh, DMA_PERIPH, 14);
channel!(DMA15, Dma15, DMA, Dma, DMA15_CH, EdmaCh, DMA_PERIPH, 15);
