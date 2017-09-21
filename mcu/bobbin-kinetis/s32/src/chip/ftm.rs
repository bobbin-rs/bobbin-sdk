#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::ftm::*;

periph!( FTM0, Ftm0, _FTM0, FtmPeriph, 0x40038000);
periph!( FTM1, Ftm1, _FTM1, FtmPeriph, 0x40039000);
periph!( FTM2, Ftm2, _FTM2, FtmPeriph, 0x4003a000);
periph!( FTM3, Ftm3, _FTM3, FtmPeriph, 0x40026000);

impl super::sig::Signal<super::sig::Ftm0Ch0> for Ftm0Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch0> for Ftm0Ch0 {}
impl super::sig::Signal<super::sig::Ftm0Ch1> for Ftm0Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch1> for Ftm0Ch1 {}
impl super::sig::Signal<super::sig::Ftm0Ch2> for Ftm0Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch2> for Ftm0Ch2 {}
impl super::sig::Signal<super::sig::Ftm0Ch3> for Ftm0Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch3> for Ftm0Ch3 {}
impl super::sig::Signal<super::sig::Ftm0Ch4> for Ftm0Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch4> for Ftm0Ch4 {}
impl super::sig::Signal<super::sig::Ftm0Ch5> for Ftm0Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch5> for Ftm0Ch5 {}
impl super::sig::Signal<super::sig::Ftm0Ch6> for Ftm0Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch6> for Ftm0Ch6 {}
impl super::sig::Signal<super::sig::Ftm0Ch7> for Ftm0Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch7> for Ftm0Ch7 {}

impl super::sig::Signal<super::sig::Ftm1Ch0> for Ftm1Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch0> for Ftm1Ch0 {}
impl super::sig::Signal<super::sig::Ftm1Ch1> for Ftm1Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch1> for Ftm1Ch1 {}
impl super::sig::Signal<super::sig::Ftm1Ch2> for Ftm1Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch2> for Ftm1Ch2 {}
impl super::sig::Signal<super::sig::Ftm1Ch3> for Ftm1Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch3> for Ftm1Ch3 {}
impl super::sig::Signal<super::sig::Ftm1Ch4> for Ftm1Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch4> for Ftm1Ch4 {}
impl super::sig::Signal<super::sig::Ftm1Ch5> for Ftm1Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch5> for Ftm1Ch5 {}
impl super::sig::Signal<super::sig::Ftm1Ch6> for Ftm1Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch6> for Ftm1Ch6 {}
impl super::sig::Signal<super::sig::Ftm1Ch7> for Ftm1Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch7> for Ftm1Ch7 {}

impl super::sig::Signal<super::sig::Ftm2Ch0> for Ftm2Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch0> for Ftm2Ch0 {}
impl super::sig::Signal<super::sig::Ftm2Ch1> for Ftm2Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch1> for Ftm2Ch1 {}
impl super::sig::Signal<super::sig::Ftm2Ch2> for Ftm2Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch2> for Ftm2Ch2 {}
impl super::sig::Signal<super::sig::Ftm2Ch3> for Ftm2Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch3> for Ftm2Ch3 {}
impl super::sig::Signal<super::sig::Ftm2Ch4> for Ftm2Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch4> for Ftm2Ch4 {}
impl super::sig::Signal<super::sig::Ftm2Ch5> for Ftm2Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch5> for Ftm2Ch5 {}
impl super::sig::Signal<super::sig::Ftm2Ch6> for Ftm2Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch6> for Ftm2Ch6 {}
impl super::sig::Signal<super::sig::Ftm2Ch7> for Ftm2Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch7> for Ftm2Ch7 {}

