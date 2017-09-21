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



