#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::dma2d::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( DMA2D, Dma2d, DMA2D_PERIPH, Dma2dPeriph, 0x4002b000, 0x0f);


