#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::aes::*;

periph!( AES, Aes, AES_PERIPH, AesPeriph, 0x50060000, 0x00, 0x13);

