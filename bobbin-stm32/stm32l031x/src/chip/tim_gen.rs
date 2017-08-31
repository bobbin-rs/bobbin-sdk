#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_gen::*;

periph!( TIM2, Tim2, _TIM2, TimGenPeriph, 0x40000000);
periph!( TIM21, Tim21, _TIM21, TimGenPeriph, 0x40010800);
periph!( TIM22, Tim22, _TIM22, TimGenPeriph, 0x40011400);

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


channel!(TIM2_CH1, Tim2Ch1, TIM2, Tim2, _TIM2_CH1, TimGenCh, _TIM2, 0);
channel!(TIM2_CH2, Tim2Ch2, TIM2, Tim2, _TIM2_CH2, TimGenCh, _TIM2, 1);
channel!(TIM2_CH3, Tim2Ch3, TIM2, Tim2, _TIM2_CH3, TimGenCh, _TIM2, 2);
channel!(TIM2_CH4, Tim2Ch4, TIM2, Tim2, _TIM2_CH4, TimGenCh, _TIM2, 3);
channel!(TIM21_CH1, Tim21Ch1, TIM21, Tim21, _TIM21_CH1, TimGenCh, _TIM21, 0);
channel!(TIM21_CH2, Tim21Ch2, TIM21, Tim21, _TIM21_CH2, TimGenCh, _TIM21, 1);
channel!(TIM22_CH1, Tim22Ch1, TIM22, Tim22, _TIM22_CH1, TimGenCh, _TIM22, 0);
channel!(TIM22_CH2, Tim22Ch2, TIM22, Tim22, _TIM22_CH2, TimGenCh, _TIM22, 1);

pub trait IrqTim<T> {
   fn irq_tim(&self) -> T;
}

impl IrqTim<super::irq::IrqTim2> for Tim2 {
   fn irq_tim(&self) -> super::irq::IrqTim2 { super::irq::IRQ_TIM2 }
}

impl IrqTim<super::irq::IrqTim21> for Tim21 {
   fn irq_tim(&self) -> super::irq::IrqTim21 { super::irq::IRQ_TIM21 }
}

impl IrqTim<super::irq::IrqTim22> for Tim22 {
   fn irq_tim(&self) -> super::irq::IrqTim22 { super::irq::IRQ_TIM22 }
}

