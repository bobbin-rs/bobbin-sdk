#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::i2c::*;

periph!( I2C0, I2c0, I2C0_PERIPH, I2cPeriph, 0x40066000, 0x0e);
periph!( I2C1, I2c1, I2C1_PERIPH, I2cPeriph, 0x40067000, 0x0f);

