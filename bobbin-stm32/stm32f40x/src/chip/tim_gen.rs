#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_gen::*;

periph!( TIM2, Tim2, _TIM2, TimGenPeriph, 0x40000000);
periph!( TIM3, Tim3, _TIM3, TimGenPeriph, 0x40000400);
periph!( TIM4, Tim4, _TIM4, TimGenPeriph, 0x40000800);
periph!( TIM5, Tim5, _TIM5, TimGenPeriph, 0x40000c00);
periph!( TIM9, Tim9, _TIM9, TimGenPeriph, 0x40014000);
periph!( TIM10, Tim10, _TIM10, TimGenPeriph, 0x40014400);
periph!( TIM11, Tim11, _TIM11, TimGenPeriph, 0x40014800);
periph!( TIM12, Tim12, _TIM12, TimGenPeriph, 0x40001800);
periph!( TIM13, Tim13, _TIM13, TimGenPeriph, 0x40001c00);
periph!( TIM14, Tim14, _TIM14, TimGenPeriph, 0x40002000);

impl super::sig::Signal<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::SignalTim<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::Signal<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::SignalTim<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::Signal<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::SignalTim<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::Signal<super::sig::Tim2Ch4> for Tim2Ch4 {}
impl super::sig::SignalTim<super::sig::Tim2Ch4> for Tim2Ch4 {}

impl super::sig::Signal<super::sig::Tim3Ch1> for Tim3Ch1 {}
impl super::sig::SignalTim<super::sig::Tim3Ch1> for Tim3Ch1 {}
impl super::sig::Signal<super::sig::Tim3Ch2> for Tim3Ch2 {}
impl super::sig::SignalTim<super::sig::Tim3Ch2> for Tim3Ch2 {}
impl super::sig::Signal<super::sig::Tim3Ch3> for Tim3Ch3 {}
impl super::sig::SignalTim<super::sig::Tim3Ch3> for Tim3Ch3 {}
impl super::sig::Signal<super::sig::Tim3Ch4> for Tim3Ch4 {}
impl super::sig::SignalTim<super::sig::Tim3Ch4> for Tim3Ch4 {}

impl super::sig::Signal<super::sig::Tim4Ch1> for Tim4Ch1 {}
impl super::sig::SignalTim<super::sig::Tim4Ch1> for Tim4Ch1 {}
impl super::sig::Signal<super::sig::Tim4Ch2> for Tim4Ch2 {}
impl super::sig::SignalTim<super::sig::Tim4Ch2> for Tim4Ch2 {}
impl super::sig::Signal<super::sig::Tim4Ch3> for Tim4Ch3 {}
impl super::sig::SignalTim<super::sig::Tim4Ch3> for Tim4Ch3 {}
impl super::sig::Signal<super::sig::Tim4Ch4> for Tim4Ch4 {}
impl super::sig::SignalTim<super::sig::Tim4Ch4> for Tim4Ch4 {}

impl super::sig::Signal<super::sig::Tim5Ch1> for Tim5Ch1 {}
impl super::sig::SignalTim<super::sig::Tim5Ch1> for Tim5Ch1 {}
impl super::sig::Signal<super::sig::Tim5Ch2> for Tim5Ch2 {}
impl super::sig::SignalTim<super::sig::Tim5Ch2> for Tim5Ch2 {}
impl super::sig::Signal<super::sig::Tim5Ch3> for Tim5Ch3 {}
impl super::sig::SignalTim<super::sig::Tim5Ch3> for Tim5Ch3 {}
impl super::sig::Signal<super::sig::Tim5Ch4> for Tim5Ch4 {}
impl super::sig::SignalTim<super::sig::Tim5Ch4> for Tim5Ch4 {}

impl super::sig::Signal<super::sig::Tim9Ch1> for Tim9Ch1 {}
impl super::sig::SignalTim<super::sig::Tim9Ch1> for Tim9Ch1 {}
impl super::sig::Signal<super::sig::Tim9Ch2> for Tim9Ch2 {}
impl super::sig::SignalTim<super::sig::Tim9Ch2> for Tim9Ch2 {}

impl super::sig::Signal<super::sig::Tim10Ch1> for Tim10Ch1 {}
impl super::sig::SignalTim<super::sig::Tim10Ch1> for Tim10Ch1 {}

impl super::sig::Signal<super::sig::Tim11Ch1> for Tim11Ch1 {}
impl super::sig::SignalTim<super::sig::Tim11Ch1> for Tim11Ch1 {}

