#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::crc_24::*;

pub const CRC: Crc = Periph(0x40023000, CrcId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct CrcId {}
pub type Crc = Periph<CrcId>;



