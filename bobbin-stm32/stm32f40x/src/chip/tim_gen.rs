#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_gen::*;

periph!(TimGenPeriph, TIM2, Tim2, 0x40000000);
periph!(TimGenPeriph, TIM3, Tim3, 0x40000400);
periph!(TimGenPeriph, TIM4, Tim4, 0x40000800);
periph!(TimGenPeriph, TIM5, Tim5, 0x40000c00);
periph!(TimGenPeriph, TIM9, Tim9, 0x40014000);
periph!(TimGenPeriph, TIM10, Tim10, 0x40014400);
periph!(TimGenPeriph, TIM11, Tim11, 0x40014800);
periph!(TimGenPeriph, TIM12, Tim12, 0x40001800);
periph!(TimGenPeriph, TIM13, Tim13, 0x40001c00);
periph!(TimGenPeriph, TIM14, Tim14, 0x40002000);

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


channel!(TIM2_CH1, Tim2Ch1, TIM2, Tim2, 0);
channel!(TIM2_CH2, Tim2Ch2, TIM2, Tim2, 1);
channel!(TIM2_CH3, Tim2Ch3, TIM2, Tim2, 2);
channel!(TIM2_CH4, Tim2Ch4, TIM2, Tim2, 3);
channel!(TIM3_CH1, Tim3Ch1, TIM3, Tim3, 0);
channel!(TIM3_CH2, Tim3Ch2, TIM3, Tim3, 1);
channel!(TIM3_CH3, Tim3Ch3, TIM3, Tim3, 2);
channel!(TIM3_CH4, Tim3Ch4, TIM3, Tim3, 3);
channel!(TIM4_CH1, Tim4Ch1, TIM4, Tim4, 0);
channel!(TIM4_CH2, Tim4Ch2, TIM4, Tim4, 1);
channel!(TIM4_CH3, Tim4Ch3, TIM4, Tim4, 2);
channel!(TIM4_CH4, Tim4Ch4, TIM4, Tim4, 3);
channel!(TIM5_CH1, Tim5Ch1, TIM5, Tim5, 0);
channel!(TIM5_CH2, Tim5Ch2, TIM5, Tim5, 1);
channel!(TIM5_CH3, Tim5Ch3, TIM5, Tim5, 2);
channel!(TIM5_CH4, Tim5Ch4, TIM5, Tim5, 3);
channel!(TIM9_CH1, Tim9Ch1, TIM9, Tim9, 0);
channel!(TIM9_CH2, Tim9Ch2, TIM9, Tim9, 1);
channel!(TIM10_CH1, Tim10Ch1, TIM10, Tim10, 0);
channel!(TIM11_CH1, Tim11Ch1, TIM11, Tim11, 0);
channel!(TIM12_CH1, Tim12Ch1, TIM12, Tim12, 0);
channel!(TIM12_CH2, Tim12Ch2, TIM12, Tim12, 1);
channel!(TIM13_CH1, Tim13Ch1, TIM13, Tim13, 0);
channel!(TIM14_CH1, Tim14Ch1, TIM14, Tim14, 0);
