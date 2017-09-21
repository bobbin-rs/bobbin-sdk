#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::i2c_v1::*;

periph!( I2C2, I2c2, _I2C2, I2cPeriph, 0x40005800);
periph!( I2C1, I2c1, _I2C1, I2cPeriph, 0x40005400);





