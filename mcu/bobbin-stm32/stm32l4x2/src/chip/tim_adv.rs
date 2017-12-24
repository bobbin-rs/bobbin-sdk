#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_adv::*;

periph!( TIM1, Tim1, _TIM1, TimAdvPeriph, 0x40012c00);

impl super::sig::Signal<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::SignalTim<super::sig::Tim1Ch1> for Tim1Ch1 {}
impl super::sig::Signal<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::SignalTim<super::sig::Tim1Ch2> for Tim1Ch2 {}
impl super::sig::Signal<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::SignalTim<super::sig::Tim1Ch3> for Tim1Ch3 {}
impl super::sig::Signal<super::sig::Tim1Ch4> for Tim1Ch4 {}
impl super::sig::SignalTim<super::sig::Tim1Ch4> for Tim1Ch4 {}


channel!(TIM1_CH1, Tim1Ch1, TIM1, Tim1, _TIM1_CH1, TimAdvCh, _TIM1, 0);
channel!(TIM1_CH2, Tim1Ch2, TIM1, Tim1, _TIM1_CH2, TimAdvCh, _TIM1, 1);
channel!(TIM1_CH3, Tim1Ch3, TIM1, Tim1, _TIM1_CH3, TimAdvCh, _TIM1, 2);
channel!(TIM1_CH4, Tim1Ch4, TIM1, Tim1, _TIM1_CH4, TimAdvCh, _TIM1, 3);

pub trait IrqTimBrk<T> {
    fn irq_tim_brk(&self) -> T;
}

pub trait IrqTimUp<T> {
    fn irq_tim_up(&self) -> T;
}

pub trait IrqTimTrgCom<T> {
    fn irq_tim_trg_com(&self) -> T;
}

pub trait IrqTimCc<T> {
    fn irq_tim_cc(&self) -> T;
}

impl IrqTimBrk<super::irq::IrqTim1Brk> for Tim1 {
    fn irq_tim_brk(&self) -> super::irq::IrqTim1Brk { super::irq::IRQ_TIM1_BRK }
}

impl IrqTimUp<super::irq::IrqTim1Up> for Tim1 {
    fn irq_tim_up(&self) -> super::irq::IrqTim1Up { super::irq::IRQ_TIM1_UP }
}

impl IrqTimTrgCom<super::irq::IrqTim1TrgCom> for Tim1 {
    fn irq_tim_trg_com(&self) -> super::irq::IrqTim1TrgCom { super::irq::IRQ_TIM1_TRG_COM }
}

impl IrqTimCc<super::irq::IrqTim1Cc> for Tim1 {
    fn irq_tim_cc(&self) -> super::irq::IrqTim1Cc { super::irq::IRQ_TIM1_CC }
}

