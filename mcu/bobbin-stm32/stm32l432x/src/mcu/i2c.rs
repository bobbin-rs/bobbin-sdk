#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::i2c::*;

periph!( I2C1, I2c1, I2C1_PERIPH, I2cPeriph, 0x40005400, 0x0f);
periph!( I2C2, I2c2, I2C2_PERIPH, I2cPeriph, 0x40005800, 0x10);
periph!( I2C3, I2c3, I2C3_PERIPH, I2cPeriph, 0x40005c00, 0x11);