impl super::sig::Signal<super::sig::Ftm3Ch0> for Ftm3Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch0> for Ftm3Ch0 {}
impl super::sig::Signal<super::sig::Ftm3Ch1> for Ftm3Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch1> for Ftm3Ch1 {}
impl super::sig::Signal<super::sig::Ftm3Ch2> for Ftm3Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch2> for Ftm3Ch2 {}
impl super::sig::Signal<super::sig::Ftm3Ch3> for Ftm3Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch3> for Ftm3Ch3 {}
impl super::sig::Signal<super::sig::Ftm3Ch4> for Ftm3Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch4> for Ftm3Ch4 {}
impl super::sig::Signal<super::sig::Ftm3Ch5> for Ftm3Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch5> for Ftm3Ch5 {}
impl super::sig::Signal<super::sig::Ftm3Ch6> for Ftm3Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch6> for Ftm3Ch6 {}
impl super::sig::Signal<super::sig::Ftm3Ch7> for Ftm3Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm3Ch7> for Ftm3Ch7 {}


channel!(FTM0_CH0, Ftm0Ch0, FTM0, Ftm0, _FTM0_CH0, FtmCh, _FTM0, 0);
channel!(FTM0_CH1, Ftm0Ch1, FTM0, Ftm0, _FTM0_CH1, FtmCh, _FTM0, 1);
channel!(FTM0_CH2, Ftm0Ch2, FTM0, Ftm0, _FTM0_CH2, FtmCh, _FTM0, 2);
channel!(FTM0_CH3, Ftm0Ch3, FTM0, Ftm0, _FTM0_CH3, FtmCh, _FTM0, 3);
channel!(FTM0_CH4, Ftm0Ch4, FTM0, Ftm0, _FTM0_CH4, FtmCh, _FTM0, 4);
channel!(FTM0_CH5, Ftm0Ch5, FTM0, Ftm0, _FTM0_CH5, FtmCh, _FTM0, 5);
channel!(FTM0_CH6, Ftm0Ch6, FTM0, Ftm0, _FTM0_CH6, FtmCh, _FTM0, 6);
channel!(FTM0_CH7, Ftm0Ch7, FTM0, Ftm0, _FTM0_CH7, FtmCh, _FTM0, 7);
channel!(FTM1_CH0, Ftm1Ch0, FTM1, Ftm1, _FTM1_CH0, FtmCh, _FTM1, 0);
channel!(FTM1_CH1, Ftm1Ch1, FTM1, Ftm1, _FTM1_CH1, FtmCh, _FTM1, 1);
channel!(FTM1_CH2, Ftm1Ch2, FTM1, Ftm1, _FTM1_CH2, FtmCh, _FTM1, 2);
channel!(FTM1_CH3, Ftm1Ch3, FTM1, Ftm1, _FTM1_CH3, FtmCh, _FTM1, 3);
channel!(FTM1_CH4, Ftm1Ch4, FTM1, Ftm1, _FTM1_CH4, FtmCh, _FTM1, 4);
channel!(FTM1_CH5, Ftm1Ch5, FTM1, Ftm1, _FTM1_CH5, FtmCh, _FTM1, 5);
channel!(FTM1_CH6, Ftm1Ch6, FTM1, Ftm1, _FTM1_CH6, FtmCh, _FTM1, 6);
channel!(FTM1_CH7, Ftm1Ch7, FTM1, Ftm1, _FTM1_CH7, FtmCh, _FTM1, 7);
channel!(FTM2_CH0, Ftm2Ch0, FTM2, Ftm2, _FTM2_CH0, FtmCh, _FTM2, 0);
channel!(FTM2_CH1, Ftm2Ch1, FTM2, Ftm2, _FTM2_CH1, FtmCh, _FTM2, 1);
channel!(FTM2_CH2, Ftm2Ch2, FTM2, Ftm2, _FTM2_CH2, FtmCh, _FTM2, 2);
channel!(FTM2_CH3, Ftm2Ch3, FTM2, Ftm2, _FTM2_CH3, FtmCh, _FTM2, 3);
channel!(FTM2_CH4, Ftm2Ch4, FTM2, Ftm2, _FTM2_CH4, FtmCh, _FTM2, 4);
channel!(FTM2_CH5, Ftm2Ch5, FTM2, Ftm2, _FTM2_CH5, FtmCh, _FTM2, 5);
channel!(FTM2_CH6, Ftm2Ch6, FTM2, Ftm2, _FTM2_CH6, FtmCh, _FTM2, 6);
channel!(FTM2_CH7, Ftm2Ch7, FTM2, Ftm2, _FTM2_CH7, FtmCh, _FTM2, 7);
channel!(FTM3_CH0, Ftm3Ch0, FTM3, Ftm3, _FTM3_CH0, FtmCh, _FTM3, 0);
channel!(FTM3_CH1, Ftm3Ch1, FTM3, Ftm3, _FTM3_CH1, FtmCh, _FTM3, 1);
channel!(FTM3_CH2, Ftm3Ch2, FTM3, Ftm3, _FTM3_CH2, FtmCh, _FTM3, 2);
channel!(FTM3_CH3, Ftm3Ch3, FTM3, Ftm3, _FTM3_CH3, FtmCh, _FTM3, 3);
channel!(FTM3_CH4, Ftm3Ch4, FTM3, Ftm3, _FTM3_CH4, FtmCh, _FTM3, 4);
channel!(FTM3_CH5, Ftm3Ch5, FTM3, Ftm3, _FTM3_CH5, FtmCh, _FTM3, 5);
channel!(FTM3_CH6, Ftm3Ch6, FTM3, Ftm3, _FTM3_CH6, FtmCh, _FTM3, 6);
channel!(FTM3_CH7, Ftm3Ch7, FTM3, Ftm3, _FTM3_CH7, FtmCh, _FTM3, 7);

