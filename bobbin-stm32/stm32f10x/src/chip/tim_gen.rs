#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_gen::*;

periph!( TIM2, Tim2, _TIM2, TimGenPeriph, 0x40000000);
periph!( TIM3, Tim3, _TIM3, TimGenPeriph, 0x40000400);
periph!( TIM4, Tim4, _TIM4, TimGenPeriph, 0x40000800);

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

