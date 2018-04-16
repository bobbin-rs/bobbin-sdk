#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::edma::*;

periph!( DMA, Dma, DMA_PERIPH, EdmaPeriph, DMA_OWNED, DMA_REF_COUNT, 0x40008000, 0x00, 0x05);

channel!(DMA0, Dma0, dma0, DMA, Dma, DMA0_CH, EdmaCh, DMA_PERIPH, DMA0_OWNED, DMA0_REF_COUNT, 0);
channel!(DMA1, Dma1, dma1, DMA, Dma, DMA1_CH, EdmaCh, DMA_PERIPH, DMA1_OWNED, DMA1_REF_COUNT, 1);
channel!(DMA2, Dma2, dma2, DMA, Dma, DMA2_CH, EdmaCh, DMA_PERIPH, DMA2_OWNED, DMA2_REF_COUNT, 2);
channel!(DMA3, Dma3, dma3, DMA, Dma, DMA3_CH, EdmaCh, DMA_PERIPH, DMA3_OWNED, DMA3_REF_COUNT, 3);
channel!(DMA4, Dma4, dma4, DMA, Dma, DMA4_CH, EdmaCh, DMA_PERIPH, DMA4_OWNED, DMA4_REF_COUNT, 4);
channel!(DMA5, Dma5, dma5, DMA, Dma, DMA5_CH, EdmaCh, DMA_PERIPH, DMA5_OWNED, DMA5_REF_COUNT, 5);
channel!(DMA6, Dma6, dma6, DMA, Dma, DMA6_CH, EdmaCh, DMA_PERIPH, DMA6_OWNED, DMA6_REF_COUNT, 6);
channel!(DMA7, Dma7, dma7, DMA, Dma, DMA7_CH, EdmaCh, DMA_PERIPH, DMA7_OWNED, DMA7_REF_COUNT, 7);
channel!(DMA8, Dma8, dma8, DMA, Dma, DMA8_CH, EdmaCh, DMA_PERIPH, DMA8_OWNED, DMA8_REF_COUNT, 8);
channel!(DMA9, Dma9, dma9, DMA, Dma, DMA9_CH, EdmaCh, DMA_PERIPH, DMA9_OWNED, DMA9_REF_COUNT, 9);
channel!(DMA10, Dma10, dma10, DMA, Dma, DMA10_CH, EdmaCh, DMA_PERIPH, DMA10_OWNED, DMA10_REF_COUNT, 10);
channel!(DMA11, Dma11, dma11, DMA, Dma, DMA11_CH, EdmaCh, DMA_PERIPH, DMA11_OWNED, DMA11_REF_COUNT, 11);
channel!(DMA12, Dma12, dma12, DMA, Dma, DMA12_CH, EdmaCh, DMA_PERIPH, DMA12_OWNED, DMA12_REF_COUNT, 12);
channel!(DMA13, Dma13, dma13, DMA, Dma, DMA13_CH, EdmaCh, DMA_PERIPH, DMA13_OWNED, DMA13_REF_COUNT, 13);
channel!(DMA14, Dma14, dma14, DMA, Dma, DMA14_CH, EdmaCh, DMA_PERIPH, DMA14_OWNED, DMA14_REF_COUNT, 14);
channel!(DMA15, Dma15, dma15, DMA, Dma, DMA15_CH, EdmaCh, DMA_PERIPH, DMA15_OWNED, DMA15_REF_COUNT, 15);
