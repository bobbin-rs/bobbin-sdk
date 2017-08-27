#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::lpspi::*;

periph!(_LPSPI0, LpspiPeriph, LPSPI0, Lpspi0, 0x4002c000);
periph!(_LPSPI1, LpspiPeriph, LPSPI1, Lpspi1, 0x4002d000);
periph!(_LPSPI2, LpspiPeriph, LPSPI2, Lpspi2, 0x4002e000);

impl super::sig::Signal<super::sig::Lpspi0Sck> for Lpspi0 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi0Sck> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Sout> for Lpspi0 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi0Sout> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Sin> for Lpspi0 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi0Sin> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs0> for Lpspi0 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi0Pcs0> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs1> for Lpspi0 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi0Pcs1> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs2> for Lpspi0 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi0Pcs2> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs3> for Lpspi0 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi0Pcs3> for Lpspi0 {}

impl super::sig::Signal<super::sig::Lpspi1Sck> for Lpspi1 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi1Sck> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Sout> for Lpspi1 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi1Sout> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Sin> for Lpspi1 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi1Sin> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs0> for Lpspi1 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi1Pcs0> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs1> for Lpspi1 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi1Pcs1> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs2> for Lpspi1 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi1Pcs2> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs3> for Lpspi1 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi1Pcs3> for Lpspi1 {}

impl super::sig::Signal<super::sig::Lpspi2Sck> for Lpspi2 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi2Sck> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Sout> for Lpspi2 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi2Sout> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Sin> for Lpspi2 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi2Sin> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs0> for Lpspi2 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi2Pcs0> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs1> for Lpspi2 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi2Pcs1> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs2> for Lpspi2 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi2Pcs2> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs3> for Lpspi2 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi2Pcs3> for Lpspi2 {}


