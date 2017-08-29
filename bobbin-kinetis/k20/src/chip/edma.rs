#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::edma::*;

periph!( DMA, Dma, _DMA, EdmaPeriph, 0x40008000);



channel!(DMA0, Dma0, DMA, Dma, _DMA0, EdmaCh, _DMA, 0);
channel!(DMA1, Dma1, DMA, Dma, _DMA1, EdmaCh, _DMA, 1);
channel!(DMA2, Dma2, DMA, Dma, _DMA2, EdmaCh, _DMA, 2);
channel!(DMA3, Dma3, DMA, Dma, _DMA3, EdmaCh, _DMA, 3);
channel!(DMA4, Dma4, DMA, Dma, _DMA4, EdmaCh, _DMA, 4);
channel!(DMA5, Dma5, DMA, Dma, _DMA5, EdmaCh, _DMA, 5);
channel!(DMA6, Dma6, DMA, Dma, _DMA6, EdmaCh, _DMA, 6);
channel!(DMA7, Dma7, DMA, Dma, _DMA7, EdmaCh, _DMA, 7);
channel!(DMA8, Dma8, DMA, Dma, _DMA8, EdmaCh, _DMA, 8);
channel!(DMA9, Dma9, DMA, Dma, _DMA9, EdmaCh, _DMA, 9);
channel!(DMA10, Dma10, DMA, Dma, _DMA10, EdmaCh, _DMA, 10);
channel!(DMA11, Dma11, DMA, Dma, _DMA11, EdmaCh, _DMA, 11);
channel!(DMA12, Dma12, DMA, Dma, _DMA12, EdmaCh, _DMA, 12);
channel!(DMA13, Dma13, DMA, Dma, _DMA13, EdmaCh, _DMA, 13);
channel!(DMA14, Dma14, DMA, Dma, _DMA14, EdmaCh, _DMA, 14);
channel!(DMA15, Dma15, DMA, Dma, _DMA15, EdmaCh, _DMA, 15);

