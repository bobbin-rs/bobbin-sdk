#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::i2c_v2::*;

periph!( I2C1, I2c1, _I2C1, I2cPeriph, 0x40005400);
periph!( I2C2, I2c2, _I2C2, I2cPeriph, 0x40005800);
periph!( I2C3, I2c3, _I2C3, I2cPeriph, 0x40007800);

impl super::sig::Signal<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::SignalI2cScl<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1Sda> for I2c1 {}
impl super::sig::SignalI2cSda<super::sig::I2c1Sda> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1Smbal> for I2c1 {}
impl super::sig::SignalI2cSmbal<super::sig::I2c1Smbal> for I2c1 {}

impl super::sig::Signal<super::sig::I2c1Scl> for I2c2 {}
impl super::sig::SignalI2cScl<super::sig::I2c1Scl> for I2c2 {}
impl super::sig::Signal<super::sig::I2c1Sda> for I2c2 {}
impl super::sig::SignalI2cSda<super::sig::I2c1Sda> for I2c2 {}
impl super::sig::Signal<super::sig::I2c1Smbal> for I2c2 {}
impl super::sig::SignalI2cSmbal<super::sig::I2c1Smbal> for I2c2 {}




