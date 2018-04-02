#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dma2d::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( DMA2D, Dma2d, DMA2D_PERIPH, Dma2dPeriph, DMA2D_OWNED, DMA2D_REF_COUNT, 0x4002b000, 0x00, 0x0e);


