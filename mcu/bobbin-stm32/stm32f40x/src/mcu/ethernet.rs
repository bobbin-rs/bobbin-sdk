#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::ethernet::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( ETHERNET_MAC, Ethernet, ETHERNET_MAC_PERIPH, EthernetPeriph, 0x40028000, 0x05);


