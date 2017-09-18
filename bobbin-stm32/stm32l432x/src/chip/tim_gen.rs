#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_gen::*;

periph!( TIM2, Tim2, _TIM2, TimGenPeriph, 0x40000000);
periph!( TIM3, Tim3, _TIM3, TimGenPeriph, 0x40000400);
periph!( TIM4, Tim4, _TIM4, TimGenPeriph, 0x40000800);
periph!( TIM5, Tim5, _TIM5, TimGenPeriph, 0x40000c00);
periph!( TIM15, Tim15, _TIM15, TimGenPeriph, 0x40014000);
periph!( TIM16, Tim16, _TIM16, TimGenPeriph, 0x40014400);

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

impl super::sig::Signal<super::sig::Tim15Ch1> for Tim15Ch1 {}
impl super::sig::SignalTim<super::sig::Tim15Ch1> for Tim15Ch1 {}
impl super::sig::Signal<super::sig::Tim15Ch2> for Tim15Ch2 {}
impl super::sig::SignalTim<super::sig::Tim15Ch2> for Tim15Ch2 {}

impl super::sig::Signal<super::sig::Tim16Ch1> for Tim16Ch1 {}
impl super::sig::SignalTim<super::sig::Tim16Ch1> for Tim16Ch1 {}
impl super::sig::Signal<super::sig::Tim16Ch2> for Tim16Ch2 {}
impl super::sig::SignalTim<super::sig::Tim16Ch2> for Tim16Ch2 {}


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
channel!(TIM15_CH1, Tim15Ch1, TIM15, Tim15, _TIM15_CH1, TimGenCh, _TIM15, 0);
channel!(TIM15_CH2, Tim15Ch2, TIM15, Tim15, _TIM15_CH2, TimGenCh, _TIM15, 1);
channel!(TIM16_CH1, Tim16Ch1, TIM16, Tim16, _TIM16_CH1, TimGenCh, _TIM16, 0);
channel!(TIM16_CH2, Tim16Ch2, TIM16, Tim16, _TIM16_CH2, TimGenCh, _TIM16, 1);

pub trait IrqTim<T> {
    fn irq_tim(&self) -> T;
}

impl IrqTim<super::irq::IrqTim2> for Tim2 {
    fn irq_tim(&self) -> super::irq::IrqTim2 { super::irq::IRQ_TIM2 }
}

impl IrqTim<super::irq::IrqTim3> for Tim3 {
    fn irq_tim(&self) -> super::irq::IrqTim3 { super::irq::IRQ_TIM3 }
}

impl IrqTim<super::irq::IrqTim4> for Tim4 {
    fn irq_tim(&self) -> super::irq::IrqTim4 { super::irq::IRQ_TIM4 }
}

impl IrqTim<super::irq::IrqTim5> for Tim5 {
    fn irq_tim(&self) -> super::irq::IrqTim5 { super::irq::IRQ_TIM5 }
}

impl IrqTim<super::irq::IrqTim15> for Tim15 {
    fn irq_tim(&self) -> super::irq::IrqTim15 { super::irq::IRQ_TIM15 }
}

impl IrqTim<super::irq::IrqTim16> for Tim16 {
    fn irq_tim(&self) -> super::irq::IrqTim16 { super::irq::IRQ_TIM16 }
}

