#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::i2c_v2::*;

periph!( I2C1, I2c1, _I2C1, I2cPeriph, 0x40005400);
periph!( I2C2, I2c2, _I2C2, I2cPeriph, 0x40005800);
periph!( I2C3, I2c3, _I2C3, I2cPeriph, 0x40005c00);
periph!( I2C4, I2c4, _I2C4, I2cPeriph, 0x40006000);

impl super::sig::Signal<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::SignalI2cScl<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1Sda> for I2c1 {}
impl super::sig::SignalI2cSda<super::sig::I2c1Sda> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1Smbal> for I2c1 {}
impl super::sig::SignalI2cSmbal<super::sig::I2c1Smbal> for I2c1 {}

impl super::sig::Signal<super::sig::I2c2Scl> for I2c2 {}
impl super::sig::SignalI2cScl<super::sig::I2c2Scl> for I2c2 {}
impl super::sig::Signal<super::sig::I2c2Sda> for I2c2 {}
impl super::sig::SignalI2cSda<super::sig::I2c2Sda> for I2c2 {}
impl super::sig::Signal<super::sig::I2c2Smbal> for I2c2 {}
impl super::sig::SignalI2cSmbal<super::sig::I2c2Smbal> for I2c2 {}

impl super::sig::Signal<super::sig::I2c3Scl> for I2c3 {}
impl super::sig::SignalI2cScl<super::sig::I2c3Scl> for I2c3 {}
impl super::sig::Signal<super::sig::I2c3Sda> for I2c3 {}
impl super::sig::SignalI2cSda<super::sig::I2c3Sda> for I2c3 {}
impl super::sig::Signal<super::sig::I2c3Smbal> for I2c3 {}
impl super::sig::SignalI2cSmbal<super::sig::I2c3Smbal> for I2c3 {}

impl super::sig::Signal<super::sig::I2c4Scl> for I2c4 {}
impl super::sig::SignalI2cScl<super::sig::I2c4Scl> for I2c4 {}
impl super::sig::Signal<super::sig::I2c4Sda> for I2c4 {}
impl super::sig::SignalI2cSda<super::sig::I2c4Sda> for I2c4 {}
impl super::sig::Signal<super::sig::I2c4Smbal> for I2c4 {}
impl super::sig::SignalI2cSmbal<super::sig::I2c4Smbal> for I2c4 {}



