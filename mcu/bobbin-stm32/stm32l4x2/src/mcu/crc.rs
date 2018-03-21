#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::crc::*;

periph!( CRC, Crc, CRC_PERIPH, CrcPeriph, 0x40023000, 0x00, 0x04);

