#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dma::*;

periph!( DMA, Dma, DMA_PERIPH, DmaPeriph, DMA_OWNED, DMA_REF_COUNT, 0x40008000, 0x00, 0x06);

channel!(DMA0, Dma0, dma0, DMA, Dma, DMA0_CH, DmaCh, DMA_PERIPH, DMA0_OWNED, DMA0_REF_COUNT, 0);
channel!(DMA1, Dma1, dma1, DMA, Dma, DMA1_CH, DmaCh, DMA_PERIPH, DMA1_OWNED, DMA1_REF_COUNT, 1);
channel!(DMA2, Dma2, dma2, DMA, Dma, DMA2_CH, DmaCh, DMA_PERIPH, DMA2_OWNED, DMA2_REF_COUNT, 2);
channel!(DMA3, Dma3, dma3, DMA, Dma, DMA3_CH, DmaCh, DMA_PERIPH, DMA3_OWNED, DMA3_REF_COUNT, 3);
