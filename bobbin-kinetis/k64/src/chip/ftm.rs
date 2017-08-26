#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::ftm::*;

periph!(FtmPeriph, FTM0, Ftm0, 0x40038000);
periph!(FtmPeriph, FTM1, Ftm1, 0x40039000);
periph!(FtmPeriph, FTM2, Ftm2, 0x4003a000);

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


channel!(FTM0_CH0, Ftm0Ch0, FTM0, Ftm0, 0);
channel!(FTM0_CH1, Ftm0Ch1, FTM0, Ftm0, 1);
channel!(FTM0_CH2, Ftm0Ch2, FTM0, Ftm0, 2);
channel!(FTM0_CH3, Ftm0Ch3, FTM0, Ftm0, 3);
channel!(FTM0_CH4, Ftm0Ch4, FTM0, Ftm0, 4);
channel!(FTM0_CH5, Ftm0Ch5, FTM0, Ftm0, 5);
channel!(FTM0_CH6, Ftm0Ch6, FTM0, Ftm0, 6);
channel!(FTM0_CH7, Ftm0Ch7, FTM0, Ftm0, 7);
channel!(FTM1_CH0, Ftm1Ch0, FTM1, Ftm1, 0);
channel!(FTM1_CH1, Ftm1Ch1, FTM1, Ftm1, 1);
channel!(FTM1_CH2, Ftm1Ch2, FTM1, Ftm1, 2);
channel!(FTM1_CH3, Ftm1Ch3, FTM1, Ftm1, 3);
channel!(FTM1_CH4, Ftm1Ch4, FTM1, Ftm1, 4);
channel!(FTM1_CH5, Ftm1Ch5, FTM1, Ftm1, 5);
channel!(FTM1_CH6, Ftm1Ch6, FTM1, Ftm1, 6);
channel!(FTM1_CH7, Ftm1Ch7, FTM1, Ftm1, 7);
channel!(FTM2_CH0, Ftm2Ch0, FTM2, Ftm2, 0);
channel!(FTM2_CH1, Ftm2Ch1, FTM2, Ftm2, 1);
channel!(FTM2_CH2, Ftm2Ch2, FTM2, Ftm2, 2);
channel!(FTM2_CH3, Ftm2Ch3, FTM2, Ftm2, 3);
channel!(FTM2_CH4, Ftm2Ch4, FTM2, Ftm2, 4);
channel!(FTM2_CH5, Ftm2Ch5, FTM2, Ftm2, 5);
channel!(FTM2_CH6, Ftm2Ch6, FTM2, Ftm2, 6);
channel!(FTM2_CH7, Ftm2Ch7, FTM2, Ftm2, 7);