impl super::sig::Signal<super::sig::Tim12Ch1> for Tim12Ch1 {}
impl super::sig::SignalTim<super::sig::Tim12Ch1> for Tim12Ch1 {}
impl super::sig::Signal<super::sig::Tim12Ch2> for Tim12Ch2 {}
impl super::sig::SignalTim<super::sig::Tim12Ch2> for Tim12Ch2 {}

impl super::sig::Signal<super::sig::Tim13Ch1> for Tim13Ch1 {}
impl super::sig::SignalTim<super::sig::Tim13Ch1> for Tim13Ch1 {}

impl super::sig::Signal<super::sig::Tim14Ch1> for Tim14Ch1 {}
impl super::sig::SignalTim<super::sig::Tim14Ch1> for Tim14Ch1 {}


channel!(TIM2_CH1, Tim2Ch1, TIM2, Tim2, _TIM2_CH1, TimGenCh, _TIM2, 0);
channel!(TIM2_CH2, Tim2Ch2, TIM2, Tim2, _TIM2_CH2, TimGenCh, _TIM2, 1);
channel!(TIM2_CH3, Tim2Ch3, TIM2, Tim2, _TIM2_CH3, TimGenCh, _TIM2, 2);
channel!(TIM2_CH4, Tim2Ch4, TIM2, Tim2, _TIM2_CH4, TimGenCh, _TIM2, 3);
channel!(TIM3_CH1, Tim3Ch1, TIM3, Tim3, _TIM3_CH1, TimGenCh, _TIM3, 0);
channel!(TIM3_CH2, Tim3Ch2, TIM3, Tim3, _TIM3_CH2, TimGenCh, _TIM3, 1);
channel!(TIM3_CH3, Tim3Ch3, TIM3, Tim3, _TIM3_CH3, TimGenCh, _TIM3, 2);
channel!(TIM3_CH4, Tim3Ch4, TIM3, Tim3, _TIM3_CH4, TimGenCh, _TIM3, 3);
channel!(TIM4_CH1, Tim4Ch1, TIM4, Tim4, _TIM4_CH1, TimGenCh, _TIM4, 0);
channel!(TIM4_CH2, Tim4Ch2, TIM4, Tim4, _TIM4_CH2, TimGenCh, _TIM4, 1);
channel!(TIM4_CH3, Tim4Ch3, TIM4, Tim4, _TIM4_CH3, TimGenCh, _TIM4, 2);
channel!(TIM4_CH4, Tim4Ch4, TIM4, Tim4, _TIM4_CH4, TimGenCh, _TIM4, 3);
channel!(TIM5_CH1, Tim5Ch1, TIM5, Tim5, _TIM5_CH1, TimGenCh, _TIM5, 0);
channel!(TIM5_CH2, Tim5Ch2, TIM5, Tim5, _TIM5_CH2, TimGenCh, _TIM5, 1);
channel!(TIM5_CH3, Tim5Ch3, TIM5, Tim5, _TIM5_CH3, TimGenCh, _TIM5, 2);
channel!(TIM5_CH4, Tim5Ch4, TIM5, Tim5, _TIM5_CH4, TimGenCh, _TIM5, 3);
channel!(TIM9_CH1, Tim9Ch1, TIM9, Tim9, _TIM9_CH1, TimGenCh, _TIM9, 0);
channel!(TIM9_CH2, Tim9Ch2, TIM9, Tim9, _TIM9_CH2, TimGenCh, _TIM9, 1);
channel!(TIM10_CH1, Tim10Ch1, TIM10, Tim10, _TIM10_CH1, TimGenCh, _TIM10, 0);
channel!(TIM11_CH1, Tim11Ch1, TIM11, Tim11, _TIM11_CH1, TimGenCh, _TIM11, 0);
channel!(TIM12_CH1, Tim12Ch1, TIM12, Tim12, _TIM12_CH1, TimGenCh, _TIM12, 0);
channel!(TIM12_CH2, Tim12Ch2, TIM12, Tim12, _TIM12_CH2, TimGenCh, _TIM12, 1);
channel!(TIM13_CH1, Tim13Ch1, TIM13, Tim13, _TIM13_CH1, TimGenCh, _TIM13, 0);
channel!(TIM14_CH1, Tim14Ch1, TIM14, Tim14, _TIM14_CH1, TimGenCh, _TIM14, 0);