pub trait IrqFtmFault<T> {
    fn irq_ftm_fault(&self) -> T;
}

pub trait IrqFtmOverflow<T> {
    fn irq_ftm_overflow(&self) -> T;
}

pub trait IrqFtm<T> {
    fn irq_ftm(&self) -> T;
}

impl IrqFtmFault<super::irq::IrqFtm0Fault> for Ftm0 {
    fn irq_ftm_fault(&self) -> super::irq::IrqFtm0Fault { super::irq::IRQ_FTM0_FAULT }
}

impl IrqFtmOverflow<super::irq::IrqFtm0Overflow> for Ftm0 {
    fn irq_ftm_overflow(&self) -> super::irq::IrqFtm0Overflow { super::irq::IRQ_FTM0_OVERFLOW }
}

impl IrqFtm<super::irq::IrqFtm0Ch0> for Ftm0Ch0 {
    fn irq_ftm(&self) -> super::irq::IrqFtm0Ch0 { super::irq::IRQ_FTM0_CH0 }
}

impl IrqFtm<super::irq::IrqFtm0Ch1> for Ftm0Ch1 {
    fn irq_ftm(&self) -> super::irq::IrqFtm0Ch1 { super::irq::IRQ_FTM0_CH1 }
}

impl IrqFtm<super::irq::IrqFtm0Ch2> for Ftm0Ch2 {
    fn irq_ftm(&self) -> super::irq::IrqFtm0Ch2 { super::irq::IRQ_FTM0_CH2 }
}

impl IrqFtm<super::irq::IrqFtm0Ch3> for Ftm0Ch3 {
    fn irq_ftm(&self) -> super::irq::IrqFtm0Ch3 { super::irq::IRQ_FTM0_CH3 }
}

impl IrqFtm<super::irq::IrqFtm0Ch4> for Ftm0Ch4 {
    fn irq_ftm(&self) -> super::irq::IrqFtm0Ch4 { super::irq::IRQ_FTM0_CH4 }
}

impl IrqFtm<super::irq::IrqFtm0Ch5> for Ftm0Ch5 {
    fn irq_ftm(&self) -> super::irq::IrqFtm0Ch5 { super::irq::IRQ_FTM0_CH5 }
}

