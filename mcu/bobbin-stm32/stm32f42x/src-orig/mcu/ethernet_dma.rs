#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::ethernet_dma::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( ETHERNET_DMA, EthernetDma, ETHERNET_DMA_PERIPH, EthernetDmaPeriph, 0x40029000, 0x07);


