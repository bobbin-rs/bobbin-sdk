#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::crc::*;

periph!( CRC, Crc, CRC_PERIPH, CrcPeriph, 0x40023000, 0x07);

