#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::dcmi::*;

periph!( DCMI, Dcmi, _DCMI, DcmiPeriph, 0x50050000);

impl super::sig::Signal<super::sig::DcmiHsync> for Dcmi {}
impl super::sig::SignalSigDcmiHsync<super::sig::DcmiHsync> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiVsync> for Dcmi {}
impl super::sig::SignalSigDcmiVsync<super::sig::DcmiVsync> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiPixclk> for Dcmi {}
impl super::sig::SignalSigDcmiPixclk<super::sig::DcmiPixclk> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD0> for Dcmi {}
impl super::sig::SignalSigDcmiD0<super::sig::DcmiD0> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD1> for Dcmi {}
impl super::sig::SignalSigDcmiD1<super::sig::DcmiD1> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD2> for Dcmi {}
impl super::sig::SignalSigDcmiD2<super::sig::DcmiD2> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD3> for Dcmi {}
impl super::sig::SignalSigDcmiD3<super::sig::DcmiD3> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD4> for Dcmi {}
impl super::sig::SignalSigDcmiD4<super::sig::DcmiD4> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD5> for Dcmi {}
impl super::sig::SignalSigDcmiD5<super::sig::DcmiD5> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD6> for Dcmi {}
impl super::sig::SignalSigDcmiD6<super::sig::DcmiD6> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD7> for Dcmi {}
impl super::sig::SignalSigDcmiD7<super::sig::DcmiD7> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD8> for Dcmi {}
impl super::sig::SignalSigDcmiD8<super::sig::DcmiD8> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD9> for Dcmi {}
impl super::sig::SignalSigDcmiD9<super::sig::DcmiD9> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD10> for Dcmi {}
impl super::sig::SignalSigDcmiD10<super::sig::DcmiD10> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD11> for Dcmi {}
impl super::sig::SignalSigDcmiD11<super::sig::DcmiD11> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD12> for Dcmi {}
impl super::sig::SignalSigDcmiD12<super::sig::DcmiD12> for Dcmi {}
impl super::sig::Signal<super::sig::DcmiD13> for Dcmi {}
impl super::sig::SignalSigDcmiD13<super::sig::DcmiD13> for Dcmi {}



pub trait IrqDcmi<T> {
    fn irq_dcmi(&self) -> T;
}

impl IrqDcmi<super::irq::IrqDcmi> for Dcmi {
    fn irq_dcmi(&self) -> super::irq::IrqDcmi { super::irq::IRQ_DCMI }
}

