pub use stm32_common::chip::crc_24::*;

pub const CRC: Crc = Periph(0x40023000, CrcId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CrcId {}
pub type Crc = Periph<CrcId>;



