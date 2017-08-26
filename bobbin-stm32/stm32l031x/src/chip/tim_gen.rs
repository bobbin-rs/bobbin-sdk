#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_gen::*;

periph!(TimGenPeriph, TIM2, Tim2, 0x40000000);
periph!(TimGenPeriph, TIM21, Tim21, 0x40010800);
periph!(TimGenPeriph, TIM22, Tim22, 0x40011400);

impl super::sig::Signal<super::sig::Tim2Etr> for Tim2 {}
impl super::sig::SignalEtr<super::sig::Tim2Etr> for Tim2 {}
impl super::sig::Signal<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::SignalTim<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::Signal<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::SignalTim<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::Signal<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::SignalTim<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::Signal<super::sig::Tim2Ch4> for Tim2Ch4 {}
impl super::sig::SignalTim<super::sig::Tim2Ch4> for Tim2Ch4 {}

impl super::sig::Signal<super::sig::Tim21Etr> for Tim21 {}
impl super::sig::SignalEtr<super::sig::Tim21Etr> for Tim21 {}
impl super::sig::Signal<super::sig::Tim21Ch1> for Tim21Ch1 {}
impl super::sig::SignalTim<super::sig::Tim21Ch1> for Tim21Ch1 {}
impl super::sig::Signal<super::sig::Tim21Ch2> for Tim21Ch2 {}
impl super::sig::SignalTim<super::sig::Tim21Ch2> for Tim21Ch2 {}

impl super::sig::Signal<super::sig::Tim22Etr> for Tim22 {}
impl super::sig::SignalEtr<super::sig::Tim22Etr> for Tim22 {}
impl super::sig::Signal<super::sig::Tim22Ch1> for Tim22Ch1 {}
impl super::sig::SignalTim<super::sig::Tim22Ch1> for Tim22Ch1 {}
impl super::sig::Signal<super::sig::Tim22Ch2> for Tim22Ch2 {}
impl super::sig::SignalTim<super::sig::Tim22Ch2> for Tim22Ch2 {}


channel!(TIM2_CH1, Tim2Ch1, TIM2, Tim2, 0);
channel!(TIM2_CH2, Tim2Ch2, TIM2, Tim2, 1);
channel!(TIM2_CH3, Tim2Ch3, TIM2, Tim2, 2);
channel!(TIM2_CH4, Tim2Ch4, TIM2, Tim2, 3);
channel!(TIM21_CH1, Tim21Ch1, TIM21, Tim21, 0);
channel!(TIM21_CH2, Tim21Ch2, TIM21, Tim21, 1);
channel!(TIM22_CH1, Tim22Ch1, TIM22, Tim22, 0);
channel!(TIM22_CH2, Tim22Ch2, TIM22, Tim22, 1);
