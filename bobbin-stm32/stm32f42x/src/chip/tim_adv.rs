#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_adv::*;

periph!(TimAdvPeriph, TIM1, Tim1, 0x40010000);
periph!(TimAdvPeriph, TIM8, Tim8, 0x40010400);

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


channel!(TIM1_CH1, Tim1Ch1, TIM1, Tim1, 0);
channel!(TIM1_CH2, Tim1Ch2, TIM1, Tim1, 1);
channel!(TIM1_CH3, Tim1Ch3, TIM1, Tim1, 2);
channel!(TIM1_CH4, Tim1Ch4, TIM1, Tim1, 3);
channel!(TIM8_CH1, Tim8Ch1, TIM8, Tim8, 0);
channel!(TIM8_CH2, Tim8Ch2, TIM8, Tim8, 1);
channel!(TIM8_CH3, Tim8Ch3, TIM8, Tim8, 2);
channel!(TIM8_CH4, Tim8Ch4, TIM8, Tim8, 3);
