#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::ethernet_dma::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( ETHERNET_DMA, EthernetDma, ETHERNET_DMA_PERIPH, EthernetDmaPeriph, ETHERNET_DMA_OWNED, 0x40029000, 0x00, 0x08);


