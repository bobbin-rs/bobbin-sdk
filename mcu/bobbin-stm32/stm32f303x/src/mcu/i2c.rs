#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::i2c::*;

periph!( I2C1, I2c1, I2C1_PERIPH, I2cPeriph, I2C1_OWNED, I2C1_REF_COUNT, 0x40005400, 0x00, 0x16);
periph!( I2C2, I2c2, I2C2_PERIPH, I2cPeriph, I2C2_OWNED, I2C2_REF_COUNT, 0x40005800, 0x01, 0x17);
periph!( I2C3, I2c3, I2C3_PERIPH, I2cPeriph, I2C3_OWNED, I2C3_REF_COUNT, 0x40007800, 0x02, 0x18);