impl IrqFtm<super::irq::IrqFtm0Ch6> for Ftm0Ch6 {
    fn irq_ftm(&self) -> super::irq::IrqFtm0Ch6 { super::irq::IRQ_FTM0_CH6 }
}

impl IrqFtm<super::irq::IrqFtm0Ch7> for Ftm0Ch7 {
    fn irq_ftm(&self) -> super::irq::IrqFtm0Ch7 { super::irq::IRQ_FTM0_CH7 }
}

impl IrqFtmFault<super::irq::IrqFtm1Fault> for Ftm1 {
    fn irq_ftm_fault(&self) -> super::irq::IrqFtm1Fault { super::irq::IRQ_FTM1_FAULT }
}

impl IrqFtmOverflow<super::irq::IrqFtm1Overflow> for Ftm1 {
    fn irq_ftm_overflow(&self) -> super::irq::IrqFtm1Overflow { super::irq::IRQ_FTM1_OVERFLOW }
}

impl IrqFtm<super::irq::IrqFtm1Ch0> for Ftm1Ch0 {
    fn irq_ftm(&self) -> super::irq::IrqFtm1Ch0 { super::irq::IRQ_FTM1_CH0 }
}

impl IrqFtm<super::irq::IrqFtm1Ch1> for Ftm1Ch1 {
    fn irq_ftm(&self) -> super::irq::IrqFtm1Ch1 { super::irq::IRQ_FTM1_CH1 }
}

impl IrqFtm<super::irq::IrqFtm1Ch2> for Ftm1Ch2 {
    fn irq_ftm(&self) -> super::irq::IrqFtm1Ch2 { super::irq::IRQ_FTM1_CH2 }
}

impl IrqFtm<super::irq::IrqFtm1Ch3> for Ftm1Ch3 {
    fn irq_ftm(&self) -> super::irq::IrqFtm1Ch3 { super::irq::IRQ_FTM1_CH3 }
}

impl IrqFtm<super::irq::IrqFtm1Ch4> for Ftm1Ch4 {
    fn irq_ftm(&self) -> super::irq::IrqFtm1Ch4 { super::irq::IRQ_FTM1_CH4 }
}

impl IrqFtm<super::irq::IrqFtm1Ch5> for Ftm1Ch5 {
    fn irq_ftm(&self) -> super::irq::IrqFtm1Ch5 { super::irq::IRQ_FTM1_CH5 }
}

impl IrqFtm<super::irq::IrqFtm1Ch6> for Ftm1Ch6 {
    fn irq_ftm(&self) -> super::irq::IrqFtm1Ch6 { super::irq::IRQ_FTM1_CH6 }
}

impl IrqFtm<super::irq::IrqFtm1Ch7> for Ftm1Ch7 {
    fn irq_ftm(&self) -> super::irq::IrqFtm1Ch7 { super::irq::IRQ_FTM1_CH7 }
}

impl IrqFtmFault<super::irq::IrqFtm2Fault> for Ftm2 {
    fn irq_ftm_fault(&self) -> super::irq::IrqFtm2Fault { super::irq::IRQ_FTM2_FAULT }
}

impl IrqFtmOverflow<super::irq::IrqFtm2Overflow> for Ftm2 {
    fn irq_ftm_overflow(&self) -> super::irq::IrqFtm2Overflow { super::irq::IRQ_FTM2_OVERFLOW }
}

impl IrqFtm<super::irq::IrqFtm2Ch0> for Ftm2Ch0 {
    fn irq_ftm(&self) -> super::irq::IrqFtm2Ch0 { super::irq::IRQ_FTM2_CH0 }
}

impl IrqFtm<super::irq::IrqFtm2Ch1> for Ftm2Ch1 {
    fn irq_ftm(&self) -> super::irq::IrqFtm2Ch1 { super::irq::IRQ_FTM2_CH1 }
}

