#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::i2c::*;

periph!( I2C0, I2c0, I2C0_PERIPH, I2cPeriph, I2C0_OWNED, I2C0_REF_COUNT, 0x40066000, 0x00, 0x0e);
periph!( I2C1, I2c1, I2C1_PERIPH, I2cPeriph, I2C1_OWNED, I2C1_REF_COUNT, 0x40067000, 0x01, 0x0f);

