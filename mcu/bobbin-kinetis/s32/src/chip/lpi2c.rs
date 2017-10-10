#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::lpi2c::*;

periph!( LPI2C0, Lpi2c0, _LPI2C0, Lpi2cPeriph, 0x40066000);

impl super::sig::Signal<super::sig::Lpi2c0Hreq> for Lpi2c0 {}
impl super::sig::SignalI2cHreq<super::sig::Lpi2c0Hreq> for Lpi2c0 {}
impl super::sig::Signal<super::sig::Lpi2c0Scl> for Lpi2c0 {}
impl super::sig::SignalI2cScl<super::sig::Lpi2c0Scl> for Lpi2c0 {}
impl super::sig::Signal<super::sig::Lpi2c0Sda> for Lpi2c0 {}
impl super::sig::SignalI2cSda<super::sig::Lpi2c0Sda> for Lpi2c0 {}
impl super::sig::Signal<super::sig::Lpi2c0Scls> for Lpi2c0 {}
impl super::sig::SignalI2cScls<super::sig::Lpi2c0Scls> for Lpi2c0 {}
impl super::sig::Signal<super::sig::Lpi2c0Sdas> for Lpi2c0 {}
impl super::sig::SignalI2cSdas<super::sig::Lpi2c0Sdas> for Lpi2c0 {}



pub trait IrqLpi2cMaster<T> {
    fn irq_lpi2c_master(&self) -> T;
}

pub trait IrqLpi2cSlave<T> {
    fn irq_lpi2c_slave(&self) -> T;
}

impl IrqLpi2cMaster<super::irq::IrqLpi2c0Master> for Lpi2c0 {
    fn irq_lpi2c_master(&self) -> super::irq::IrqLpi2c0Master { super::irq::IRQ_LPI2C0_MASTER }
}

impl IrqLpi2cSlave<super::irq::IrqLpi2c0Slave> for Lpi2c0 {
    fn irq_lpi2c_slave(&self) -> super::irq::IrqLpi2c0Slave { super::irq::IRQ_LPI2C0_SLAVE }
}

