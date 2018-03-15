#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::i2c::*;

periph!( I2C1, I2c1, I2C1_PERIPH, I2cPeriph, 0x40005400, 0x0b);

