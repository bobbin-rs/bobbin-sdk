#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::crc_24::*;

periph!( CRC, Crc, _CRC, CrcPeriph, 0x40023000);




