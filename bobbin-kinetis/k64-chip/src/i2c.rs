pub use kinetis_common::chip::i2c::*;

pub const I2C0: I2c0 = Periph(0x40066000, I2c0Id {});
pub const I2C1: I2c1 = Periph(0x40067000, I2c1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct I2c0Id {}
pub type I2c0 = Periph<I2c0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct I2c1Id {}
pub type I2c1 = Periph<I2c1Id>;




