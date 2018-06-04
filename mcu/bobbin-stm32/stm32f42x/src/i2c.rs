pub use ::stm32_common::i2c_v1::*;

::bobbin_mcu::periph!( I2C3, I2c3, I2C3_PERIPH, I2cPeriph, I2C3_OWNED, I2C3_REF_COUNT, 0x40005c00, 0x00, 0x27);
::bobbin_mcu::periph!( I2C2, I2c2, I2C2_PERIPH, I2cPeriph, I2C2_OWNED, I2C2_REF_COUNT, 0x40005800, 0x01, 0x28);
::bobbin_mcu::periph!( I2C1, I2c1, I2C1_PERIPH, I2cPeriph, I2C1_OWNED, I2C1_REF_COUNT, 0x40005400, 0x02, 0x29);

