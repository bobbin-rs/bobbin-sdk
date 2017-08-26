#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::dma::*;

periph!(DmaPeriph, DMA, Dma, 0x40008000);



channel!(DMA0, Dma0, DMA, Dma, 0);
channel!(DMA1, Dma1, DMA, Dma, 1);
channel!(DMA2, Dma2, DMA, Dma, 2);
channel!(DMA3, Dma3, DMA, Dma, 3);
