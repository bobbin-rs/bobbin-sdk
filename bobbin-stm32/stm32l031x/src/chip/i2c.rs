#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::i2c_v2::*;

periph!( I2C1, I2c1, _I2C1, I2cPeriph, 0x40005400);

impl super::sig::Signal<super::sig::I2c1Smba> for I2c1 {}
impl super::sig::SignalI2cSmba<super::sig::I2c1Smba> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::SignalI2cScl<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1Sda> for I2c1 {}
impl super::sig::SignalI2cSda<super::sig::I2c1Sda> for I2c1 {}



pub trait IrqI2c<T> {
    fn irq_i2c(&self) -> T;
}

impl IrqI2c<super::irq::IrqI2c1> for I2c1 {
    fn irq_i2c(&self) -> super::irq::IrqI2c1 { super::irq::IRQ_I2C1 }
}

