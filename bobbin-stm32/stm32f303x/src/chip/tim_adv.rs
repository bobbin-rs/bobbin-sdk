#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_adv::*;

periph!( TIM1, Tim1, _TIM1, TimAdvPeriph, 0x40012c00);
periph!( TIM8, Tim8, _TIM8, TimAdvPeriph, 0x40013400);
periph!( TIM20, Tim20, _TIM20, TimAdvPeriph, 0x40015000);

impl super::sig::Signal<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::SignalTim<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::Signal<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::SignalTim<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::Signal<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::SignalTim<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::Signal<super::sig::Tim1Ch4> for Tim1Ch4 {}
impl super::sig::SignalTim<super::sig::Tim1Ch4> for Tim1Ch4 {}

impl super::sig::Signal<super::sig::Tim8Ch1> for Tim8Ch1 {}
impl super::sig::SignalTim<super::sig::Tim8Ch1> for Tim8Ch1 {}
impl super::sig::Signal<super::sig::Tim8Ch2> for Tim8Ch2 {}
impl super::sig::SignalTim<super::sig::Tim8Ch2> for Tim8Ch2 {}
impl super::sig::Signal<super::sig::Tim8Ch3> for Tim8Ch3 {}
impl super::sig::SignalTim<super::sig::Tim8Ch3> for Tim8Ch3 {}
impl super::sig::Signal<super::sig::Tim8Ch4> for Tim8Ch4 {}
impl super::sig::SignalTim<super::sig::Tim8Ch4> for Tim8Ch4 {}

impl super::sig::Signal<super::sig::Tim20Ch1> for Tim20Ch1 {}
impl super::sig::SignalTim<super::sig::Tim20Ch1> for Tim20Ch1 {}
impl super::sig::Signal<super::sig::Tim20Ch2> for Tim20Ch2 {}
impl super::sig::SignalTim<super::sig::Tim20Ch2> for Tim20Ch2 {}
impl super::sig::Signal<super::sig::Tim20Ch3> for Tim20Ch3 {}
impl super::sig::SignalTim<super::sig::Tim20Ch3> for Tim20Ch3 {}
impl super::sig::Signal<super::sig::Tim20Ch4> for Tim20Ch4 {}
impl super::sig::SignalTim<super::sig::Tim20Ch4> for Tim20Ch4 {}


channel!(TIM1_CH1, Tim1Ch1, TIM1, Tim1, _TIM1_CH1, TimAdvCh, _TIM1, 0);
channel!(TIM1_CH2, Tim1Ch2, TIM1, Tim1, _TIM1_CH2, TimAdvCh, _TIM1, 1);
channel!(TIM1_CH3, Tim1Ch3, TIM1, Tim1, _TIM1_CH3, TimAdvCh, _TIM1, 2);
channel!(TIM1_CH4, Tim1Ch4, TIM1, Tim1, _TIM1_CH4, TimAdvCh, _TIM1, 3);
channel!(TIM8_CH1, Tim8Ch1, TIM8, Tim8, _TIM8_CH1, TimAdvCh, _TIM8, 0);
channel!(TIM8_CH2, Tim8Ch2, TIM8, Tim8, _TIM8_CH2, TimAdvCh, _TIM8, 1);
channel!(TIM8_CH3, Tim8Ch3, TIM8, Tim8, _TIM8_CH3, TimAdvCh, _TIM8, 2);
channel!(TIM8_CH4, Tim8Ch4, TIM8, Tim8, _TIM8_CH4, TimAdvCh, _TIM8, 3);
channel!(TIM20_CH1, Tim20Ch1, TIM20, Tim20, _TIM20_CH1, TimAdvCh, _TIM20, 0);
channel!(TIM20_CH2, Tim20Ch2, TIM20, Tim20, _TIM20_CH2, TimAdvCh, _TIM20, 1);
channel!(TIM20_CH3, Tim20Ch3, TIM20, Tim20, _TIM20_CH3, TimAdvCh, _TIM20, 2);
channel!(TIM20_CH4, Tim20Ch4, TIM20, Tim20, _TIM20_CH4, TimAdvCh, _TIM20, 3);
