#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::i2c::*;

periph!( I2C0, I2c0, _I2C0, I2cPeriph, 0x40066000);
periph!( I2C1, I2c1, _I2C1, I2cPeriph, 0x40067000);

impl super::sig::Signal<super::sig::I2c0Scl> for I2c0 {}
impl super::sig::SignalI2cScl<super::sig::I2c0Scl> for I2c0 {}
impl super::sig::Signal<super::sig::I2c0Sda> for I2c0 {}
impl super::sig::SignalI2cSda<super::sig::I2c0Sda> for I2c0 {}

impl super::sig::Signal<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::SignalI2cScl<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1Sda> for I2c1 {}
impl super::sig::SignalI2cSda<super::sig::I2c1Sda> for I2c1 {}



