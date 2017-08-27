#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::dma::*;

periph!( DMA, Dma, _DMA, DmaPeriph, 0x40008000);



channel!(DMA0, Dma0, DMA, Dma, _DMA0, DmaCh, _DMA, 0);
channel!(DMA1, Dma1, DMA, Dma, _DMA1, DmaCh, _DMA, 1);
channel!(DMA2, Dma2, DMA, Dma, _DMA2, DmaCh, _DMA, 2);
channel!(DMA3, Dma3, DMA, Dma, _DMA3, DmaCh, _DMA, 3);