impl IrqFtm<super::irq::IrqFtm2Ch2> for Ftm2Ch2 {
    fn irq_ftm(&self) -> super::irq::IrqFtm2Ch2 { super::irq::IRQ_FTM2_CH2 }
}

impl IrqFtm<super::irq::IrqFtm2Ch3> for Ftm2Ch3 {
    fn irq_ftm(&self) -> super::irq::IrqFtm2Ch3 { super::irq::IRQ_FTM2_CH3 }
}

impl IrqFtm<super::irq::IrqFtm2Ch4> for Ftm2Ch4 {
    fn irq_ftm(&self) -> super::irq::IrqFtm2Ch4 { super::irq::IRQ_FTM2_CH4 }
}

impl IrqFtm<super::irq::IrqFtm2Ch5> for Ftm2Ch5 {
    fn irq_ftm(&self) -> super::irq::IrqFtm2Ch5 { super::irq::IRQ_FTM2_CH5 }
}

impl IrqFtm<super::irq::IrqFtm2Ch6> for Ftm2Ch6 {
    fn irq_ftm(&self) -> super::irq::IrqFtm2Ch6 { super::irq::IRQ_FTM2_CH6 }
}

impl IrqFtm<super::irq::IrqFtm2Ch7> for Ftm2Ch7 {
    fn irq_ftm(&self) -> super::irq::IrqFtm2Ch7 { super::irq::IRQ_FTM2_CH7 }
}

impl IrqFtmFault<super::irq::IrqFtm3Fault> for Ftm3 {
    fn irq_ftm_fault(&self) -> super::irq::IrqFtm3Fault { super::irq::IRQ_FTM3_FAULT }
}

impl IrqFtmOverflow<super::irq::IrqFtm3Overflow> for Ftm3 {
    fn irq_ftm_overflow(&self) -> super::irq::IrqFtm3Overflow { super::irq::IRQ_FTM3_OVERFLOW }
}

impl IrqFtm<super::irq::IrqFtm3Ch0> for Ftm3Ch0 {
    fn irq_ftm(&self) -> super::irq::IrqFtm3Ch0 { super::irq::IRQ_FTM3_CH0 }
}

impl IrqFtm<super::irq::IrqFtm3Ch1> for Ftm3Ch1 {
    fn irq_ftm(&self) -> super::irq::IrqFtm3Ch1 { super::irq::IRQ_FTM3_CH1 }
}

impl IrqFtm<super::irq::IrqFtm3Ch2> for Ftm3Ch2 {
    fn irq_ftm(&self) -> super::irq::IrqFtm3Ch2 { super::irq::IRQ_FTM3_CH2 }
}

impl IrqFtm<super::irq::IrqFtm3Ch3> for Ftm3Ch3 {
    fn irq_ftm(&self) -> super::irq::IrqFtm3Ch3 { super::irq::IRQ_FTM3_CH3 }
}

impl IrqFtm<super::irq::IrqFtm3Ch4> for Ftm3Ch4 {
    fn irq_ftm(&self) -> super::irq::IrqFtm3Ch4 { super::irq::IRQ_FTM3_CH4 }
}

impl IrqFtm<super::irq::IrqFtm3Ch5> for Ftm3Ch5 {
    fn irq_ftm(&self) -> super::irq::IrqFtm3Ch5 { super::irq::IRQ_FTM3_CH5 }
}

impl IrqFtm<super::irq::IrqFtm3Ch6> for Ftm3Ch6 {
    fn irq_ftm(&self) -> super::irq::IrqFtm3Ch6 { super::irq::IRQ_FTM3_CH6 }
}

impl IrqFtm<super::irq::IrqFtm3Ch7> for Ftm3Ch7 {
    fn irq_ftm(&self) -> super::irq::IrqFtm3Ch7 { super::irq::IRQ_FTM3_CH7 }
}

