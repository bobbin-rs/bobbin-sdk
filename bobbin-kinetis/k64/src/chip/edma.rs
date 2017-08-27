#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::edma::*;

periph!(_DMA, EdmaPeriph, DMA, Dma, 0x40008000);



channel!(DMA0, Dma0, DMA, Dma, 0);
channel!(DMA1, Dma1, DMA, Dma, 1);
channel!(DMA2, Dma2, DMA, Dma, 2);
channel!(DMA3, Dma3, DMA, Dma, 3);
channel!(DMA4, Dma4, DMA, Dma, 4);
channel!(DMA5, Dma5, DMA, Dma, 5);
channel!(DMA6, Dma6, DMA, Dma, 6);
channel!(DMA7, Dma7, DMA, Dma, 7);
channel!(DMA8, Dma8, DMA, Dma, 8);
channel!(DMA9, Dma9, DMA, Dma, 9);
channel!(DMA10, Dma10, DMA, Dma, 10);
channel!(DMA11, Dma11, DMA, Dma, 11);
channel!(DMA12, Dma12, DMA, Dma, 12);
channel!(DMA13, Dma13, DMA, Dma, 13);
channel!(DMA14, Dma14, DMA, Dma, 14);
channel!(DMA15, Dma15, DMA, Dma, 15);
