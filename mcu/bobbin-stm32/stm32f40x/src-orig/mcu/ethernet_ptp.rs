#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::ethernet_ptp::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( ETHERNET_PTP, EthernetPtp, ETHERNET_PTP_PERIPH, EthernetPtpPeriph, 0x40028700, 0x06);


