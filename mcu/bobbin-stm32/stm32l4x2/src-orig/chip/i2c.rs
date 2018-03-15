#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::i2c_v2::*;

periph!( I2C1, I2c1, _I2C1, I2cPeriph, 0x40005400);
periph!( I2C2, I2c2, _I2C2, I2cPeriph, 0x40005800);
periph!( I2C3, I2c3, _I2C3, I2cPeriph, 0x40005c00);
periph!( I2C4, I2c4, _I2C4, I2cPeriph, 0x40008400);

impl super::sig::Signal<super::sig::I2c1Smba> for I2c1 {}
impl super::sig::SignalI2cSmba<super::sig::I2c1Smba> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::SignalI2cScl<super::sig::I2c1Scl> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1Sda> for I2c1 {}
impl super::sig::SignalI2cSda<super::sig::I2c1Sda> for I2c1 {}

impl super::sig::Signal<super::sig::I2c2Smba> for I2c2 {}
impl super::sig::SignalI2cSmba<super::sig::I2c2Smba> for I2c2 {}
impl super::sig::Signal<super::sig::I2c2Scl> for I2c2 {}
impl super::sig::SignalI2cScl<super::sig::I2c2Scl> for I2c2 {}
impl super::sig::Signal<super::sig::I2c2Sda> for I2c2 {}
impl super::sig::SignalI2cSda<super::sig::I2c2Sda> for I2c2 {}

impl super::sig::Signal<super::sig::I2c3Smba> for I2c3 {}
impl super::sig::SignalI2cSmba<super::sig::I2c3Smba> for I2c3 {}
impl super::sig::Signal<super::sig::I2c3Scl> for I2c3 {}
impl super::sig::SignalI2cScl<super::sig::I2c3Scl> for I2c3 {}
impl super::sig::Signal<super::sig::I2c3Sda> for I2c3 {}
impl super::sig::SignalI2cSda<super::sig::I2c3Sda> for I2c3 {}

impl super::sig::Signal<super::sig::I2c4Smba> for I2c4 {}
impl super::sig::SignalI2cSmba<super::sig::I2c4Smba> for I2c4 {}
impl super::sig::Signal<super::sig::I2c4Scl> for I2c4 {}
impl super::sig::SignalI2cScl<super::sig::I2c4Scl> for I2c4 {}
impl super::sig::Signal<super::sig::I2c4Sda> for I2c4 {}
impl super::sig::SignalI2cSda<super::sig::I2c4Sda> for I2c4 {}



pub trait IrqI2cEv<T> {
    fn irq_i2c_ev(&self) -> T;
}

pub trait IrqI2cEr<T> {
    fn irq_i2c_er(&self) -> T;
}

impl IrqI2cEv<super::irq::IrqI2c1Ev> for I2c1 {
    fn irq_i2c_ev(&self) -> super::irq::IrqI2c1Ev { super::irq::IRQ_I2C1_EV }
}

impl IrqI2cEr<super::irq::IrqI2c1Er> for I2c1 {
    fn irq_i2c_er(&self) -> super::irq::IrqI2c1Er { super::irq::IRQ_I2C1_ER }
}

impl IrqI2cEv<super::irq::IrqI2c2Ev> for I2c2 {
    fn irq_i2c_ev(&self) -> super::irq::IrqI2c2Ev { super::irq::IRQ_I2C2_EV }
}

impl IrqI2cEr<super::irq::IrqI2c2Er> for I2c2 {
    fn irq_i2c_er(&self) -> super::irq::IrqI2c2Er { super::irq::IRQ_I2C2_ER }
}

impl IrqI2cEv<super::irq::IrqI2c3Ev> for I2c3 {
    fn irq_i2c_ev(&self) -> super::irq::IrqI2c3Ev { super::irq::IRQ_I2C3_EV }
}

impl IrqI2cEr<super::irq::IrqI2c3Er> for I2c3 {
    fn irq_i2c_er(&self) -> super::irq::IrqI2c3Er { super::irq::IRQ_I2C3_ER }
}

impl IrqI2cEv<super::irq::IrqI2c4Ev> for I2c4 {
    fn irq_i2c_ev(&self) -> super::irq::IrqI2c4Ev { super::irq::IRQ_I2C4_EV }
}

impl IrqI2cEr<super::irq::IrqI2c4Er> for I2c4 {
    fn irq_i2c_er(&self) -> super::irq::IrqI2c4Er { super::irq::IRQ_I2C4_ER }
}

