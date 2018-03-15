#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::ethernet_mmc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( ETHERNET_MMC, EthernetMmc, ETHERNET_MMC_PERIPH, EthernetMmcPeriph, 0x40028100, 0x06);


