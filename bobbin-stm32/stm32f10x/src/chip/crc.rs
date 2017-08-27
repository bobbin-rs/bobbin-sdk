#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::crc_24::*;

periph!(_CRC, CrcPeriph, CRC, Crc, 0x40023000);



