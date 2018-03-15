#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::aes::*;

periph!( AES, Aes, AES_PERIPH, AesPeriph, 0x50060000, 0x13);

